# Sealed briefs — lane B-CANVAS

Run `HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION`, lane B-CANVAS (WORKING_ITEMS, Type 1, the canvas lane manager). Every child brief is written, hashed and listed here before its child is launched. Each row records the model requested, the role and the return. The model that actually ran is recorded with the return. Returns are retained verbatim under `../returns/` with SHA-256. Type 2 children do not delegate and run no state-changing git command; the manager commits on the lane branch. Mechanism for every child: Claude Code `Agent` tool, general-purpose type, background, in the lane's git worktree on branch `codex/swbpipe-b-canvas-20260918`. One writer per path at a time.

The lane's own instructions are ROOT's: `../../../briefs/B-CANVAS_canvas_lane.md` (`df89f5b622a63f206ccc0f879e7255642e4b9ee16174faf84e539ab51bc12084`) and `../../../lanes/B-CANVAS/briefs/B-CANVAS_addendum_1.md` (`4d5af33abf5b916ab3ddae09214e0d98070f6f2a984f0604d953e5381cd25ad5`), both verified by the manager with `shasum -a 256` at launch.

| Brief | SHA-256 | Sealed | Model requested | Role | Return |
|---|---|---|---|---|---|
| `C1A-PALETTE.md` | `e2c8fffe4957d8ce112a66d0a2449755cf77a499182ed51b1e59482d7d2db77b` | 2026-09-19T00:22Z | `fable` (Claude Fable 5.1) | TASK implementer; slice C1, first part: the palette module, matte figure shading, live repaint, every canvas colour but the two held | `../returns/C1A-PALETTE_RETURN.md` |
| `INV1-INSTRUMENT.md` | `4322fc25107c1c289be6670e1a880a610e1ef04b646fb38c0b580f3e269885fe` | 2026-09-19T00:24Z | `sonnet` (Claude Sonnet), `Explore` type | TASK, read-only inventory of what the benchmark instrument under `apps/desktop/e2e/ui-foundation/**` pins, for the second-profile proposal | `../returns/INV1-INSTRUMENT_RETURN.md` |

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
