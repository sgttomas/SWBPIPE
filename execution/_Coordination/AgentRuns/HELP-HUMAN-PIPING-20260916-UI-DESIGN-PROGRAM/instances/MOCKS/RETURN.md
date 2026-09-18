# RETURN — MOCKS-04, the frames after the owner's direction on the contradictions

Child: HELPS_HUMANS design manager, working alone (no delegation); the MOCKS-03 child resumed by the host's message-to-agent mechanism. Brief: `{RUN}/briefs/MOCKS-04_contradictions.md`, SHA-256 `c9631183ce65421f0926e5d8fd8e297457c460a699e02e2f3be80aa912b843a1`, checked before any work against the working file and against the committed bytes, and found to match. Model actually used: Claude Fable 5.1 (`claude-fable-5-1`). Writes only under `{RUN}/instances/MOCKS/`; no agent was spawned; no product build, test or dev server was run; no git command that changes state was run (`git rev-parse`, `git status`, `git log`, `git show`, `git diff`, `git cat-file` only). `MOCKS_V1.md`, `MOCKS_V2.md`, `MOCKS_V3.md`, `sample_model.md` and `frames/tokens.css` are byte-for-byte as committed.

## 1. What was read

1. The brief — whole.
2. `{RUN}/instances/ROOT/OWNER_DIRECTION_2026-09-18_CONTRADICTIONS.md` — whole, all three sections.
3. `DESIGN_SYSTEM_V1.md` at V1.3 — the status paragraph, §9 rows 90 to 111 whole, and the paragraphs those rows changed that bear on a frame: the toolbar band with the units selector (§5.2), the results tables and the run menu (§5.1), the issues drawer (§5.3), the footer's edit chip, the row expansion's caption, the marks table's tooltips (§4), the comment stream's Kind menu (§5.5), the tooltip rule (§7.6) and §8's list of what the frames must draw. `specimen.html` V1.3 — the markup of the edit chip, the run text button, the units control, the drawer's header with the Filter button and the Kind menu. `tools/agree.mjs` — its V1.3 checks.
4. `UX_SPEC_V1.md` at V1.2 — §13 rows 35 to 47 whole; §10.1, §10.3, §10.4 and the hanger table's state paragraph of §3.7.

The rest of both documents was not re-read in this pass; §5 says what that leaves uncertain.

## 2. What was produced

- `frames/` — eighteen frames. `s1_both_light` is new in name and view and `s1_table_light` is deleted with its shot (the rename is recorded in `MOCKS_V4.md` §1). `index.html` regenerated; `mocks.css` revised; `tokens.css` not regenerated in content (the build rewrote the same bytes), because `tokens.json` stays 1.2.
- **Byte-identical to the third pass: no frame.** Every frame inlines the revised stylesheet and carries the 80 px units control. Byte-identical: `frames/tokens.css`, `sample_model.md`. Changed by those two common items only: `s2_model_light`, `s2_model_dark`.
- `MOCKS_V4.md` — per frame what changed and its source; C-17 to C-25 with outcomes; Q-23 to Q-30 and G-13 to G-20 re-stated (closed by V1.3: Q-26, G-18, G-20); new: C-26, Q-31 to Q-33, G-21; four departures D4-1 to D4-4.
- `tools/` — `ui.mjs`, `frames.mjs`, `canvas.mjs` (a camera extent for an empty model; a node with no element drawn as a point), `build.mjs`, `render.mjs` (three new checks).
- `shots/` — all eighteen shots, `index.png` and `report.json` regenerated.

SHA-256:

```
995bdb62b3fb4ba14a893cb49f1301f041314a6584d0f77e717b4d6a18904417  s1_both_light.html
0e26169631fa2d92e39d1aef8c2f59fd863724007b854874566c9df2fa0cacd9  s2_model_dark.html
73af73c3fb5490aec8c9871dd6213b88bf3c9ec7f54cc731af6a882f67a5f238  s2_model_light.html
0fc7b447af94923a3e08718726dbfa1dc36f79ab2183cf7a92c6b5044c993706  s3_table_light.html
ea33eed6047565691f05b83e2959bb9db189007832fd8f0e9bc412faa69aa80e  s4_both_light.html
110093698a180a30e6ba5faf8858bc189efad3cc3aba6bec53f5511af5c5d9e4  s4_both_light_column.html
7e0952549dff20588d996df0c56e4526771678509a954ad47a858e80d8ba8eb8  s4_both_light_slideover.html
adf21ed176ed61028a297b5eb0dbfcecf48ae4c75378be4267088e0c9477ce7e  s4_table_light.html
91077b8efbac0d016b4169a621624c1d75d023b29913bac45c1dc73ba969217f  s5_table_light.html
6fc448d744f2f242c3fab2c33b6366882164f321aece9d2f7e6989943fd97385  s6_both_light.html
18e37f241eaa274f64d5679097e7912e2a07d5ca5e9d743ddde2bd98b5cc526e  s7_both_dark.html
07de12dcbba5cc41906d852da18797a151a426fadb5d7fb752a6f433ba310629  s7_both_light.html
54a9f7dc4b64a63bfc128693636d6c8cf4162643a47ca0ce907a62997ac64a0a  s7_both_light_historical.html
0582c62a594ef39ab6eae0ee00332bb7ff73201a5c09c8daad6a9e8f4e69f4f7  s7_table_light.html
0fa4512b4c9d09e1a5f351c6c36c56d928d0c78467a2dd94437bf43c2ae5e3f8  s8_model_light.html
4b5b4dc42e7e266a6bf702f97e805918310cba4dbb52131b77b519d23d1fded4  s8_table_light.html
a7ada63e9034a6eaf03687488430a9ebbdaa17a30b21f76c515eacf22f044178  s9_table_dark.html
9939bf5832f7366e19d8ca800eb9ef8e0db19c07752311e59067a0d5c8b0f072  s9_table_light.html
de2e25efc45dcf852e55869e63105d428afc23b2aed3991e5447eea7a64b2456  index.html
8da987bf7e43f86f1971979b826f0a372d03e34a99b999cbced82c653cc07ac2  tokens.css
f4fabfc898d7c840a6b5d1dbe1941439f8084ecf6fbbd4d58fc972339cb8a610  mocks.css
```

## 3. The lint and its result

The third pass's checks continue. Added: no tooltip writes a key outside parentheses (every `title` attribute, SVG `<title>` and drawn tooltip); no control's face carries a key (the hint strip and the palette field excepted); "Commit" is no control's text, tooltip or label, and is again a barred label while "Apply" is allowed; the bar-overflow check covers the Review page's filter row and the drawer's header; the chip expectation is 3 on the two Review frames and at most 2 elsewhere. The retired-string search reads every shipped file except the three history documents and `report.json`.

Final result: no findings in eighteen frames and the index; retired strings: none found; no network request, no external dependency, no absolute path; the inventory of frames, shots and index rows agrees.

Found on the way and fixed at the cause: "Apply" was itself on the barred-label list, which V1.3 changes; the new key-on-face check at first read SVG `<title>` text as part of a control's face and was corrected to read the face alone.

## 4. How the frames were verified

Every redrawn frame's screenshot was opened and looked at: `s1_both_light`, `s3_table_light`, `s4_table_light`, `s6_both_light`, `s7_both_light`, `s8_table_light`, `s9_table_dark`. What they showed, and was fixed: in `s1_both_light` the first canvas was fitted to a single node, so the grid was a few giant lines, the node was off the canvas and the scale reference spanned the pane; then the node was drawn as a selection ring and not as a point, the scale reference ran through the mock line, and the Run tooltip covered the HUD. In `s9_table_dark` "Kind: all" pushed the filter row past the 320 px column, cutting the combobox's chevron and the menu's counts. In `s6_both_light` the failed run's header had no run menu. The other frames change by a tooltip string, a caption or the common items and were checked through the lint's per-frame facts in `shots/report.json`.

## 5. What did not apply cleanly, and uncertainties

1. **The filter row (C-18).** "Kind: all" does not fit the 320 px column with DS's chip; the chips were tightened (D4-2) and the question of a chosen kind's longer label is Q-31.
2. **The footer's state counts (C-21)** meet the footer rule under which counts give way to the selection group; the state counts were kept (D4-4; Q-30).
3. **Brief item 8, the units selector's third choice:** no frame opens that menu, so only the control's 80 px width and its reading are drawn. The View tool's menu and the stress components (10, 11) are likewise drawn by no frame.
4. **The Connecting node column (8)** is still drawn in `s4_table_light`'s joined row; whether a reserved column is drawn is Q-32, and the frame was left as it was, since the brief does not list it.
5. **C-26:** the specification's §13 rows 36 and 37 cross their citations of C-18 and C-21. Reported, not resolved; no frame depends on it.
6. The empty model's camera extent is this pass's invention (D4-3; G-21).
7. Only the changed paragraphs of V1.3 and V1.2 were read; a change outside §9 rows 91 to 111 and §13 rows 35 to 47, if there is one, was not seen.
8. `s8_table_light`'s caption says "Stale" for a location that had a size selected before the model changed; the specification lists Selected and Stale as separate states and does not say which wins. Stale was chosen because the values are hatched.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
