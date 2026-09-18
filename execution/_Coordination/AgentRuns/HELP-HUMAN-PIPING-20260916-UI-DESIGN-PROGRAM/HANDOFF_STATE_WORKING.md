# Handoff state — working

Updated 2026-09-17 by ROOT. Working surface; superseded lines are struck rather than deleted once the program has a second author.

## Where the program stands

- Phase 0 complete. Opening report at `instances/ROOT/OPENING_REPORT_2026-09-16.md` (pre-#789 shell, with the post-#789 re-screen deltas stated in the rendering boundary answer). Research returns A to D under `instances/RESEARCH/`.
- Sequencing with the piping session is ruled: [D-70](../../_DECISIONS/D-70_RULING_2026-09-17.md), recorded on this branch at `81f59e98b`. The baseline report from the piping session's characterization tranche is the handoff condition for redesign implementation, not for design work, which proceeds now.
- The rendering boundary is answered at `instances/ROOT/RENDERING_BOUNDARY_ANSWER_2026-09-17.md`: the redesign replaces the presentation layer and chrome, not the engine (Three.js, instancing, invalidation scheduler, ownership ledger, typed model index, picking).

## Open with the owner

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
