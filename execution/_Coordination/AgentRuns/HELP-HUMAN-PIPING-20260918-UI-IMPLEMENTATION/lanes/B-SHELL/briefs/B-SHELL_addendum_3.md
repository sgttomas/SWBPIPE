# Sealed addendum 3 to the B-SHELL lane brief — the control layer first

Sealed by ROOT (HELP_HUMAN, Agent 0) on 2026-09-19T05:16Z. Everything in the lane brief and in addenda 1 and 2 still binds, except the clauses this addendum names and replaces.

## Why

On 2026-09-19 the owner re-sequenced the program: the control layer first, the visual redesign last. The owner's words, ROOT's recommendation that they refer to, and ROOT's reading are recorded in `{RUN}/instances/ROOT/OWNER_DIRECTION_2026-09-19_CONTROL_LAYER_FIRST.md` (SHA-256 `bc41f82680cfcb69a193ee3820cb0861f36fbe872fae970df1be1729f9a23268`; it reaches `main` with the handoff's pull request). The plan's amendment 1 is in `{RUN}/ORCHESTRATION_PLAN.md`.

The control layer is what each control does and what it may not do: the one typed-operation route, the tables and the canvas as projections of the canonical model, run standing, the generation gates, undo and redo, reviewed application, and a control that is absent or disabled with its reason where its operation is a gap. The handoff's constraints that protect results are all in that layer, and one slice of it has landed. The appearance is where the open questions are, and they are better answered once, on real controls.

## What stays

- **The order of slices:** B2F, then B3, B4, B5, B6, B7, as the brief and addendum 2 give them.
- **Layout follows the frames from the start.** Regions, their order, what is docked and what slides over, which controls a surface has, the table's gutter, footer and row expansion, the status bar's chips: these are structure, and they are built to the specification and the named frames now. A shell built without the frames would be built twice.
- **Every limit of the brief.** The one route, result integrity, the picking repair, no tolerance, oracle or limit altered to obtain a pass, constraint 6, named semantic changes, the browser-test lock, both Playwright lanes.

## What a slice no longer tunes

Colour, contrast, washes, shadows, fine spacing and type detail. Build each surface plainly on the tokens and the `--ui-*` variables that are already in the product. Write no colour literal and ask for no new token. Where a frame shows an appearance that today's variables cannot give, use the nearest existing variable and list it in the slice return under **appearance left for the closing pass**. Do not pin appearance in a test: no new pixel, colour or computed-style assertion beyond those that exist. Tests that pin geometry and structure are wanted.

The closing visual pass comes after both lanes' control work. It settles the design system's open contrast items on the product's real controls, changes token values, adopts them, and takes one design-fidelity review by screenshot over every surface. Leave it a clean job: every colour, border, radius and spacing through a token or a `--ui-*` variable, stable test ids, no literals.

## Clauses this addendum replaces

1. **Addendum 2, the last sentence of "Token adoption"** ("B3 builds its controls from `border.control`, not from `border.strong`"). Design system V1.4 is parked with its branch (ROOT's decision DS5-D3, open to the owner) and the token adoption slice waits with it. B3 builds a control's boundary from `--ui-border` and its disabled ink from `--ui-disabled-text`, as they are; slice B1 kept both, and the product's own resolved-style check holds the boundary at 3:1. Addendum 1's instruction stands: a frame takes `--ui-border`, a separator stays on `--ui-divider`. No merge of V1.4 is coming; do not wait for one.
2. **The brief's "Review" clause.** For each slice ROOT runs a fresh read-only Opus code review over the complete frozen diff; a slice merges only on PASS with no actionable finding, then ROOT's DEC-025 sweep and CI. For a slice that draws a surface, ROOT's review brief carries a structural checklist drawn from the specification and the named frames (regions, order, each control present, absent or disabled with its reason), and the reviewer checks it from the code and the tests. There is no design-fidelity review per slice. There are two in the lane's life: one on B3's candidate before it merges, for structure only (layout, regions, order, presence; not colour, contrast or spacing), because everything later hangs on the shell; and the closing pass's review of every surface.
3. **The brief's "Accessibility" clause.** For every control a slice touches, in that slice: name, role, state, keyboard operation, focus order, a visible focus indicator drawn with the existing focus token, target size. The contrast criteria (WCAG 2.2 criteria 1.4.3 and 1.4.11) are checked once, at the closing visual pass, on the real controls. Each slice return lists the controls it touched under **contrast not yet checked**. PDU-045 and PDU-046 remain holds; claim no usability acceptance.
4. **The brief's "Children" clause, the words "(general-purpose, background)".** Launch a child in the foreground (`run_in_background: false`) whenever its return is the next thing you need: its reply then comes back to you as the result of your own `Agent` call, in the same turn, and nothing goes through ROOT. ROOT probed this on 2026-09-19T05:14Z: a nested foreground child's reply reached its parent directly after the child had worked for about half a minute, and no notice came to ROOT. Children on disjoint paths may be launched together in one message, in the foreground. Launch in the background only when you have other work to do while the child runs; if you then stop while it is alive, the host delivers its completion to ROOT and not to you, and ROOT relays it by file and hash as before (`{RUN}/tools/relay.py`). Not probed: a foreground child that runs for a long time. If a foreground call is cut off, resume the child by message with its agent id and tell ROOT what you saw. While you wait on a foreground child you cannot read a message from ROOT; ROOT knows that, and a pause then takes effect at the child's return.

## Tests that carry the control layer

Name them in each slice return, with the command and the result.

- **B2F:** each repair's failing test first, as addendum 2 says.
- **B3:** the guard that no file other than `workspaceSession.ts` imports a `*SessionState.ts` hook, and the session key-set test, both in B3's first commit (addendum 2). The three views keep panel state mounted, and a test shows it.
- **B4:** the operation-equivalence test of constraint 1: every cell edit issues the same typed operation, through `applyModelOperation` or `applyOperationBatch`, and reaches the same applier result as today's route for the same change. Sort and filter are views: a test shows that neither changes the model, the revision or the solve-input basis. The Checked mark of `DEC-104` is interface state and a test shows it writes nothing to the model.
- **B5:** the inspector edits through the same route as the table; one test drives the same change from both and compares the operations.
- **B6:** the presentation test that a Historical record gains no overlay, chip or readiness cue; the Stale standing is not drawn.
- **Every slice that adds a control:** a table in the return with one row per control: the operation it maps to, or the gap id and class with the reason shown to the user, and the test that holds the row.

## If you are a new manager

The manager that ran slice B2 belonged to ROOT's session of 2026-09-18 and 2026-09-19. If you were started later you have none of its context, and its children cannot be resumed. Read, in this order: the lane brief (`{RUN}/briefs/B-SHELL_shell_lane.md`); addenda 1, 2 and 3; `{RUN}/lanes/B-SHELL/returns/B2_RETURN.md`; `{RUN}/lanes/B-SHELL/reviews/B2-REVIEW_RETURN.md`; `{RUN}/instances/B-SHELL/LANE_LOG.md` with `B2_RENDER_ORDER_FINDING.md` and `B2_SESSION_OBJECT_FINDING.md`; then `src/features/workspace/workspaceSession.ts` and the six state hooks. Keep the lane log going in the same file. Your first slice is B2F.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
