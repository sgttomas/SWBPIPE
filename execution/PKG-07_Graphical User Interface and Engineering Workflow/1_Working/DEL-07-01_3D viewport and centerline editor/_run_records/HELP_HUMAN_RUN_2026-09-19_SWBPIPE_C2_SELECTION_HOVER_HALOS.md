# SWBPIPE Tranche B, canvas lane, slice C2 — selection and hover as halos: bounded technical evidence

Date: 2026-09-19 (UTC). HELP_HUMAN (ROOT, Claude Fable 5.1) with the lane's Type 1 WORKING_ITEMS manager (Claude Fable 5.1), two Type 2 TASK children it dispatched (the halo implementer and the probe extension, both Claude Fable 5.1), and one Type 2 read-only reviewer (Claude Opus 5) dispatched by ROOT, under the owner's implementation authorization of 2026-09-18.

## Accepted basis

The implementation handoff's constraints 4 (picking repair preserved) and 5 (rendering foundation retained); design system V1.3 section 6 and the specification's named semantic change for selection (G-31); the lane brief and addenda. The owner's rule of 2026-09-19 for the stencil question: a renderer-construction change only if the picking tests are unchanged and it stays inside the D-72 limits. Source base `d20eb1294`.

## Implemented behaviour

Six files under `src/features/viewport/`, among them the new `viewportHalo.ts` with 24 tests. Named semantic changes: a selected element keeps its own colour and its edge line and gains a 2 CSS px outline in `canvas.selection`; an unselected hovered element, including hover that arrives from a label, gains a 1 px outline in `canvas.hover`; the halo shows through nearer geometry; overlapping halos read as their union; load arrows and the deformed overlay take no halo and no longer recolour; a hidden element shows none. The halo is driven by selection state alone and carries no result meaning. Mechanism: a depth mask and a screen-space shell, form (b) of the lane's three options, chosen by ROOT under the owner's rule because it draws the same picture as the stencil form at about a quarter of its cost on this host and changes no renderer construction; the stencil patch is kept in the lane's records. With nothing selected or hovered the scene draws what it drew before. Picking is byte-identical; nothing under `e2e/**` changed; the first profile's diamond cue is drawn as before.

## Evidence

`{RUN}/lanes/B-CANVAS/INDEX.md` and `{RUN}/lanes/B-CANVAS/CLOSEOUT_CHECKS_C2.json`: the independent review (PASS, no actionable finding), ROOT's own look in both themes, and the DEC-025 sweep, pass on all five surfaces at `031f1855c1732b1c92d9f7a1837933f25065b5e9`, summary SHA-256 `d0d3ec57cca3a6866cea9dcc5c2d93a2becc6f94d6db9c3f2c5d175d10a152a7`.

## Remaining work and authority boundary

For the owner, with the second profile's freeze package: the casing (P1 ASK-4). Drawn alone, the halo fails the design's pair rule in most samples of dense fixtures (42 of 200 pass at 1,000 pipes in light; 1 of 200 at 10,000), because over other tubes it reads 2.21:1 in light and 1.85:1 in dark; the product's own build passes only because the first profile's diamond satisfies the rule alone. The lane draws no casing, lowers no 3:1 and drops no pair check. Appearance left for the closing visual pass: the halo's and hover's contrast figures, coverage at device pixel ratio 1, the diamond beside the halo. Requests for the shell lane's files, not made here: the label plate's selected border colour in `styles.css`; a hovered-entity field in `uiDiagnostics.ts`. No usability, conformance or performance acceptance is claimed; PDU-045 and PDU-046 remain holds.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
