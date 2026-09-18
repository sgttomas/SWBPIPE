# RETURN — MOCKS-03, the frames regenerated from design system V1.2 and UX specification V1.1

Child: HELPS_HUMANS design manager, working alone (no delegation). Brief: `{RUN}/briefs/MOCKS-03_regeneration.md`, SHA-256 `46343a8ce53ea3b6d6fba50ce46f384391d9c171f5100aefb29e30a2c6a7992b`, checked before any work and found to match. Model actually used: Claude Fable 5.1 (`claude-fable-5-1`), in one Claude Code agent session that was continued across context compactions. Writes only under `{RUN}/instances/MOCKS/`; no other file in the repository was modified; no agent was spawned; no product build, test or dev server was run; no git command that changes state was run (`git rev-parse`, `git status`, `git diff --stat` and `git grep` only). `MOCKS_V1.md` and `MOCKS_V2.md` are byte-for-byte as committed.

## 1. What was read

1. The brief — whole.
2. `{RUN}/../../_DECISIONS/D-71_RULING_2026-09-18.md` and its two addenda — whole.
3. `{RUN}/instances/ROOT/REVISION_PASS_PREPARATION_2026-09-18.md` — whole.
4. `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md` (V1.2) — §0, §1, §2.1, §2.3 to §2.8, §3, §4, §5 whole, §6, §7, §8, and §9 rows 48 to 89; §2.2's tables and §2.9 by grep only. `tokens.json` 1.2 — whole. `specimen.html` V1.2 — its icon sprite and the markup and rules of the V1.2 components. `tools/render.mjs` (how Playwright's location is taken at run time) and `tools/agree.mjs` (the retired-string list).
5. `{RUN}/instances/UX-SPEC/RETURN.md` — whole, including the sixteen contradictions of its §6. `UX_SPEC_V1.md` (V1.1) — §1, §2, §3.1, §3.6, §3.7, §4.3 to §4.7, §5.3 to §5.5, §6, §7 and §10; §3.2 to §3.5, §8, §9 and §11 to §13 by grep only, as the build needed a string or a width.
6. `{RUN}/instances/MOCKS/` — `MOCKS_V2.md`, `tools/` whole, `frames/mocks.css`, `sample_model.md`.

Nothing under `apps/` or any product source was read or written. The sections read by grep only are an uncertainty (§5).

## 2. What was produced

Under `{RUN}/instances/MOCKS/`, revised in place:

- `frames/` — eighteen self-contained frames: the sixteen design frames regenerated and two new (`s4_both_light_slideover`, `s7_both_light_historical`); the two decision-aid frames deleted with their screenshots, the retirement and its reason recorded in `MOCKS_V3.md` §1; `index.html`, `tokens.css` (from `tokens.json` 1.2, 99 colour tokens) and `mocks.css` regenerated or revised.
- `MOCKS_V3.md` — per frame what changed and its source; eight departures (D3-1 to D3-8); seven frame decisions (F3-1 to F3-7); nine contradictions beyond the sixteen (C-17 to C-25); eight questions (Q-23 to Q-30); eight gaps (G-13 to G-20).
- `tools/` — all six modules revised; `render.mjs` takes Playwright's location from `--playwright-from` or `PLAYWRIGHT_FROM` at run time and no machine path is written in any file.
- `sample_model.md` — one section added, "Hanger selection candidates (state 8)"; nothing else changed (`MOCKS_V3.md` §8).
- `shots/` — one 1:1 screenshot of every frame's stage (eighteen), `index.png`, and `report.json`.

SHA-256 of the frames and their shared files:

```
8ec686b82d597ae645efd74c2a2921500e7670c4d82a2936bb89d2739204571b  s1_table_light.html
d61f1fbd26b6bce38b9edbc55f99176b5efd9c13c8d096b121d3d20dabe26406  s2_model_dark.html
a8c94fc89541de67385f9ab1ddb8971a47e72e12e8c30d4be5f196881e172614  s2_model_light.html
439c14efb988bc0bd0163d647ef14044df3c036e4afb5e313bf4dc0bcb1f5ec4  s3_table_light.html
8f13d0969a183390b52eaffd366a0fe522e4e6bb6910681bac091f3ee0284cab  s4_both_light.html
9e37355094d40bc8b8501107c8ced26bc874d8f57e6a2ee562e20aac6b0ec8a8  s4_both_light_column.html
5acd7baca2c09a78727a3c3adce14f74cbe69d6cf821157d220ece9724c3c7f1  s4_both_light_slideover.html
f1e749cfc4cfc3c50836fb1a8f4a021befab06e0710cafbd467924dc920a93fd  s4_table_light.html
63e8c22ccd593ba48ccdaf91f1e4585139a478ac035170f1417697dc1c4f0bf9  s5_table_light.html
42b4062148e5a772fd3ca38f91135e56877f05791f1814b90b5208b4681780a1  s6_both_light.html
5bb35742ccfca10cb3bbbe7cee5c78f6c79dc60e964c8edb24478224a120de2d  s7_both_dark.html
a3034fb5eb7223b65d9819b6bec123e3cfa6871222903c0cbf4046b8e5391059  s7_both_light.html
cab293cd5db11d034918e0a8a77def3417a4b410bd4be942f5b458bfa89e382d  s7_both_light_historical.html
da68f2ea9ad1ab616b32b14fb9f4b5cfd586b7e79a15794e859e3964a561822a  s7_table_light.html
7bc3ff9a22f673fb7065cdd9d35c57b990e84f51c5820f4559940ef830849ad9  s8_model_light.html
f12c219115fd175cea23894afd31bd4a5509f30487a65210a765bdc27c797e00  s8_table_light.html
755d81f178a5b822673f1f09654a5a2cd6bafca37cbf89479d2caa9ae3e5f967  s9_table_dark.html
fca69134a56415caa432f8fe92e93f7b3b05cdbb030087631d9c96843ef19b5b  s9_table_light.html
ee3b4a078cb55175e7952ced1e23c444a18146e2964030dd105a5331e0f10ee9  index.html
8da987bf7e43f86f1971979b826f0a372d03e34a99b999cbced82c653cc07ac2  tokens.css
9445d34d7a3a735b9f51963972edd165e6b3a421d290a227e9281eed91e1f7d8  mocks.css
```

## 3. The lint and its result

`tools/render.mjs` loads every frame at 1440 × 900, at the stage's own size and at 960 × 700 with the network blocked, and reports a finding for any of: an element outside the stage; clipped text in cells, buttons, chips, bands, cards, toasts and popovers; a bar's child that runs past the bar's right edge; a region whose content is taller than the region; a font under 11 px; a forbidden word of the design brief's §6 list, visible or in an attribute; the word that asks the reader to press a key; a status or evidence label outside a chip, or a chip that differs from the `labels` table of `tokens.json`; an agent card whose class is not one of the five; the HUD at other than 304 × 34 or 154 × 64; a toast not 320 px or a run log not 360 px wide; a canvas under 220 px; an expectation per frame (the display-only caption, "draft until accepted", the boundary's short variant, the number of chips, the standing facts); a blocked request, a console message, an external dependency or an absolute path; a disagreement between the frame list, the files in `frames/`, the files in `shots/` and the index; and any retired string, searched case-insensitively over every shipped file except `MOCKS_V1.md`, `MOCKS_V2.md` and `report.json` (the list is built by concatenation so that the tool does not contain what it searches for).

Final result: no findings in eighteen frames and the index; retired strings searched in 30 files, none found; index links 23, none missing; no network request attempted by any frame; minimum font 11 px.

What the lint found on the way, each fixed at its cause: the paste band wider than its table (`s3`); the hanger candidates' reason cells clipped (`s8_table`); another vendor's product named in a source comment of `tools/model.mjs`; the Review page's comment column 24 px too tall (both `s9`); and, once the bar check was added, the results header's Run identity control cut off in the 737 px pane (both `s7_both`).

## 4. How the frames were verified

Every one of the eighteen screenshots and `index.png` was opened and looked at after rendering, and what they showed was fixed before returning: the Review header cutting "Export…" short, which the lint at that time had not caught (the bar check was added because of it); nested tables that had lost their cell padding, so that From and Type touched in the paste preview; the wrapped Run identity control sitting at the left; colliding restraint plates and a cut force reference in the 303 px canvas; a tooltip left pointing at a column outside the 574 px pane in the slide-over frame. Measured facts are in `shots/report.json`: fonts and sizes; no network request; no overflow; the docked frame at tables 737, canvas 303, inspector 300, strip 44; the slide-over frame at tables 574, canvas 470, slide-over 300, column 340; the HUD at 154 × 64 in the 303 px canvas and 304 × 34 elsewhere; the toast at 320 px; the run log at 360 px with its left edge at 793 against the Run button's 687; the scale steps' colours per theme (`scaleSteps`) and the edge-line choices (`edges`, `edgePlan`); the standing facts of the Current, Stale and Historical frames.

## 5. Uncertainties

1. The specification's §3.2 to §3.5, §8, §9 and §11 to §13 and DS §2.2 and §2.9 were read by grep and not in full; a string or a rule there that bears on a frame may have been missed, and more contradictions than the nine reported may exist.
2. C-17 to C-25 are reported as found from the sections read; C-19, C-20 and C-25 are as much inconsistencies inside one document or with the brief as between the two documents, and are listed so that ROOT can decide where they belong.
3. D3-1 (the results header wrapping to two lines in the 737 px pane) is a layout neither document draws; it was preferred to removing a required item. Q-26 asks for the rule.
4. Q-23: the selection group on read-only results tables follows the letter of the footer rule; ruling 7 may mean Check rows should not be there.
5. The Historical frame keeps the table's data bars in the result colour (Q-29).
6. The slide-over frame is drawn to DS §0's widths literally; the result hides the selected node and most of the HUD (Q-24). That is a finding about the rule and not a rendering defect.
7. The status bar's chips in the light theme have so little fill that they read as plain text; they are drawn from the tokens as given, and the dark theme shows the chip shape clearly.
8. The iteration menu no longer lists a "Current" working-state entry, which the second pass drew: DS §5.5 and spec §7.1 list the iterations and "Compare with…" only, while spec §7.4 says "Current" is the working state (G-19).
9. Fonts: the frames name the system UI and mono stacks; the rendering machine resolved them to its own system faces, so glyph widths on another machine may differ by a pixel or two from the measured boxes.
10. The frames remain composed moments (D3-7), and the canvas remains a schematic figure labelled as a mock rendering, not the engine's canvas.

## 6. Where the brief could not be satisfied, or only in part

- "The sixteen design frames": eighteen are delivered; the two additions and why are in `MOCKS_V3.md` §1. The sixteen are all present.
- "`sample_model.md` unchanged unless a state needs a value": one section added for hanger selection; listed in `MOCKS_V3.md` §8.
- Nothing else is known to be unmet.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
