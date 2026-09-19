# Sealed briefs — lane B-CANVAS

Run `HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION`, lane B-CANVAS (WORKING_ITEMS, Type 1, the canvas lane manager). Every child brief is written, hashed and listed here before its child is launched. Each row records the model requested, the role and the return. The model that actually ran is recorded with the return. Returns are retained verbatim under `../returns/` with SHA-256. Type 2 children do not delegate and run no state-changing git command; the manager commits on the lane branch. Mechanism for every child: Claude Code `Agent` tool, general-purpose type, background, in the lane's git worktree on branch `codex/swbpipe-b-canvas-20260918`. One writer per path at a time.

The lane's own instructions are ROOT's: `../../../briefs/B-CANVAS_canvas_lane.md` (`df89f5b622a63f206ccc0f879e7255642e4b9ee16174faf84e539ab51bc12084`) and `../../../lanes/B-CANVAS/briefs/B-CANVAS_addendum_1.md` (`4d5af33abf5b916ab3ddae09214e0d98070f6f2a984f0604d953e5381cd25ad5`), both verified by the manager with `shasum -a 256` at launch.

| Brief | SHA-256 | Sealed | Model requested | Role | Return |
|---|---|---|---|---|---|
| `C1A-PALETTE.md` | `e2c8fffe4957d8ce112a66d0a2449755cf77a499182ed51b1e59482d7d2db77b` | 2026-09-19T00:22Z | `fable` (Claude Fable 5.1) | TASK implementer; slice C1, first part: the palette module, matte figure shading, live repaint, every canvas colour but the two held | `../returns/C1A-PALETTE_RETURN.md` |
| `INV1-INSTRUMENT.md` | `4322fc25107c1c289be6670e1a880a610e1ef04b646fb38c0b580f3e269885fe` | 2026-09-19T00:24Z | `sonnet` (Claude Sonnet), `Explore` type | TASK, read-only inventory of what the benchmark instrument under `apps/desktop/e2e/ui-foundation/**` pins, for the second-profile proposal | `../returns/INV1-INSTRUMENT_RETURN.md` |

## Returns retained

Host routing, as ROOT confirmed it on 2026-09-19: a nested child's completion notification goes to ROOT's session and not to this manager. ROOT extracts the child's final message verbatim to a file, hashes it and relays the path, the SHA-256 and a short digest. The manager verifies the hash with `shasum -a 256` and copies the bytes unchanged into `../returns/`. ROOT's digest is a pointer only; the retained file is the return.

| Return | SHA-256 | Model that ran | Notes |
|---|---|---|---|
| `../returns/INV1-INSTRUMENT_RETURN.md` | `c2279008809c16f032ff51b337cd70f45d833f3404c9f564e59c4aef8e25cc1b` | Claude Sonnet 5 (`claude-sonnet-5`), as the return itself names | Relayed by ROOT and verified 2026-09-19. Read-only: the child wrote nothing. Its own stated limits: about 30 of roughly 60 hash literals are not traced to a binding; three large spec files and one verifier were grep-sampled and not read whole. The manager re-read the lines it relies on before citing them in `../proposals/`, and found one error: the two checks that require exactly 34 method files are in `characterization-observations.mjs` (lines 196 and 278), not in `resource-accounting.ts` as the return says. The retained return is verbatim and is not edited; `../proposals/P1_SECOND_PROFILE.md` §8 carries the correction. |
| `../returns/C1A-PALETTE_RETURN.md` | `01f6199ccb97a9e94f33fcf2324c775cbf7a0ccd0ddb47ac7f5d878e6c3108cc` | Claude Fable 5.1 (`claude-fable-5-1`), as the transcript's final message records and the return itself names | 34,864 bytes; the child's final message of 2026-09-19T01:20:20Z. This completion notification reached the manager directly, because the manager was mid-turn when the child finished, so ROOT's relay was not needed. The notification text was entity-escaped, so the manager did not retain it; it extracted the final message byte-for-byte from the host's task record by script, printing only its size and hash, and did not read the transcript. If ROOT relays its own extraction and the hashes differ, ROOT's file governs. |

## Proposals returned to ROOT

| Proposal | SHA-256 at first commit | Subject |
|---|---|---|
| `../proposals/P1_SECOND_PROFILE.md` | recorded in the slice return | The benchmark instrument's second profile (D-72): form, bound values, visual tokens and the halo rule, bindings, order. Asked for by addendum 1. |
| `../proposals/P2_EDGE_LINE_SLICE.md` | recorded in the slice return | A slice, C1E, for the design's edge line, which no slice of the brief draws. |

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
