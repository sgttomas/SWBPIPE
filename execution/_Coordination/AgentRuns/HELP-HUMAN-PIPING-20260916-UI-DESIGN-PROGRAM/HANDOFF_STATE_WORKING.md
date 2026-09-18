# Handoff state — working

Updated 2026-09-17 by ROOT. Working surface; superseded lines are struck rather than deleted once the program has a second author.

## Where the program stands

- Phase 0 complete. Opening report at `instances/ROOT/OPENING_REPORT_2026-09-16.md` (pre-#789 shell, with the post-#789 re-screen deltas stated in the rendering boundary answer). Research returns A to D under `instances/RESEARCH/`.
- Sequencing with the piping session is ruled: [D-70](../../_DECISIONS/D-70_RULING_2026-09-17.md), recorded on this branch at `81f59e98b`. The baseline report from the piping session's characterization tranche is the handoff condition for redesign implementation, not for design work, which proceeds now.
- The rendering boundary is answered at `instances/ROOT/RENDERING_BOUNDARY_ANSWER_2026-09-17.md`: the redesign replaces the presentation layer and chrome, not the engine (Three.js, instancing, invalidation scheduler, ownership ledger, typed model index, picking).

## Open with the owner

Update 2026-09-18, UX specification accepted: UX-SPEC-01 returned `instances/UX-SPEC/` (UX_SPEC_V1.md, OPERATIONS_MAP.md with 277 rows and 29 classed gaps, RETURN.md); accepted by ROOT with its checks and decisions recorded in `briefs/_INDEX.md`. RESEARCH-F returned and was sent one correction (a false zero-occurrence claim for the phrase "Human review required" in desktop source); retention waits for the corrected file. DESIGN-SYSTEM-02 still running.

Update 2026-09-18, mock review decided: the owner approved ROOT's recommendation on all fourteen screen questions (direction record §11). Three children launched in parallel from sealed briefs: DESIGN-SYSTEM-02 (Fable; revision V1.1 folding in the decisions, the frames' proposals and the token gaps), UX-SPEC-01 (Fable; the UX specification mapped to the product's operations and typed interfaces), RESEARCH-F (Opus; the inventory of governed texts, registry entries, lint anchors and tests binding the nine decision-packet items). After DESIGN-SYSTEM-02: MOCKS-02 regenerates the frames. After RESEARCH-F: ROOT drafts the decision packet with the frames as evidence.

Update 2026-09-18, mocks returned and accepted: `instances/MOCKS/` holds fifteen self-contained frames of the nine states from one sample model ("Loop 4 header"), the contact page `frames/index.html`, `sample_model.md`, `MOCKS_V1.md` (per-frame decisions as proposals for the design system, nine departures, fourteen questions for the owner, six token gaps) and the child's generator and verification evidence. Open with the owner: the fourteen screen questions. Next: the owner's mock review, then the decision packet with the frames as evidence, then the UX specification; the acceptance-criteria proposal (D-70 effect 6) waits on the piping session's observation report.

Update 2026-09-18, mocks launched and rendering brief delivered: MOCKS-01 sealed and launched (Fable; fifteen offline HTML frames from one sample model, return under `instances/MOCKS/`). The rendering brief owed under D-70 effect 4 is written at `instances/ROOT/RENDERING_BRIEF_2026-09-18.md` from the design system's figure language, with the notice `../../NOTICE_2026-09-18_RENDERING_BRIEF.md`; the owner relays it to the piping session. Remaining from this program: the mocks review, the decision packet after the mocks, and the acceptance-criteria proposal (D-70 effect 6) after the observation report.

Update 2026-09-18, design system returned and accepted: `instances/DESIGN-SYSTEM/` holds the design system V1 (88 colour tokens with light and dark values, 85 plain tokens, the marks vocabulary, the components to the cell state, the presentation language chosen as the report figure with a theme-aware edge line and direction arrows on restraint glyphs, the copy rules), `tokens.json`, an offline specimen and the child's scripts. Eight open items are listed in its §8, each already an item of the decision packet or a question for the mocks. Next: the mocks brief (fifteen HTML frames from one sample model), then the rendering brief to the piping session from §6 of the design system, then the decision packet.

Update 2026-09-18, later: the checked mark is settled (kept) and the owner agreed all eleven recommendations on the product shape, the mock states and the new work (direction record §10); brief at V1.4. Phase 3 opened: DESIGN-SYSTEM-01 sealed and launched (Fable). Next after its return: the mocks brief (fifteen HTML frames from one sample model), then the rendering brief to the piping session, then the decision packet after the mocks.

Update 2026-09-18, direction confirmed: the owner confirmed readings (a), (b) and (e) of the direction record, corrected the view names to Table, Model and Both, and chose numeric node IDs throughout; reading (d), the checked mark, is under discussion (the owner asked for more) and is provisional in phase 3. Brief at V1.3. Phase 3 begins with the design-system brief.

Update 2026-09-17, RESEARCH-E returned and accepted: CAEPIPE's documented neutral format is the `.mbf` model batch file (import and export, ASCII, specified in Appendix A of the public User's Manual). Its LAYOUT section has the same shape as the owner's grammar: one line per node carrying what arrives at it, From-ness as a prefix letter, a branch as a line re-anchoring to an earlier node, material and section propagating until changed, extra items at a node on Location lines; the absolute-coordinate form the brief wants is already in the file. Sixteen mapping gaps are named; two need the owner's word and are added to the direction record §8: a tee is a node attribute in CAEPIPE, not an element type (item e), and node numbers are numeric in the file while the brief specifies alphanumeric IDs (item f). Load cases and combinations have no section in the file; provenance, checked tags and export metadata have no place in it and stay in the product's own store.

Update 2026-09-17, direction given: the owner did not choose one of the three returned directions. In two messages the owner set out a synthesis, recorded verbatim with ROOT's transcription at `instances/ROOT/DIRECTION_DECISION_2026-09-17.md`: the tables are the model and the 3D view is generated from them; the layout table uses the CAEPIPE grammar (one row per node, element type a column, attachments as separate tables with marks on the node row) because export compatible with CAEPIPE is wanted; the stages are the workflow across pages of tables and the canvas is not a fixture; three views per stage (Table, Model, Mixed) remembered per stage; the canvas keeps a few gestures that are row shortcuts; agent proposals accepted row by row with multi-row and batch available; the agent checks the engineer's work without altering the tables, on a Review page shared with report writing that keeps iterations. The brief is amended to V1.2 from those answers; the transcription awaits the owner's confirmation. RESEARCH-E (CAEPIPE exchange format, Opus) is launched from a sealed brief. Phase 3 (design system, mocks, rendering brief, decision packet) follows confirmation.

Update 2026-09-17, phase 2 returned: the CONCEPTS-01 child returned three concept directions (A Layout Sheet, B Workbench, C Run Book) with nine state-7 wireframes at `instances/CONCEPTS/`. ROOT reviewed the return against the sealed brief's acceptance section and accepted it: three distinct organising ideas, eight states storyboarded for each, every mandatory disclosure M-01 to M-17 homed in each, wireframes present and matching the shell tables, recommendation reasoned. The child recommends C with A's sheet grammar and B's probe carried in; the owner chooses. Open with the owner now: the direction. After the choice: design system, mock drafts, the rendering brief to the piping session, and the decision packet (maturity line, boundary-sentence placement, product rename, plus the four further items the concept return names in its §6).

Update 2026-09-17, later: the owner answered all seven questions in session and then accepted the brief with one correction and answered its open questions. `instances/ROOT/DESIGN_BRIEF_V1.md` is the accepted V1.1 brief; phase 1 checkpoint passed. Product name is now SWB Piping Designer (SWBPIPE). Phase 2, concept directions, is next. The list below is retained as history.

The seven opening questions, as first put:

1. What was the session that made you say "I don't like it"? What were you trying to do and where did it fall apart?
2. Who is the one user whose delight matters most: you running a real analysis, a stress engineer at a firm deciding whether to trust it, or a layout designer routing pipe?
3. Model-first or table-first? Should that person live in the 3D canvas, or in a grid with 3D as verification?
4. What is the bar? Two or three products, in any domain, that feel like what this should feel like; which piping tools do you respect versus want to beat?
5. Where does the agent live in the UI? A visible collaborator with its own panel, or an invisible actor whose proposals arrive as ordinary reviewed changes?
6. Governance on the surface: how much of the boundary, provenance and hash apparatus must stay visible at all times versus on demand? Is "Technical preview" a permanent footer?
7. Platform: macOS native window first? Minimum size you care about? Dark mode? Keyboard-first?

A naming decision is also owed. The domain `openpipestress.com` resolves to GitHub Pages and serves the documentation site of a separate open-source pipe stress project, PSI by denisgomes, titled "OpenPipeStress: PSI" (HTTPS fetch on 2026-09-17, HTTP 200). The product name in this repository collides with that project. Brand, wordmark and design-system naming wait on the owner's call.

## Owed by this program

- Design brief after the Q&A (phase 1 checkpoint).
- Rendering brief to the piping session after the concept phase, for its separate overlay and deformation observation run (D-70 effect 4).
- Proposed acceptance criteria for the redesigned product, for a later owner ruling (D-70 effect 6). Any proposal carries a score-independent rationale and is frozen before use.

## Owed by others

- Piping session: adopt D-70 in its run record and deliverable Remaining entries; run the bounded baseline tranche; deliver the baseline report with per-run canvas dimensions, device pixel ratio and rendered label count.

## Working notes

- The app runs locally from this worktree with the dev server configuration in the worktree's `.claude/launch.json` (gitignored) after `npm ci` and `npm run build:wasm --workspace apps/desktop` in `projects/chirality-piping`.
- Constraints that bind the design and are not the design program's to change: the lint-anchored "Technical preview" footer text, the DEC-081 claims vocabulary, DEC-051 on egress and consent indication, DEC-037 on writable rule expressions, DEC-094 and SCA-009 on the single palette surface, the PDU-045 and PDU-046 usability holds. Each is either honoured in the design or brought back to the owner as a decision with evidence.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
