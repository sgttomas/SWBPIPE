C4 preparation is complete, read-only. The brief hash matches `697f9345210fc02b2f702c3ed03ae26f135cb708cd40ae143922332b4cffcd6d`. Checkout `9fb4b88b…` remains clean; its production tree equals the required `226b1db5…` basis. No files, agents, tests, builds, UI sessions or ports were created.

The smallest useful C4 change is **Budget / All / Off with a budget derived from the actual drawing area**, followed by priority-aware placement and the shell’s current-row binding.

Today:

- `PipeViewport.tsx:449` stores a boolean; the control simply toggles it.
- `viewportSelection.ts:609` caps candidates at 80, prioritizing primary, hover, selection and diagnostics, then distance from the model centre.
- `PipeViewport.tsx:1339–1394` projects those candidates, tries one position, and hides overlaps using a linear search through previously placed rectangles. It does not search around tubes/glyphs.
- Diagnostics report `budget: 80` regardless of drawing size (`PipeViewport.tsx:1207`).
- Labels currently include typed entity targets, not only ordinary node annotations.

I propose one Astra/low TASK, launched only after activation, with two internal checkpoints in one integration batch:

1. **Pure policy and layout surface.** Add `viewportLabels.ts` and focused tests defining `ViewportLabelMode = "budget" | "all" | "off"`, CSS-area budgeting, deterministic candidate priority and placement. Preserve `viewportSelection.ts` and its picking tests byte-identically; stop calling its historical capped-label helper from the new production path. Use actual canvas CSS dimensions, never backing-buffer pixels or outer-pane dimensions. The function must yield 138 for 603×828 and 230 for 1000×828; today’s smaller drawing receives its actual smaller budget.
2. **Live control and projection.** Integrate into `PipeViewport.tsx`, replace the boolean toggle with an explicitly named mode and L accelerator, and reuse the existing resource label-update/invalidation mechanism. Keep mode through stage/view changes and ordinary edits; default to Budget on a new project session. Do not introduce persisted view-state storage in C4—Budget/Off persistence belongs to the scheduled G-17 carrier, and All must never become a persisted preference.

Use bounded placement search with a spatial occupancy grid rather than extending the current pairwise overlap scan. Measure actual plate dimensions, project candidates before spending the budget, and keep labels inside the drawing and away from geometry. Candidate exhaustion must not masquerade as a filled budget. Additional rendering cost remains unqualified; no timed probe is authorized.

The exact ownership handoff should be:

| Owner | Surface |
|---|---|
| Canvas | New label policy/layout module and tests; `PipeViewport.tsx`; narrowly necessary resource callback tests; dedicated C4 journey; later `profiles/d72` draft |
| Shell/B4 | `App.tsx`, table focus publication and any controller bridge; `styles.css`; `uiDiagnostics.ts`; shared source/dist journey amendments |
| ROOT | Interface acknowledgement, profile policy disposition, independent review and combined validation |

For current-row priority, reuse the existing focus publication where possible: `App.tsx:403` already publishes `selectionState.focusKey`. B4 should confirm its row-to-node interpretation and supply a resolved `currentRowNodeKey`, rather than canvas inventing another current-row state or treating primary selection as equivalent. Derive restrained/loaded/branch priorities only from represented model relationships; do not fabricate missing node-data or detailed load/restraint plate capabilities.

The already approved diagnostics handoff is additive optional `labels.mode`. Preserve truthful `renderedCount` and report the computed budget. Shell owns that type change. Preserve C3 Hide precedence, dimmed-label styling, literal isolation, and unchanged picking/selection dispatch.

Two specification tensions need explicit disposition before claiming complete C4:

- UX §4.5 says selected/hovered plates and the current row’s node always show. My recommendation is that Off suppress ordinary labels while retaining these required annotations. That differs from the historical instrument’s “Off means zero” assumption and P1’s corresponding proposal; neither historical contract should be silently rewritten.
- “Always show,” “never overlap,” and the fixed Budget cap cannot all hold when required annotations alone exceed capacity—for example, 139 required labels at the 138-label boundary. Return unplaced required identities from the pure planner and expose this conflict for ROOT’s design disposition; do not silently exceed a cap or drop a required label and claim compliance. Pure budgeting and ordinary-case layout work can proceed independently.

This also affects real-canvas tests: required annotations must not cover their geometry’s click point. Existing journeys, including C3, must explicitly select Off rather than assume one click disables labels. Preserve their actual pointer-hit assertions.

Focused verification should cover mode cycling by pointer/L, CSS-area versus DPR, resize and pane changes, deterministic priorities, collision/obstacle avoidance, hidden versus dimmed entities, current-row changes without selection changes, session reset, and stable resources/settling. A connected journey should resize the canvas, cycle modes, focus another table row, select/hover geometry, Hide/Isolate/Show All, and verify model/history/Current–Historical invariance. Include one combined native keyboard/resize witness because browser rendering alone does not establish native interaction.

The dependent instrument increment is the already authorized sibling `e2e/ui-foundation/profiles/d72`, initially **DRAFT_UNFROZEN**. Its label admission rules must reflect the settled semantics. First-profile fixtures, manifests, helpers, oracles and tolerances remain unchanged. Owner-held profile freeze and timing, casing, deformation, closing appearance work and C5 remain separate. Use one coherent batch, focused development checks, independent whole-diff review and one final sweep.

Consulted identities, in addition to the unchanged Root/role/Piping/loop hashes recorded in C3’s `BASELINE.json`:

```text
UX_SPEC_V1.md                  2141c1e844109d4869287048c695e79677f423acae965d5dd0436b7f6aeff682
DESIGN_SYSTEM_V1.md            5aef3bdf5746b0f93f9419b4c7ad459eb634a71db28a846f19133b225f277c81
OPERATIONS_MAP.md              47e2aa303a358b77fa89439aac198146dbc0a2a13cef9fb86521789888f111c2
B-CANVAS_canvas_lane.md        df89f5b622a63f206ccc0f879e7255642e4b9ee16174faf84e539ab51bc12084
B-CANVAS_addendum_2.md         a5b11b94b23a3eaaabc69680ee03f8dcdb6377ddcce6ea5a44b08fa6b54bdc74
P1_SECOND_PROFILE.md          cd4650cb4a8c5d65b80bed8555b2349c1aa222938bb987aa09d1e696d7fd4626
PipeViewport.tsx              69565ede18bb93d0eb18fcff126a2c7543a2c4f17e31571a37e8d98e94be3ef8
viewportSelection.ts          fdf3eaa49b9685b932bce404421086c45a82fa22e5c8abc3f8146fc09cb846d5
viewportResource.ts           ecc235baa8867101b9ac913d37f3072f3493992393f1253ae65b50fc47b98153
uiDiagnostics.ts              9cee0e7c543cceaeaa9045ff91f74ff3f2fc2cbcaecf2eab6fb2fd958fecfca7
characterization-commands.ts   3bd12296b03a2faee580bf4ccba4ef0db1c1d8ddbd31e24fc32bc78e1e76b421
fixture-manifest.json         6e7fba8fdba11853ad7aa558c7e7c29ffb1a82e79c11a1b7633827335f458739
```

Design workflow first and optional agent assistance introduce no additional C4 dependency or Runtime work. Standard F-PIP-2/DEC-081 fence applies.

