# Proposal: UI and feature candidates from a survey of an established piping-stress tool (2026-10-26)

**Status:** draft proposal for the owner. Nothing here changes a commitment,
ScopeOfWork or acceptance criterion until the owner approves a bundle or an
item. Acceptance, issuance and release remain the owner's acts (F-PIP-3).

**Basis.** A hands-on survey of an established piping-analysis tool, used only
with our own invented pilot models, together with its publicly available
documentation. Details stay in private survey notes. Each candidate describes
what a piping-analysis user can do and why it matters for SWBPIPE, in our own
words. This document reproduces no screen layouts, icons, labels, help text,
tables, figures or examples. It carries no third-party library content
(materials, valves, hanger catalogues, spectra, fatigue curves) and no result
values from the surveyed tool. It makes no claim about any other tool's quality
or about compatibility with it. Where a finding involves code-specific content
(SIFs, allowables, load combinations, code formulas), it becomes a rule-pack
*capability* proposal: the product would gain the means to evaluate what a
user's private pack supplies, and would ship no code data.

**SWBPIPE references** are to `projects/chirality-piping/docs/PRD.md`,
`docs/COMPLETENESS_MAP.md` (basis `8a8a8b2584` plus the rulings it folds in),
`docs/user_guide/index.md` and the deliverable localities under `execution/`.
Statements that SWBPIPE lacks something come from that map, the user guide or
a keyword survey of `apps/desktop/src` and `core/`. They are marked "(code
survey)" where they rest only on the keyword survey and should be confirmed
before a ScopeOfWork is edited.

**Fields.** *Workflow*: what the user does. *Why*: value for SWBPIPE.
*Touches*: PRD requirement, deliverable, or the gap it fills. *Size*: S (days),
M (one to two weeks), L (several weeks or a new deliverable), with a one-line
basis. *Rec.*: recommendation. *Pri.*: P1 (next work plan), P2 (after P1), P3
(backlog or later release). *Basis*: observed in use, documented publicly, or
both.

## Approvable bundles

| Bundle | Theme | Items | P1 | P2 | P3 |
|---|---|---:|---:|---:|---:|
| A | Results review | 9 | 5 | 4 | 0 |
| B | Fast authoring and bulk edits | 11 | 2 | 7 | 2 |
| C | Load conditions and support design | 8 | 2 | 3 | 3 |
| D | Reports, records and the solve experience | 7 | 2 | 4 | 1 |
| E | Rule-pack capability extensions | 6 | 0 | 2 | 4 |
| F | Dynamics and specialty mechanics roadmap | 5 | 0 | 1 | 4 |
| | **Total** | **46** | **11** | **21** | **14** |

The owner can approve a bundle whole, approve its P1 items only, or pick items.
Approving a bundle authorises ScopeOfWork edits for the named deliverables and
a work-plan slice. It does not approve any acceptance criterion change; those
edits come back for review.

---

## Bundle A: Results review

The workflow idea most worth adopting here is that a stress engineer iterates
many times on one model. After each solve they want a few key views at once and can
then step through detail tables without leaving one pane. SWBPIPE has rich
result rows and a viewport overlay, but its review surface is organised around
filters on one result set (FR-GUI-001 notes: "Colour is one value per member").

**A1. Results navigator with keyboard stepping.**
- *Workflow:* one results pane shows one result table at a time: displacements,
  element forces, reactions, stresses, user-rule ratios and summaries. Keys or
  buttons step to the next or previous table kind, load case or combination,
  and support or element. The viewport follows the current row.
- *Why:* review speed on real models with many cases. Reading three things in
  turn (one support, all cases) becomes a single key press.
- *Touches:* FR-GUI-001, FR-GUI-002 (runs in the tree); DEL-07-05 (results
  viewer), DEL-00-05 (GUI state).
- *Size:* M. The pane exists; this adds a navigation model over existing
  envelopes and selection sync.
- *Rec.:* adopt. *Pri.:* P1. *Basis:* observed and documented.

**A2. Per-support load summary across all cases.**
- *Workflow:* pick a support (or equipment connection). A table lists its
  forces, moments, resultants and displacements for every solved case and
  combination. Extreme-value rows (maximum and minimum by component, with the
  governing case) follow, plus an optional row of user-supplied allowable loads
  from the support record or the user's rule pack. Step to the next support.
- *Why:* support and equipment-load hand-offs are the main deliverable of a
  stress run to the structural and mechanical disciplines. No cross-case
  per-support summary was found in the desktop (code survey).
- *Touches:* FR-SOL-007, PRD §12.4; DEL-07-05, DEL-08-04 (result export),
  DEL-05-02 (envelopes).
- *Size:* M. Pure post-processing of existing reaction rows plus one table and
  an export.
- *Rec.:* adopt. The allowable row appears only when the user supplies it; the
  product supplies none. *Pri.:* P1. *Basis:* observed.

**A3. Sorted stress and ratio table with thresholds.**
- *Workflow:* per stress category, list result locations sorted by user-rule
  ratio (or by stress magnitude when no pack is loaded). The user sets a
  threshold to show only locations above it, and toggles stress or ratio
  display. Clicking a row highlights the location.
- *Why:* the first question after a solve is "where is it worst?". This answers
  it without scanning per-element tables.
- *Touches:* FR-SOL-008, FR-RULE-003 (incomplete checks stay visible as
  incomplete, never sorted as passing); DEL-07-05, DEL-06-03.
- *Size:* S–M. Sorting and filtering exist in parts (code survey). The work is
  per-category grouping and the incomplete-input rendering.
- *Rec.:* adopt. *Pri.:* P1. *Basis:* observed.

**A4. Continuous colour contour with legend and threshold.**
- *Workflow:* colour the rendered pipe by a chosen quantity (a stress category,
  a user-rule ratio, or a displacement), varying along members by result
  station, with a numeric legend. An option shows only values above a
  threshold, or shows numbers in place of colours.
- *Why:* closes the FR-GUI-001 note that colour is one value per member. It is
  also the most effective single picture for an owner reviewer.
- *Touches:* FR-GUI-001, PRD §14.2 ("stress/result overlays"); DEL-07-01,
  DEL-07-05.
- *Size:* M. Per-station interpolation on the existing overlay and a legend
  component; the D-72 orbit qualification applies.
- *Rec.:* adopt. *Pri.:* P1. *Basis:* observed and documented.

**A5. Deflected-shape controls.**
- *Workflow:* set the displacement magnification factor, keep the undeformed
  shape as a faint ghost, and animate between undeformed and deformed. Pick
  the case from the navigator (A1).
- *Why:* engineers judge thermal flexibility by eye. A fixed normalised scale
  hides small but important movements near equipment.
- *Touches:* FR-GUI-001; DEL-07-01 (`deformationOverlay.ts`), DEL-07-05.
- *Size:* S. Factor, ghost and animation over the existing overlay.
- *Rec.:* adopt. Label the factor on screen, since the display is not
  physical-scale (DEL-07-05 CLM-006). *Pri.:* P2. *Basis:* observed and documented.

**A6. Key-results review panel after each solve.**
- *Workflow:* one panel, opened automatically or on demand after a solve,
  offers one-click views of:
  - ratio contours for each stress category the user's pack defines;
  - deflected shapes for the main cases;
  - the support summary (A2) for anchors and equipment connections;
  - the resting-support contact or lift-off status per case;
  - the hanger table (C4);
  - frequencies, once F1 exists.

  From there the user jumps to detailed tables or back to editing.
- *Why:* this is the iteration loop the owner's users live in. It also shows
  SWBPIPE's boundary: categories and ratios appear only from the user's pack,
  and an incomplete pack shows as incomplete.
- *Touches:* PRD §10.2–10.3 journeys, FR-GUI-001/006; DEL-07-05, DEL-07-01,
  DEL-04-04 (support active state).
- *Size:* M. It composes A2, A4 and A5 and the existing nonlinear status.
- *Rec.:* adopt after A2/A4. *Pri.:* P1. *Basis:* observed and documented.

**A7. Element local-axis display and local/global toggle.**
- *Workflow:* when reading element forces, show a small local-axis triad on the
  current element in the viewport, and toggle the table between local and
  global frames.
- *Why:* most misread force tables come from axis conventions. SWBPIPE already
  publishes global end rows (W-7); this makes both frames legible.
- *Touches:* FR-SOL-007; DEL-07-05, DEL-07-01, DEL-04-01.
- *Size:* S. Both frames exist as rows; this adds a triad glyph and a toggle.
- *Rec.:* adopt. *Pri.:* P2. *Basis:* observed and documented.

**A8. Displacement extremes table.**
- *Workflow:* for each node, the largest and smallest displacement and
  rotation components across chosen cases, with the governing case. Filter to
  supports of a kind (guides, stops, hangers).
- *Why:* clearance checks and support travel reviews.
- *Touches:* FR-SOL-007, FR-SOL-009; DEL-07-05, DEL-05-02.
- *Size:* S. An envelope over existing rows.
- *Rec.:* adopt. *Pri.:* P2. *Basis:* observed and documented.

**A9. Weight and centre-of-gravity summary.**
- *Workflow:* totals of empty pipe, insulation, contents, lining, components and
  added weights, with the overall centre of gravity. Optionally by line or
  selection.
- *Why:* a fast sanity check on model mass. Lifting and structural teams ask
  for it.
- *Touches:* FR-SOL-003 (weights), PRD §12.5; DEL-03-08 (mass properties),
  DEL-07-05, DEL-08-01.
- *Size:* S. The mass properties already exist per element.
- *Rec.:* adopt. *Pri.:* P2. *Basis:* observed.

## Bundle B: Fast authoring and bulk edits

A common way to author a piping model is a run sheet: one row per element,
entered as increments from the previous node. Rows inherit material, section and
operating conditions from the row above, and bulk commands act on row ranges.
SWBPIPE is viewport-first with a typed entity grid (Model tree "Grid" mode) and
validated, undoable operations. Each item below should emit ordinary operation
batches (DEL-16-06), so diff preview and undo apply unchanged.

**B1. Run-sheet entry mode in the grid.**
- *Workflow:* in a "run" grid, each row creates the next element and its end
  node from three increments. Blank cells inherit from the row above. A
  single-key code picks the element kind or the support at the end node. A
  "start from" row begins a branch at any existing node. The viewport grows as
  rows are typed.
- *Why:* experienced users model from isometrics far faster by typing than by
  placing. This is the main productivity gap a user trained on run-sheet
  entry will feel in SWBPIPE.
- *Touches:* FR-GUI-004, PRD §14.1; DEL-07-02 (grid), DEL-07-09 (operation
  vocabulary), DEL-16-06.
- *Size:* L. Grid semantics, inheritance rules, mapping each row to operation
  batches, and keyboard flow.
- *Rec.:* adopt as a design-first slice: write the inheritance and row-to-
  operation rules in DEL-07-02 Design before code. *Pri.:* P1.
  *Basis:* observed and documented.

**B2. Range bulk change and replace.**
- *Workflow:* over a selection or a run range:
  - replace one element kind with another;
  - replace one support kind with another, or remove a support kind;
  - reassign material, section or operating-condition set (C1);
  - set a tag.

  The preview shows the count of affected entities.
- *Why:* "what-if" studies, such as changing all guides to stops or all
  elbows to another radius class, are daily work.
- *Touches:* FR-GUI-004, FR-MOD-004; DEL-16-06, DEL-07-02, DEL-07-09.
- *Size:* M. Selection-scoped operation generation with family-aware
  validation.
- *Rec.:* adopt. *Pri.:* P1. *Basis:* observed and documented.

**B3. Pattern generate.**
- *Workflow:* copy a selected set of elements N times with a node-label
  increment and a fixed XYZ offset per copy. The copy refuses duplicates
  unless the user overrides.
- *Why:* pipe racks, headers with repeated branches, and parallel lines.
- *Touches:* FR-GUI-004; DEL-16-06, DEL-07-01 (geometry tools have copy).
- *Size:* M. It extends the existing copy operation with repetition and
  increments.
- *Rec.:* adopt. *Pri.:* P2. *Basis:* observed and documented.

**B4. Reverse a run.**
- *Workflow:* re-author a selected run so that it starts from its far end, with
  new node labels, preserving components and supports.
- *Why:* merging models authored from different ends, and matching the direction of
  a supplied isometric.
- *Touches:* FR-GUI-004; DEL-16-06.
- *Size:* S–M.
- *Rec.:* defer until B1 lands. *Pri.:* P3. *Basis:* documented.

**B5. Multiple split and slope entry.**
- *Workflow:* split an element into N equal parts with labelled intermediate
  nodes. Define an element by length plus a direction vector, or by an angle or
  a "1 in n" slope.
- *Why:* support placement, and sloped lines (drains, gravity lines).
- *Touches:* FR-GUI-004; DEL-07-01 (split exists), DEL-16-06.
- *Size:* S.
- *Rec.:* adopt. *Pri.:* P2. *Basis:* observed.

**B6. Node renumbering and a node increment preference.**
- *Workflow:* relabel nodes over a range from a start value and step, and set
  the default label increment used by every creating tool.
- *Why:* labelling conventions matter for drawings and reports. SWBPIPE ids
  stay stable (FR-MOD-003); only labels change.
- *Touches:* FR-MOD-003/004; DEL-02-01, DEL-16-06.
- *Size:* S.
- *Rec.:* adopt, labels only. *Pri.:* P2. *Basis:* observed.

**B7. Merge another model.**
- *Workflow:* bring a second SWBPIPE model into the current one, joining at a
  named node. Name collisions in materials, sections and libraries are
  resolved by preview, with optional node relabelling.
- *Why:* teams split large systems by area and join them for the final run.
- *Touches:* FR-MOD-001/003, FR-CMP-003 (state diff shows the merge);
  DEL-02-05, DEL-16-06, DEL-14-03.
- *Size:* M. Id collision policy and a merge preview.
- *Rec.:* adopt. *Pri.:* P2. *Basis:* observed and documented.

**B8. Edit from the viewport, with selection synchronized everywhere.**
- *Workflow:* clicking a component or support in the viewport opens its
  editor. The grid, tree and results pane move to the same entity. While
  results are shown, the editor is read-only.
- *Why:* modelling becomes direct. Results review links back to inputs.
- *Touches:* FR-GUI-003/004; DEL-07-01, DEL-07-02, DEL-00-05.
- *Size:* S–M. Selection state exists; this adds editor routing and a
  read-only mode.
- *Rec.:* adopt; confirm what already exists first. *Pri.:* P2.
  *Basis:* documented.

**B9. Context menus on entities and grid cells.**
- *Workflow:* right-click a support, component or grid column to get the
  relevant operations: delete this support, list all of this kind, split
  here, set slope, insert before.
- *Why:* discoverability, and fewer trips to the toolbar.
- *Touches:* DEL-07-09 (operation vocabulary), DEL-07-06 (accessibility:
  keyboard equivalents required).
- *Size:* S.
- *Rec.:* adopt. *Pri.:* P2. *Basis:* documented.

**B10. Annotation and display filters.**
- *Workflow:* choose which annotation categories the viewport shows (supports
  by kind, loads, lengths, node labels only at chosen support kinds). Change
  symbol size, make the selection transparent, and freeze the view while
  editing.
- *Why:* legibility on large models.
- *Touches:* FR-GUI-001; DEL-07-01.
- *Size:* S.
- *Rec.:* adopt when convenient. *Pri.:* P3. *Basis:* observed.

**B11. On-demand model checks.**
- *Workflow:* commands that check:
  - bend fit (the bend radius fits between adjacent nodes);
  - near-miss connections in loops (a node that looks on a run but is not
    connected);
  - tees lacking the branch modifiers that the user's pack requires;
  - references to missing nodes.

  Each finding links to the entity.
- *Why:* catches the modelling errors that solve-time topology warnings do not
  name.
- *Touches:* FR-KNOW-002, FR-GUI-006; DEL-04-06, DEL-07-04 (W-8 warnings are
  the base).
- *Size:* S–M. Extends W-8.
- *Rec.:* adopt. *Pri.:* P2. *Basis:* observed and documented.

## Bundle C: Load conditions and support design

**C1. Operating-condition sets assigned along runs.**
- *Workflow:* define named condition sets. Each set holds several operating
  temperatures and pressures, peak and design values, contents density, added
  weight per length, and exposure flags (wind, snow/ice, wave). Assign a set to
  elements; it inherits down the run like material and section. Primitive load
  cases then refer to "condition k of whatever set each element carries".
- *Why:* real systems have different temperatures per line segment and several
  operating modes. Authoring per-case, per-element values does not scale.
- *Touches:* PRD §12.5–12.6, FR-SOL-006; DEL-05-01, DEL-02-01 (model document
  slot), DEL-07-02.
- *Size:* M–L. A model-document version step, a case-resolution rule and an
  authoring UI.
- *Rec.:* adopt; it needs a model-document version (coordinate with the 0.8.0
  step). *Pri.:* P1. *Basis:* observed and documented.

**C2. Case recipes declared by the user's rule pack.**
- *Workflow:* the user's pack declares the case recipes its checks need, for
  example:
  - weight plus pressure;
  - operating per condition;
  - a range between two named states;
  - empty weight;
  - occasional with a combination method.

  The product proposes the matching primitive cases and combinations as one
  reviewable operation batch. The user ticks which to run.
- *Why:* today the user builds the cases by hand and maps them to the pack
  (format 0.5.0). Recipes keep code-specific combinations in the private pack
  while removing the manual work.
- *Touches:* §12.6 ("shall not ship protected code-specific load
  combinations"), FR-RULE-001; DEL-06-01, DEL-05-02, DEL-16-06.
- *Size:* M. A pack schema addition, a generator and a preview.
- *Rec.:* adopt as a rule-pack capability. Only the invented demo pack ships a
  recipe. *Pri.:* P1. *Basis:* observed and documented.

**C3. Sustained case with the operating support status.**
- *Workflow:* an option re-solves a weight-and-pressure case with the support
  contact state found in a chosen operating case. Supports that lift off in
  operation carry nothing. The rule pack can require this case.
- *Why:* with lift-off, the sustained stresses the engineer must evaluate are
  those in the redistributed state. A plain sustained solve understates them.
- *Touches:* FR-SOL-014, §12.6; DEL-04-04, DEL-04-07, DEL-06-01.
- *Size:* M. It reuses the active-set result as a fixed support state.
- *Rec.:* adopt on the pressure-free route. *Pri.:* P2. *Basis:* observed and documented.

**C4. Hanger sizing workflow with the user's catalogue.**
- *Workflow:* mark supports as "to be sized" and follow these steps:
  1. The product solves weight with those points held, to get the hot load.
  2. It solves the operating case with them released, to get the travel.
  3. It picks a spring size from the user's private hanger catalogue that
     meets the user's load-variation limit.
  4. It writes the selection back as operations and lists a hanger table
     (cold and hot load, travel, size, rate).
- *Why:* a core stress-engineering task. SWBPIPE already imports hanger
  records and applies one to a support (`features/hanger-selection`).
- *Touches:* PRD §12.4 (variable spring, constant effort), FR-RULE-006;
  DEL-03-01/DEL-03-07 (hanger library), DEL-04-07, DEL-07-05.
- *Size:* L. A multi-solve procedure, a selection rule and a report table.
- *Rec.:* adopt; ship no manufacturer catalogue, only invented example records.
  *Pri.:* P2. *Basis:* observed and documented.

**C5. Cold spring element and its own cases.**
- *Workflow:* mark an element as cut short or long by a length. The product
  adds installed-state cases that feed support and equipment loads but not
  stress ranges.
- *Why:* equipment-load reduction on hot lines.
- *Touches:* §12.5 (imposed movements); DEL-05-01, DEL-04-07.
- *Size:* M.
- *Rec.:* defer. *Pri.:* P3. *Basis:* documented.

**C6. Occasional-load generators driven by user parameters.**
- *Workflow:* wind as a pressure or velocity profile over elevation with a
  shape factor and direction. Static seismic as g-levels per axis with a
  chosen directional combination (square-root-sum-of-squares, absolute or
  algebraic). Snow and ice as added weight. Each element carries exposure
  flags from its condition set (C1). The user supplies code-derived
  coefficients through a pack or the dialog; the product ships none.
- *Why:* SWBPIPE has wind and seismic on the pressure-free route. Elevation
  profiles and per-element exposure make them usable on real plants.
- *Touches:* PRD §12.5; DEL-05-01, DEL-05-05, `features/wind-exposure`.
- *Size:* M.
- *Rec.:* adopt as extensions; a "load generator pack" is a rule-pack-style
  capability. *Pri.:* P2. *Basis:* observed and documented.

**C7. Hydrotest condition.**
- *Workflow:* a flagged condition with test pressure, test contents density
  and optional temporary locking of spring supports. The resulting case is
  marked as a test case in results and reports.
- *Touches:* PRD §12.5 (hydrotest load); DEL-05-01.
- *Size:* S–M.
- *Rec.:* adopt after C1. *Pri.:* P3. *Basis:* observed and documented.

**C8. Structural steel members in the pipe model.**
- *Workflow:* add beam members with user sections and end releases, to model
  support frames and their flexibility together with the pipe.
- *Why:* support stiffness changes loads near equipment. The frame kernel can
  carry this directly.
- *Touches:* FR-SOL-002/004; DEL-04-01, DEL-03-02.
- *Size:* M.
- *Rec.:* defer. *Pri.:* P3. *Basis:* observed and documented.

## Bundle D: Reports, records and the solve experience

**D1. Report composer.**
- *Workflow:* pick the report's sections:
  - inputs: settings, run sheet, coordinates, materials, sections, condition
    sets, loads;
  - results per selected case: tables, summaries (A2, A3, A8, A9), plots
    (A4, A5);
  - bill of materials (D3) and a table of contents.

  Preview the result, then save or print.
- *Why:* FR-REP-001 is partial. Users expect to choose content per audience.
- *Touches:* FR-REP-001..004; DEL-08-01, DEL-08-03.
- *Size:* M. Section selection over the existing renderer; the local-private
  gating stays.
- *Rec.:* adopt. *Pri.:* P1. *Basis:* observed.

**D2. Table export to CSV and text.**
- *Workflow:* export any results or input table shown (A1–A9, grid) to CSV or
  plain text, behind the existing local-private intent.
- *Why:* engineers post-process in spreadsheets. It is also the feed for the
  correlation harness.
- *Touches:* FR-CMP-010, §19; DEL-08-04, DEL-12-02.
- *Size:* S.
- *Rec.:* adopt. *Pri.:* P1. *Basis:* observed and documented.

**D3. Bill of materials.**
- *Workflow:* lengths by section and material, counts of components by kind,
  and weights.
- *Touches:* §19; DEL-08-01, DEL-03-08.
- *Size:* S.
- *Rec.:* adopt. *Pri.:* P2. *Basis:* observed.

**D4. Document-control fields and a revision log.**
- *Workflow:* the user types client, project, file and report numbers, and the
  names of the preparer and checker. A dated revision log lives with the
  model, and reports print both.
- *Why:* every issued calculation needs a title block and a history.
- *Touches:* FR-MOD-004, §19.2; DEL-02-01, DEL-08-01.
- *Boundary note:* these are free text the user enters. The product never
  fills, infers or validates them, and they carry no acceptance meaning
  (F-PIP-3, §21.3).
- *Size:* S.
- *Rec.:* adopt with the boundary note in the field help. *Pri.:* P2.
  *Basis:* observed.

**D5. Autosave and backup copies.**
- *Workflow:* the user can turn on a timed autosave of the open project and
  keep a backup of the last saved version in a chosen local folder.
- *Why:* protects against crash loss. Projects live in one local store.
- *Touches:* §20.4; DEL-12-01, DEL-02-05.
- *Size:* S–M.
- *Rec.:* adopt. *Pri.:* P2. *Basis:* observed.

**D6. Solve progress panel.**
- *Workflow:* during a solve, show the equation count, matrix size and fill,
  elapsed time and nonlinear iteration count, with a cancel button. At the
  end, offer to open the review panel (A6).
- *Why:* trust and control on large models. These statistics also feed
  performance tracking.
- *Touches:* FR-SOL-015; DEL-07-07, DEL-04-05.
- *Size:* S.
- *Rec.:* adopt. *Pri.:* P2. *Basis:* observed.

**D7. View export as an image or drawing.**
- *Workflow:* save the current view as an image or a vector drawing with a
  title block and page size.
- *Touches:* §19; DEL-08-01, DEL-17-08.
- *Size:* S.
- *Rec.:* defer. *Pri.:* P3. *Basis:* observed.

## Bundle E: Rule-pack capability extensions

Each item adds a capability to the rule-pack format and evaluator, so that a
user's private pack can express a check family that users expect to run. No
formulas, limits or curves ship. The invented demo pack gets an invented
example of each.

**E1. Entity-scoped checks.**
- *Workflow:* a check bound to supports, equipment connection points or flange
  entities. It reads per-entity user inputs (for example, nozzle load limits
  supplied with the equipment) and aggregates across the mapped cases. The
  output is a per-entity table: governing case, utilisation, and an incomplete
  status where inputs are missing.
- *Why:* equipment nozzle qualification and support capacity checks are
  routine and code- or vendor-specific. Today's checks run per result location.
- *Touches:* FR-RULE-001/003; DEL-06-01, DEL-06-03, DEL-07-03.
- *Size:* M.
- *Rec.:* adopt. *Pri.:* P2. *Basis:* observed and documented.

**E2. Interpolated tables in the evaluator.**
- *Workflow:* a pack declares unit-aware lookup tables, such as an allowable
  versus temperature or a curve, and expressions interpolate them.
- *Touches:* FR-RULE-001/002; DEL-06-02.
- *Size:* S–M.
- *Rec.:* adopt. *Pri.:* P2. *Basis:* observed and documented.

**E3. Cycle-based cumulative usage.**
- *Workflow:* the user enters cycle counts per case pair. The pack supplies the
  curve and the method; the product supplies the stress ranges per location and
  sums usage.
- *Touches:* FR-RULE-001; DEL-06-01, DEL-05-02.
- *Size:* M.
- *Rec.:* defer. *Pri.:* P3. *Basis:* observed and documented.

**E4. Pressure-design checks per element.**
- *Workflow:* the pack supplies the wall-thickness formula. The product
  supplies design pressure and temperature from C1, and the section's
  thickness, corrosion allowance and mill tolerance. A per-element table shows
  required against available thickness.
- *Touches:* FR-RULE-001; DEL-06-01, DEL-03-02.
- *Size:* S–M.
- *Rec.:* defer. *Pri.:* P3. *Basis:* observed and documented.

**E5. Flange load check hooks.**
- *Workflow:* the product exposes the axial force and bending moment at flange
  entities per case. The pack's expression converts them to an equivalent
  pressure, or to whatever measure it defines, and compares.
- *Touches:* FR-RULE-001; DEL-06-01, DEL-03-05.
- *Size:* S.
- *Rec.:* defer. *Pri.:* P3. *Basis:* observed and documented.

**E6. Branch-connection evaluation hooks.**
- *Workflow:* at tees, publish run-side and branch-side moments separately, so
  that a pack can evaluate each with its own section property and modifier.
- *Touches:* FR-SOL-011, FR-RULE-001; DEL-03-04, DEL-05-03.
- *Size:* S once FR-SOL-011 branch flexibility exists.
- *Rec.:* defer behind FR-SOL-011. *Pri.:* P3. *Basis:* documented.

## Bundle F: Dynamics and specialty mechanics roadmap

PRD §12.1 places dynamics after the linear static core. Mature piping-analysis
products commonly include them. These items would each need a new or extended
deliverable and owner approval of scope.

**F1. Modal analysis.**
- *Workflow:* natural frequencies and mode shapes up to a cut-off frequency or
  mode count, with animated shapes and participation factors. A helper adds
  intermediate mass points on long spans.
- *Why:* the minimum dynamic screen (vibration-prone small bore, compressor
  lines). It is open mechanics and needs no code data.
- *Touches:* PRD §12.1 ("later modules may add ... dynamics"); new deliverable
  under PKG-04, plus DEL-07-05.
- *Size:* L. Mass assembly, an eigen solver, benchmarks and UI.
- *Rec.:* research and scope as the first dynamic module. *Pri.:* P2.
  *Basis:* observed and documented.

**F2. Response spectrum with user spectra.**
- *Workflow:* the user enters spectra; the product combines modes and
  directions and corrects for missing mass.
- *Size:* L. *Rec.:* defer behind F1; ship no spectrum libraries. *Pri.:* P3.
  *Basis:* observed and documented.

**F3. Harmonic and time-history loading.**
- *Workflow:* harmonic and time-history loads, and relief-valve thrust handled
  as a force spectrum.
- *Size:* L. *Rec.:* defer. *Pri.:* P3. *Basis:* observed and documented.

**F4. Buried pipe.**
- *Workflow:* distributed soil springs generated from user soil parameters,
  with automatic mesh refinement near bends.
- *Size:* L. *Rec.:* defer. *Pri.:* P3. *Basis:* observed and documented.

**F5. Joints with rotational limits and friction.**
- *Workflow:* hinge, ball and slip joints with rotational limits and friction,
  and tie rods with separate tension and compression gaps.
- *Touches:* FR-SOL-013/014; DEL-03-06, DEL-04-04.
- *Size:* M. *Rec.:* defer. *Pri.:* P3. *Basis:* observed and documented.

---

## Pressure-free Timoshenko option (owner choice)

This is a separate owner choice, not part of any bundle.

**What the choice is.** Whether SWBPIPE's pressure-free route should offer
Timoshenko straight members (transverse shear deformation) as an explicit
per-document choice, as the exact pressure route already does. Today the
pressure-free route is Euler–Bernoulli only.

**Evidence.**
- *Correlation pilot.* The pilot's comparison and slenderness-series records bear
  on this choice. They evaluate the external prover, so they stay in the private
  exchange folder (owner, 2026-10-26: "interoperate openly, evaluate privately"),
  where the owner can review them.
- *Public documentation.* The external prover's pipe stiffness is publicly
  described as including a shear term proportional to L/GA, with an
  unpublished shape factor. The pilot's Timoshenko values assume shape factor 2
  (κ = 0.5). The value is a question for the tool's vendor (DEL-17-01
  CQ-17-01-010).
- *Size of the effect.* The shear term grows as (r/L)². For ordinary pipe spans
  it is a fraction of a percent of deflection. In short, stiff legs of
  indeterminate systems (the pilot's L-bend and branch cases) it reaches a
  percent or two of reactions and moments. That is enough to show in
  side-by-side checks against tools that include it, and in thermal loads at
  anchors and nozzles.

**What already exists.** DEL-05-03 implemented Timoshenko straight members for the
exact pressure contract `3.0.0/exact_pressure_v3`:
- the shear model is an explicit document field, `euler_bernoulli` or
  `timoshenko_annulus_energy_v1`;
- the shear factor is energy-matched for an annulus, κ_E (Design
  `section-and-shear-decisions.md` D-6, `section-bases-and-timoshenko.md` §3);
- independent references and the certified recovery path cover it;
- arcs stay Euler–Bernoulli, with a disclosure.

The pressure-free route was deliberately kept Euler–Bernoulli and byte-identical
(H-1, DEL-04-07 Design).

**What adopting it would take** (size M).
- Admit the existing shear-model field on pressure-free documents. This is a
  model-document change, so it rides a schema version.
- Route the pressure-free frame preview through the existing
  `BeamTheory::Timoshenko` section.
- Re-freeze the pressure-free references a Timoshenko choice would touch. They
  are opt-in, so Euler–Bernoulli documents stay byte-identical.
- Add the desktop selection and report disclosure.
- Run the targeted numerical checks and independent review that Root
  `AGENTS.md` requires for numerical meaning.
- If retained-route memory moves, re-price it.

**Not settled by adopting it.** SWBPIPE's energy-matched κ_E and the shape
factor the pilot assumes (κ = 0.5) are different shear factors. Correlation
against a tool that uses another factor needs the shear assumptions aligned or
declared.

**Recommendation.** Offer it, as an explicit opt-in on the pressure-free route,
keeping Euler–Bernoulli as the default. Reasons:
- the formulation, references and certification already exist;
- it lets SWBPIPE include transverse shear deformation where a user's reference tool does;
- users can then match the shear assumption of the tool they correlate against.

Before building, ask the tool's vendor for the shape factor (CQ-17-01-010), so the correlation
declaration can be exact. Priority: medium, after the import and export
streams. It does not block the current correlation work, because correlation
declares any such difference with its evidence.

## Native US-customary MBF writing (owner choice)

This is a separate owner choice, not part of any bundle.

**What the choice is.** Whether SWBPIPE's MBF writer should also write the
MBF format's US-customary unit set natively, in addition to the SI profile.

**What is already decided** (Agent 0, 2026-10-26, O-2). The planned MBF import
(DEL-17-04; its Design `caepipe-mbf-import.md` §5.2 arrives with the importer
PR) reads US-customary files and re-exports them through the SI profile by
documented exact normalization (25.4 mm per inch, 0.45359237 kg per pound, the kilogram-force and
pound-force definitions, and 5/9 for Fahrenheit intervals). The result states
that normalization was applied. Round trips are therefore exact in meaning but
not in the file's unit set.

**What native writing would add.** A US-customary model would leave SWBPIPE in
the units it came in, so a user iterating between the two tools sees the same
numbers in both, and byte-reproducible round trips extend to those files.

**What adopting it would take** (size M).
- A second writer profile beside the SI one, with its own unit table and
  number formatting, refusing by name what it cannot write.
- Round-trip and byte-reproduction tests for the new profile on invented models.
- The targeted numerical checks and independent review that numerical meaning
  requires (Root `AGENTS.md`).

**Recommendation.** Defer until a US-customary user workflow needs it.
Normalization keeps meaning exact, and the import tranche plan already puts
pressure and Z-vertical first. Priority: low.

---

## Other research proposals

Evidence classes: **[observed]** seen in hands-on use during the survey;
**[documented]** read in publicly available documentation; **[inference]**
our reasoning, not observed.

**R1. Make the expansion-range basis explicit.** [observed] [documented]
- *Finding:* users can meet two ways of getting the thermal case. One
  subtracts the sustained solution from the operating one. The other solves
  the thermal load on its own. The two
  differ when supports are nonlinear.
- *Proposal:* record which basis SWBPIPE uses for each range combination in the
  run manifest. Let a rule pack's load-case mapping require one basis. Add a
  benchmark with a lift-off support where the two bases differ.
- *Touches:* DEL-05-02, DEL-06-01, DEL-09-03.

**R2. Modulus used for thermal and sustained solves.** [observed]
- *Finding:* users may expect to choose between temperature-dependent
  modulus and modulus at the reference temperature.
- *Proposal:* state SWBPIPE's choice in the run manifest and the report. If
  both are wanted, make it a recorded run setting, not a silent default
  (PRD §5.5).
- *Touches:* DEL-04-07, DEL-03-01 (`features/material-temperature`),
  DEL-08-01.

**R3. Pressure effects as separate, selectable mechanics quantities.** [observed]
- *Finding:* users may expect to choose:
  - two longitudinal pressure-stress forms;
  - a peak-pressure factor;
  - pressure elongation (Bourdon) applied to chosen case families;
  - a pressure correction to bend flexibility.
- *Proposal:* on the exact route, publish both longitudinal pressure-stress
  forms as named mechanics quantities, so that a pack picks one. State in
  results which pressure elongation effects the solve included. Treat any
  bend-flexibility pressure correction as a user-supplied modifier, not a
  built-in.
- *Touches:* DEL-05-03, DEL-04-07, DEL-03-03.

**R4. Rigid component representation.** [documented]
- *Finding:* one established representation of a rigid element is a fixed,
  very large stiffness, with its weight spread along its length. SWBPIPE
  scales stiffness (DEC-045) and applies component weight at the centre of
  gravity (W-15 b).
- *Proposal:* add one invented validation shape (a heavy valve between two
  supports, off-centre CoG) to quantify the expected difference. Record it as
  a known modelling difference in the correlation tolerance profile, not as an
  error.
- *Touches:* DEL-03-05, DEL-14-05, DEL-17-05.

**R5. Local-axis conventions for force comparison.** [documented]
- *Finding:* element local-axis conventions differ between tools, typically
  with separate rules for vertical elements and for point supports that take
  the adjacent element's axes.
- *Proposal:* state SWBPIPE's convention and the external prover's in the CSV
  parser's mapping notes. Then compare element-local forces through a declared transform, not
  row by row.
- *Touches:* DEL-17-05, DEL-04-01. Directly relevant to the correlation
  work.

**R6. Nonlinear support convergence and friction.** [documented]
- *Finding:* a common scheme iterates with a relative displacement tolerance
  at gap supports, with friction acting only after a gap closes.
- *Proposal:* document SWBPIPE's active-set stopping rule and friction model
  against such a scheme. Add invented gap-with-friction and resting-lift-off shapes to
  DEL-09-03 for later correlation.
- *Touches:* DEL-04-04, DEL-09-03.

**R7. Open hanger-sizing algorithm.** [documented] [inference]
- *Finding:* hanger sizing commonly relies on manufacturers' catalogues; we
  must not reproduce any.
- *Proposal:* specify an open procedure in our own words (C4), with invented
  catalogue records only. Validate its load and travel outputs against hand
  calculations before any correlation.
- *Touches:* DEL-04-07, DEL-03-07.

**R8. Mass-point spacing for dynamics.** [documented]
- *Finding:* a common rule derives intermediate mass-point spacing from the
  frequency of an equivalent simply supported span.
- *Proposal:* when F1 is scoped, derive SWBPIPE's own spacing rule from first
  principles and benchmark it against an analytical beam.
- *Touches:* the F1 deliverable.

**R9. Validation-case shapes for fuller correlation.** [inference]
- *Proposal:* invent these and run them privately in SWBPIPE and the external
  prover when fuller correlation runs are possible:
  1. expansion loop under thermal load;
  2. Z-bend with pressure on the exact route;
  3. tee with user-supplied branch modifiers;
  4. resting supports with lift-off;
  5. gap stop with friction;
  6. spring hanger sized by C4;
  7. constant-effort support;
  8. anchor settlement (W-16);
  9. uniform wind on an elevated line;
  10. static seismic g-load in three axes with each combination method;
  11. bellows with tie rods and pressure thrust;
  12. heavy rigid valve with off-centre CoG (R4);
  13. eccentric reducer (W-15 c);
  14. cold spring (C5);
  15. a 2,000-node rack model for SWBPIPE's own solve-time and conditioning
      statistics (D6).

  Each is our own invented geometry and data. Results, differences and
  agreement statistics stay private, in the exchange folder. That follows the
  stream-1 harness pattern under DEL-17-05.
- *Touches:* DEL-09-01/03, DEL-17-05, DEL-14-05.

**R10. Solve statistics as numerical-health evidence.** [observed]
- *Finding:* solve statistics such as equation count, bandwidth before and
  after reordering, and matrix size are useful to show at each solve.
- *Proposal:* add equation count, nonzeros and reordering fill to SWBPIPE's
  numerical-health record (D6), so that performance regressions are visible
  per run.
- *Touches:* DEL-04-05, DEL-04-06.

**R11. Import adapters for other tools' neutral files.** [observed] [inference]
- *Finding:* users expect to bring in models from other tools' neutral
  files.
- *Proposal:* before any SWBPIPE import adapter for a third-party neutral
  format, check the owner's rights to the format documentation. Only formats
  with public, permissive documentation qualify. This is a legal-first
  research item, not a build item.
- *Touches:* DEL-10-02, DEL-17-09, DEL-01-02.

**R12. Exporting the deformed geometry.** [documented] [inference]
- *Finding:* sending deformed geometry to plant-design tools supports clash
  review.
- *Proposal:* SWBPIPE's glTF review export (DEL-17-08) could add the deformed
  shape of a chosen case as a second node set. That is useful for clash
  review, and its privacy follows the result-export gating.
- *Touches:* DEL-17-08, DEL-12-02.

## Decisions (Agent 0, 2026-10-27, under the owner's delegation in #1662)

The owner's steer of 2026-10-27, recorded in the loop's `LOOP_INIT.md` by #1662, sets the goal of an app on
par with or superior to industry piping-analysis tools in modelling and UI, and delegates scope decisions
that clearly serve it to Agent 0. Agent 0 adopted the manager's recommendations as written. Each adopted
item becomes a commitment through its own deliverable's ScopeOfWork when its lane builds it; nothing here
changes a commitment by itself. Lanes: solver, reports, W-17/model (model document, grid, viewport,
desktop), CAEPIPE, numerical.

| Item | Decision | Reason | Lane |
|---|---|---|---|
| Timoshenko on the pressure-free route | Adopt now | the formulation, references and certification exist on the exact route; opt-in keeps existing documents byte-identical | numerical |
| Native US-customary MBF writing | Adopt later | exact normalization already round-trips meaning; build when a US-customary workflow needs same-unit files | CAEPIPE |
| A1 | Adopt now | core of everyday results review | W-17/model |
| A2 | Adopt now | the support-design deliverable users expect | reports |
| A3 | Adopt now | fast triage, keeps the incomplete-check rule | reports |
| A4 | Adopt now | expected visual standard | W-17/model |
| A5 | Adopt now | small, high-visibility | W-17/model |
| A6 | Adopt now | after A2/A4: composes them | W-17/model |
| A7 | Adopt now | small; pairs with the importer's local-axis convention | W-17/model |
| A8 | Adopt now | small envelope over existing rows | reports |
| A9 | Adopt now | data exists per entity | reports |
| B1 | Adopt now | design first: the biggest authoring-speed gap; write inheritance rules before code | W-17/model |
| B2 | Adopt now | core bulk editing | W-17/model |
| B3 | Adopt later | after B2's operation vocabulary | W-17/model |
| B4 | Adopt later | after B1 | W-17/model |
| B5 | Adopt now | small, frequent | W-17/model |
| B6 | Adopt later | labels only; low urgency | W-17/model |
| B7 | Adopt later | needs an id-collision policy; pairs with the importer | W-17/model |
| B8 | Adopt now | confirm what exists first | W-17/model |
| B9 | Adopt now | small; keyboard equivalents required | W-17/model |
| B10 | Adopt later | convenience | W-17/model |
| B11 | Adopt now | extends W-8 | W-17/model |
| C1 | Adopt now | foundation for multi-condition modelling; model-document version step | W-17/model |
| C2 | Adopt later | after C1; invented demo pack only | solver |
| C3 | Adopt later | after C1, on the pressure-free route | solver |
| C4 | Adopt later | high value; multi-solve procedure; no manufacturer data | solver |
| C5 | Adopt later | niche | solver |
| C6 | Adopt later | generator packs as rule-pack capability | solver |
| C7 | Adopt later | after C1 | solver |
| C8 | Decline for now | outside the piping focus until supports demand it | solver |
| D1 | Adopt now | reports are the deliverable users hand over | reports |
| D2 | Adopt now | small; follows result-export gating | reports |
| D3 | Adopt later | useful, not core | reports |
| D4 | Adopt later | with the boundary note in field help | W-17/model |
| D5 | Adopt now | protects work; expected baseline | W-17/model |
| D6 | Adopt now | small; helps large models | W-17/model |
| D7 | Adopt later | follows export gating | reports |
| E1 | Adopt later | after current rule-pack work | solver |
| E2 | Adopt later | needed by most real packs; tables stay user-supplied | solver |
| E3 | Adopt later | after E2 | solver |
| E4 | Adopt later | pack-driven | solver |
| E5 | Adopt later | pack-driven | solver |
| E6 | Adopt later | behind FR-SOL-011 branch flexibility | solver |
| F1 | Adopt later | research now: scope it as the first dynamic module; the idle numerical lane can do the research | numerical |
| F2 | Adopt later | behind F1; no spectra shipped | numerical |
| F3 | Decline for now | behind F1/F2 | numerical |
| F4 | Decline for now | specialised; revisit with users | solver |
| F5 | Adopt later | after the nonlinear-support work settles | solver |
