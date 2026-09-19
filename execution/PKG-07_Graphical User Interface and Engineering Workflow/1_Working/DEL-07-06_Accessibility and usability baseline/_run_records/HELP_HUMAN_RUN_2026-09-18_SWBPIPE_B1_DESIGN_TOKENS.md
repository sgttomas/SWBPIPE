# SWBPIPE Tranche B, slice B1 — design tokens in the product: bounded technical evidence

Date: 2026-09-18. HELP_HUMAN (ROOT, Claude Fable 5.1) dispatching one Type 2 TASK implementer and one Type 2 read-only reviewer directly, under the owner's implementation authorization of 2026-09-18. Mechanism: Claude Code `Agent` tool children; models as each return states them. Non-delegation by the children is instruction asserted.

## Accepted basis

Design system V1.3 (`tokens.json` 1.2) and the implementation handoff of the design program, merged in PRs #796 to #799; D-68 (WCAG 2.2 AA for touched controls). Source base `7866f0a3c2c846cf071f735ea9b263a44fb00ca9`. Run: `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION` (`{RUN}` below).

## Implemented behaviour

The product owns a byte-identical copy of the design system's token file and generates its stylesheet of custom properties from it; tests fail if the copy, the generated file or the import drift. The resolved theme is mirrored onto the document root so the token blocks follow the user's preference and the system's. Twenty-one of the workspace's twenty-five colour variables now take their values from tokens by role.

Four variables keep their values, each for a stated reason. Two mirror the viewport's colour constants, which the frozen benchmark instrument reads back; they belong to the canvas lane. The control border stays because the design's `border.strong` measures below the 3:1 that D-68's criterion 1.4.11 and the product's own resolved-style check ask of a control's boundary. The disabled ink stays because the design's `text.disabled` would have halved the contrast of disabled labels. Both contrast questions go to a design-system amendment.

Not changed: layout, structure, behaviour, any operation, gate, picking or viewport file, any existing test, any tolerance, oracle or limit. The slice draws no new surface. Visible differences are colour only: neutral surfaces and inks move to the design's values in both themes, and the selection bar takes the interaction blue.

## Evidence

The sealed briefs with hashes, the implementer's return and correction 1, and the independent code review (Claude Opus 5: PASS, no actionable finding, with four carried observations) are indexed at `{RUN}/briefs/_INDEX.md`. Check results and the tested and reviewed revisions are at `{RUN}/_run_records/CLOSEOUT_CHECKS_B1.json`. ROOT opened the product in both themes and confirmed that the document root and the shell agree and the variables resolve.

## Remaining work and authority boundary

The shell lane replaces the stylesheet's hard-coded colours as it rebuilds each surface; the canvas lane moves the canvas colours. This record establishes no review acceptance, usability acceptance, accessibility conformance, release or lifecycle change; PDU-045 and PDU-046 remain holds.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
