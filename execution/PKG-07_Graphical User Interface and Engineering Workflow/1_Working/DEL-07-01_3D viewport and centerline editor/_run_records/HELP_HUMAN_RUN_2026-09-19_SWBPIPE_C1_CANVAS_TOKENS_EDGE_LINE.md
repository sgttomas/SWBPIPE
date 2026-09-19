# SWBPIPE Tranche B, canvas lane, first pull request — canvas colours from tokens and the edge line: bounded technical evidence

Date: 2026-09-19 (UTC). HELP_HUMAN (ROOT, Claude Fable 5.1) with a Type 1 WORKING_ITEMS lane manager (Claude Fable 5.1; two managers across two sessions of ROOT, the second started from the lane's records), Type 2 TASK children dispatched by the managers (palette implementer, instrument inventory, orbit probe, edge-line implementer), and two Type 2 read-only reviewers (Claude Opus 5) dispatched by ROOT, under the owner's implementation authorization of 2026-09-18 and the re-sequencing of 2026-09-19. This pull request is the one place where work that looks visual comes before the control layer, for the reasons in the lane's sealed addendum 4.

## Accepted basis

The implementation handoff's ten constraints, above all 4 (picking repair preserved) and 5 (rendering foundation retained); design system V1.3 section 6 and the `canvas.*` tokens; the lane brief and sealed addenda 1 to 5, indexed at `{RUN}/lanes/B-CANVAS/INDEX.md`. Source base `172cd5689`.

## Implemented behaviour

Eleven product files: ten under `src/features/viewport/` and the granted rows of `src/styles.css`. Every canvas colour reads the `canvas.*` token of its role, with live repaint in both themes and no colour literal left in the folder (a test fails on a new one). The figure is drawn by a matte unlit material; two scene lights and eleven dead builders are gone. Named semantic changes: the figure gains a 1 CSS px edge line in `canvas.edge` inside the silhouette of pipes and the four placeholder kinds, with a rim line on a visible cap; the selected colour moves from orange to `canvas.selection` and is still a recolour until slice C2; the ground moves to `canvas.bg`; the surface behind the canvas, the no-WebGL fallback and the length scale follow the theme. An empty layer replacement no longer raises two console errors. Picking is byte-identical and its 69 tests pass first and last. Nothing under `e2e/**` changed; from this change the benchmark instrument's first profile is a recorded demonstration and no longer describes the product's cue colours (ROOT's decision in addendum 2, ASK-11); the second profile is prepared in the lane and its freeze goes to the owner before any timed run.

Not built, with reasons recorded: side rings at free ends and section changes (no pipe end is visible outside a node sphere until slice C5). Provisional, adopted by the owner on 2026-09-19 (C1-D1): the deformed shape on `canvas.vector`.

## Evidence

`{RUN}/lanes/B-CANVAS/INDEX.md` and `{RUN}/lanes/B-CANVAS/CLOSEOUT_CHECKS_PR1.json`: the early review of the first part (FINDINGS, all four answered), the complete review of the combined candidate (PASS, no actionable finding), ROOT's own look in both themes, and the DEC-025 sweep, pass on all five surfaces at `16d339ac0d9950cf769409fcb9bd69455bd85bf1`, summary SHA-256 `428d7f87269cf28ba73cda325179728e7673797003988a63eb76ee104c95cf1d`. The lane's orbit probe is guidance and decides nothing about D-72: on this host the uncapped orbit is bound by the main thread, so the line's GPU cost is invisible to it.

## Remaining work and authority boundary

Appearance left for the closing visual pass, with numbers in the lane's return: derived silhouette shades (worst 1.61:1 on `canvas.bg`, a node or rigid element in dark), the line at device pixel ratio 1, the line on a selected element until C2, roles the design does not name. Three residual risks from the review, no action asked: the edge width is captured once while the cue reads the pixel ratio live; the partial-torus guard; the untested orthographic branch. No usability, conformance or performance acceptance is claimed; D-72 qualification runs last in Tranche B; PDU-045 and PDU-046 remain holds.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
