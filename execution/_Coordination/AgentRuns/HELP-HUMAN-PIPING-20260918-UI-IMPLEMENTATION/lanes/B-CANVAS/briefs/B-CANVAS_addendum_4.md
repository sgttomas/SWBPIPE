# Sealed addendum 4 to the B-CANVAS lane brief — the control layer first, and what it changes here

Sealed by ROOT (HELP_HUMAN, Agent 0) on 2026-09-19T05:16Z. Everything in the lane brief and in addenda 1, 2 and 3 still binds, except the clauses this addendum names and replaces.

## Why

On 2026-09-19 the owner re-sequenced the program: the control layer first, the visual redesign last. The owner's words, ROOT's recommendation that they refer to, and ROOT's reading are recorded in `{RUN}/instances/ROOT/OWNER_DIRECTION_2026-09-19_CONTROL_LAYER_FIRST.md` (SHA-256 `bc41f82680cfcb69a193ee3820cb0861f36fbe872fae970df1be1729f9a23268`; it reaches `main` with the handoff's pull request). The plan's amendment 1 is in `{RUN}/ORCHESTRATION_PLAN.md`.

## What does not change: the first pull request

The lane finishes its first pull request as addenda 2 and 3 seal it: slice C1's first part with findings F1 to F3 disposed, the guidance probe T1, slice C1E (the edge line, mechanism A) and slice C1b (the seven stylesheet rows, the two mirrors, the comment sentence and their test). ROOT told the owner why this is the one place where work that looks visual comes first:

- the colour work is already built on this branch, and slices C2 to C5 edit the same material and resource code, so they stack on it;
- the colours cannot reach `main` without the edge line, because in light the figure is paler than today's product without it (addendum 2);
- the D-72 ruling measures orbit with the edge line on, so its cost has to be known early.

Build the edge line to the design and to the limits. Do not tune its look beyond the design's stated values.

## The order after it

C2, C3, C4, C5, as the brief gives them. They are mechanisms, and each carries behaviour: selection that leaves the element's own colour in place (G-31), isolate as dimming with a stated pick rule and a total hidden count (G-30), the label budget with its All, Budget and Off control, and real outside diameter with the HUD and the fitted camera (G-32). Build each with the `canvas.*` tokens as they are. **C6 (`canvas.edgeAlt`) leaves the lane's order** and goes to the closing visual pass.

Addendum 2's dispositions of P1 ASK-4 (the casing) and ASK-5 (the stencil buffer and draw-over) stand: C2 brings numbers and options, and no renderer-construction change is returned in a slice without the owner's word. C2's return says which option it built as the working form and what each other option would cost. If the choice blocks the mechanism itself, bring it to ROOT at once; otherwise it joins the package below.

## Appearance questions are parked, as one package for the owner

Record each in the slice return under **appearance left for the closing pass**, with the numbers you have. Do not tune them and do not wait on them.

- the casing's look (P1 ASK-4), once the mechanism's options are known;
- the derived silhouette shade of roles other than the tube: the manager computed the worst as a node or rigid element in dark, `#3a3f45`, at 1.79:1 on the held ground and 1.61:1 on `canvas.bg`;
- the roles the design does not name (`node`, `routeGridAxis`, `routeGridLine`);
- the deformed shape's ink (ROOT's decision C1-D1, provisional) and the deformation view of design system §6.8;
- `canvas.edgeAlt` (C6).

## Clauses this addendum replaces

1. **The brief's "Review" clause, and addendum 3's "the same reviewer backchecks F1 to F3".** For each pull request ROOT runs a fresh read-only Opus code review over the complete frozen diff; it merges only on PASS with no actionable finding, then ROOT's own look at the build in both themes, ROOT's DEC-025 sweep and CI. There is no design-fidelity review per slice; the closing visual pass takes one, by screenshot, over every surface. The early reviewer of C1's first part belonged to ROOT's session of 2026-09-18 and 2026-09-19 and cannot be resumed after it. So the first pull request's review brief carries the early review's return (`{RUN}/lanes/B-CANVAS/reviews/C1-REVIEW_RETURN.md`) and addendum 3, and its reviewer re-reads the dispositions of F1, F2 and F3 as part of reading the whole diff.
2. **The brief's "Accessibility" clause.** For every HUD or canvas control a slice touches, in that slice: name, role, state, keyboard operation, focus order, a visible focus indicator, target size. The contrast criteria (WCAG 2.2 criteria 1.4.3 and 1.4.11) are checked once, at the closing visual pass. Each slice return lists the controls it touched under **contrast not yet checked**. PDU-045 and PDU-046 remain holds; claim no usability or performance acceptance.
3. **The brief's "Children" clause, the words "(general-purpose, background)".** Launch a child in the foreground (`run_in_background: false`) whenever its return is the next thing you need: its reply then comes back to you as the result of your own `Agent` call, in the same turn, and nothing goes through ROOT. ROOT probed this on 2026-09-19T05:14Z: a nested foreground child's reply reached its parent directly after the child had worked for about half a minute, and no notice came to ROOT. Children on disjoint paths may be launched together in one message, in the foreground. Launch in the background only when you have other work to do while the child runs; if you then stop while it is alive, the host delivers its completion to ROOT and not to you, and ROOT relays it by file and hash as before (`{RUN}/tools/relay.py`). Not probed: a foreground child that runs for a long time. If a foreground call is cut off, resume the child by message with its agent id and tell ROOT what you saw. While you wait on a foreground child you cannot read a message from ROOT; ROOT knows that, and a pause then takes effect at the child's return.
4. **When D-72 is qualified.** The four gated runs and the six observations run at the very end of the program, after the closing visual pass, because they must measure what ships. Nothing else about the ruling, the instrument or the second profile changes: the freeze still goes to the owner as one package before any timed run; builds stay clear of timed measurement; no tolerance, oracle or limit is altered to obtain a pass. T1's guidance measurement of the edge line's frame cost stays early, and hover halos stay behind a probe in C2.

## First steps from the pause point

The lane stands at `3a5f7c103bde915d800792112dec2dea19a0813e`, a labelled pause point, clean. The manager's pause status is retained verbatim at `{RUN}/lanes/B-CANVAS/returns/B-CANVAS_PAUSE_STATUS_2026-09-19.md`.

1. Merge `origin/main` at the clean head (slice B2 of the shell lane merged as `b9a07040857d2e6d3ec0f6a530697720801c7b59`; the handoff's records follow it). Then run the picking tests first, the viewport folder, `tsc -b` and the build. The shell lane's note for the second profile stands: after B2 the executed session code lives in `workspaceSession.ts` and the six state hooks, not only in `src/App.tsx`.
2. F3, tests first. The sealed correction is `{RUN}/instances/B-CANVAS/briefs/C1A-PALETTE_correction_1.md`; if the owning child cannot be resumed, a new child takes F3 from a new sealed brief that carries that correction's text for F3.
3. T1: re-run the tool's self-test, run its A/A and `resources` series, write its README and summary; retain, review the tool, commit. Its trial numbers so far are observations and not evidence.
4. Finish, seal and index `C1E-EDGE.md` from `{RUN}/instances/B-CANVAS/drafts/C1E-EDGE_draft.md`; launch it; then C1b; then return the first pull request's candidate to ROOT.

## If you are a new manager

The manager that ran the lane to the pause point belonged to ROOT's session of 2026-09-18 and 2026-09-19. If you were started later you have none of its context, and its children cannot be resumed. Read, in this order: the lane brief (`{RUN}/briefs/B-CANVAS_canvas_lane.md`); addenda 1 to 4; `{RUN}/lanes/B-CANVAS/returns/C1A_RETURN.md`; the pause status named above; proposals P1, P2 and P3 under `{RUN}/instances/B-CANVAS/proposals/`; the early review; the slice record `{RUN}/instances/B-CANVAS/slices/C1_PART_A.md`; the C1E draft; `{RUN}/instances/B-CANVAS/tools/STATUS.md`; the two children's pause statuses under `{RUN}/instances/B-CANVAS/returns/`. The lane has no lane log yet: start `{RUN}/instances/B-CANVAS/LANE_LOG.md` and keep it.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
