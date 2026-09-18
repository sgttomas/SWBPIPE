# RETURN — DESIGN-SYSTEM-03 · Design system V1.2

Instance: DESIGN-SYSTEM (HELPS_HUMANS design manager, child of ROOT). Brief:
`../../briefs/DESIGN-SYSTEM-03_revision.md`, sealed. Date: 2026-09-18. For ROOT's
acceptance. This file replaces the DESIGN-SYSTEM-02 return.

## 1 What was read

The sealed brief; ROOT's preparation record (`../ROOT/REVISION_PASS_PREPARATION_2026-09-18.md`);
`../MOCKS/MOCKS_V2.md` with its frames and tools; decision packet D-71; `../UX-SPEC/UX_SPEC_V1.md`
(§2.6, §3.7, §5.4 to §5.5, §6.3, §7.5 to §7.7, §8, §10.3 to §10.9); the claims registry's
entries for the acceptance boundary, the content boundary and the evidence labels; the head of
the product's historical-run context component, for the rendered historical wording; the
rendering brief; the implementation-handoff preparation; the brief index; and this
instance's own V1.1 files. Wider consultation: none of another role's instructions.

After the brief was sealed, ROOT sent one message relaying two later directions of the owner.
I applied both on ROOT's word, under the same rules, each with its own change-log row:

- the maturity sentence is removed from the product entirely (supersedes ruling 2);
- the Checked mark is ruled, D-71 item 7 option A, with its exact words (supersedes ruling 7).

## 2 What changed

All in place under this directory; nothing outside it was written.

| File | Change | sha256 |
|---|---|---|
| `DESIGN_SYSTEM_V1.md` | V1.2. Every change is a numbered row of §9, rows 50 to 87, and rows 88 and 89 for correction 1; rows 1 to 49 are byte-identical to V1.1 and rows 1 to 87 to V1.2 as first returned (both checked by hash). The §9 heading now covers V1 to V1.1 and V1.1 to V1.2. Ends with the fence line. | `1a7ffe40409889d78105e05d5f8849e02bbb0209c0aa1b0eaceae1108b215bf5` (after correction 1; as first returned `2e0db6bc…`) |
| `tokens.json` | 1.2. Adds `canvas.edgeAlt`, the toast, HUD and run log values, the `labels` table (eight rows) and `agentCardClasses` (five words); the name is the product's name alone. Every colour value of 1.1 is unchanged (diffed). | `00c1afe97365973e6c05a2a5d938a25c46022a55c3cbc8d04ff3a9d2d68cf1e9` |
| `specimen.html` | V1.2, one self-contained file, inline styles and script, no network reference, light and dark. | `219fa8ee78c44507fcfc0b6a199298c8f4a4026abe798f4166eeda2eb607941b` (after correction 1; as first returned `c390e7e6…`) |
| `tools/palette.mjs` | the new tokens and groups; `--write` reproduces `tokens.json` byte for byte (checked) | `de73ecb9…` |
| `tools/contrast.mjs` | six pairings for the edge lines on the result scale: 152 pairings | `7a3f2653…` |
| `tools/gen.mjs`, `tools/splice.mjs` | generate the label table and splice it into §2.3 between markers | `f936d18a…`, `34e986a1…` |
| `tools/agree.mjs` | V1.2 checks, §3 below; after correction 1 also rows 88 and 89 present, rows 1 to 87 unchanged by hash, and the stale band's removed clause absent | `d69f7c0f…` |
| `tools/render.mjs` | V1.2 checks; takes a relative specimen path; takes the Playwright location at run time (`--playwright-from <dir>` or `PLAYWRIGHT_FROM`), default the relative path to the piping project as before | `b886e708…` |

No file in the instance carries an absolute machine path; `agree.mjs` now walks the
instance and fails if one appears. The retired strings are built by concatenation inside
the two check scripts, so the tools directory does not carry them either.

Trace of the brief to §9 rows: ruling 1 → 50; ruling 2 (as superseded) → 51; ruling 3 → 52,
77; ruling 4 → 53, 54; ruling 5 → 55; ruling 6 → 56, 57; ruling 7 (as superseded) → 58;
ruling 8 → 59; ruling 9 → 60; Q-20 and the pointer rule → 61 to 67; Q-15 → 68; Q-16 → 69;
Q-17 → 70; Q-18 → 71; Q-19 → 72; Q-21 → 73; Q-22 → 74; G-7 → 75; G-8 → 76; G-9 (closed
unused) → 77; G-10 → 78; G-11 → 79; G-12 → 80; R-4 → 81; R-5 → 82; R-6 → 83; the
consequence line (MOCKS_V2 D2-1) → 84; §8 → 85; tokens and tools → 86; specimen → 87.

What the status bar's information popover now holds: nothing, and it goes. In V1.1 it
held two lines, the About link and the maturity sentence. With the sentence removed a
popover with one link is not worth a click, so the control (an information glyph,
tooltip "About SWBPIPE…") opens About directly. About carries the name, version and build,
then its tabs; the line the sentence held is closed. The M-01 and M-02 homes are removed
from §7.4.

## 3 What was verified, and how

Model: Claude Fable 5.1 (`claude-fable-5-1`), one agent, no delegation. One context
compaction occurred mid-run; work resumed from the files on disk and a transcript summary,
and every check below was run after the last edit. Edits to the document and specimen were
anchored replacements, each anchor required to occur exactly once.

**Agreement check** — `node tools/gen.mjs tokens.json <out>`, `node tools/splice.mjs <out>
specimen.html DESIGN_SYSTEM_V1.md`, `node tools/agree.mjs DESIGN_SYSTEM_V1.md tokens.json
specimen.html`: `problems: []`. 99 colour tokens, 98 plain tokens, 99 document colour rows,
152 contrast rows and pairs, 8 labels, 25 label chips in the specimen, 5 card classes, 87
change-log rows. The check covers: the colour tables, CSS blocks, embedded token JSON and
pair list against `tokens.json`; the §2.3 label table against a fresh generation; every
specimen label chip (class, token, domain, label) against the table, no label drawn
outside the chip markup, and the two unhyphenated forms nowhere; agent cards carrying one
of the five class words, every class shown, and agent text free of the five barred words;
the retired strings absent, case-insensitively, from the document, the token file and the
specimen (the longer name, the old name, the other vendor's product name, both halves of
the maturity sentence, three fragments of the acceptance sentence); change-log rows
contiguous, rows 1 to 49 unchanged by hash, and rows 50 on citing rulings 1 to 9, Q-15 to
Q-22, G-7 to G-12 and R-4 to R-6; every sprite reference resolving; no external reference;
the document ending with the fence line; no machine path in any instance file. I ran the
check once against a copy with sixteen faults injected and it reported each.

**Separate searches** (Python, case-insensitive, over the three deliverables): the same
retired strings and "user rules checked" / "user rule failed": no match. US spellings in
prose: none (the font licence's proper name aside).

**Render** — `node tools/render.mjs specimen.html <out> --playwright-from <dir>` with
Playwright 1.60 and its Chromium taken from the main checkout's piping project at run time,
because this worktree has no `node_modules`: 1440 and 720 wide, light and dark by system
preference and by the manual switch. In all four: no blocked request (none attempted), no
console error or warning, no horizontal overflow, smallest font 11 px, 99 token rows, 152
contrast rows, 8 label-table rows, the 25 chips redrawn from the table and equal to it,
wrapped HUD 154 by 64 with ten tools and Fit first, toast 320 px, the three Review header
buttons with icons and none accent, no retired string in the rendered text. I read the
screenshots of sections 5, 8, 9, 10 and 11 and fixed what they showed (a clipped button, a
wrapped toolbar, the wrapped HUD's size, which I had stated as 156 by 62 and is 154 by 64).

**Not run:** the dataviz palette validator was not found locally; the result ramp and the
categorical set are byte-identical to 1.1, where it was run, and the built-in ordinal check
passes in both themes. No product build or test was run, by instruction. `git status`
(read-only) shows changes in this instance directory only.

## 4 The key-only actions of V1.1 and their pointer controls

The rule is stated at the head of §5 and, for copy, in §7.6. The audit table is in §5; in
summary ("found" means the control existed in V1.1 and is now named):

| Action (V1.1 key) | Primary control in V1.2 |
|---|---|
| Open / close the Both-view inspector (⌘I, double-click; Escape) | the toolbar's Inspector toggle; the close control in the inspector's header |
| Open / collapse the agent column (⌘⇧G) | the Agent toggle (found); a click on the strip; the column header's collapse control |
| Fit (F) | the Fit tool (found), now first in the HUD at every canvas width |
| The other HUD tools (I H L D P R S) | the HUD tools (found) |
| Show what is hidden (none stated) | the HUD's count button, "3 hidden · Show all" |
| Routing: axis (⇥, arrows) | a click on an axis handle |
| Routing: reverse (−) | the Reverse control beside the length field; a click on the opposite stub |
| Routing: place (↩) | the Place control; "Place node 50" in the routing block |
| Routing: cancel (⎋) | the Cancel control beside the length field; the routing block's Cancel (found) |
| Routing: bend (B) | the routing block's "Bend at 40…" (found) |
| Unpin the probe (Escape) | the pinned card's close control |
| Start / commit / cancel a cell edit (↩, ⇥, ⎋) | a click on the focused cell; a click on another cell or the footer edit chip's Commit; the edit chip's Cancel |
| Open a row expansion (⌘↩; Space) | the expansion chevron in the owning cell |
| Open a joined row (⌘↩) | the marks slot and the faint plus (found) |
| Close an expansion (⎋, ⌘↩) | the caption line's chevron; its close control |
| Insert a row (⌥↩) | the footer's Insert row below; the "Add row" line |
| Delete, copy, paste rows (⌫, ⌘C, ⌘V) | the footer's selection group |
| Select all rows (⌘A) | the gutter's header cell |
| Clear the selection (⎋) | the clear control on the footer's selected count |
| Check rows, clear the check (⌘⇧K) | the faint check in an empty state slot; the footer's Check rows; the Checked popover's buttons; the inspector's control (found) |
| Reveal a mark (Space, ⌥I) | a click on the glyph; "All marks on this row" in its popover |
| Undo, redo (⌘Z, ⇧⌘Z) | the toolbar's Undo and Redo |
| Send to the agent (⌘↩) | the Send control |
| Close the issues drawer; collapse the table drawer (⎋) | the drawer's close control; the collapse chevron on the tab strip |
| Close a dialog (Escape) | every dialog's Cancel or Close button |
| Close a popover, the run log, a menu (⎋) | a click outside (found) |
| Dismiss a toast (none stated) | the toast's close control |
| View switch, palette, Issues, sort, Accept and Reject, paste band, combination editor, Run and Stop | all found, unchanged |

## 5 Uncertainties, and what in the rulings did not apply cleanly

1. **The domain word for the two evidence labels.** Ruling 4 requires the authority domain
   with every label; the governed domains are the statuses'. I used "Evidence"; it is one
   field in `tokens.json` and is the owner's to confirm (§8 item 14).
2. **Stale is treated on Historical terms.** Ruling 6 and R-9 bar a Historical record from
   a current-model overlay. After a model change the run is no longer current, so V1.2
   gives a Stale run no overlay, chip or evidence chip either. UX_SPEC_V1 §2.6 and §10.4
   say otherwise; the conservative reading is taken and flagged for UX-SPEC-02 (§8 item 16).
3. **Settled by ROOT (correction 1, §6 below).** As first returned: two sentences about the same run disagree across documents: the stale band keeps
   decision 4's "stays the solve basis", the specification's consequence line says the
   run stops being it. V1.2's consequence line avoids the phrase (§8 item 17).
4. **Q-16's words are geometrically inconsistent** at 1440 ("right-aligned under the Run
   button" and "over the canvas"); I specified by the stated purpose (§8 item 18).
5. **Q-22 applied as written:** the Kind menu has no entry for the engineer's notes; Mine
   reaches them (§8 item 15).
6. **The fitted camera** (Q-20) is one new bit of interface state, a semantic addition and
   not a restyling (§8 item 12). **`canvas.edgeAlt`** asks the engine for a per-element
   edge colour; feasibility is for the rendering observation (§8 item 13).
7. **Ruling 8 and the product's own strings.** "draft until accepted" and "accepted 16:31 ·
   Undo" remain on proposal cards as product text, not agent text; I reworded two sample
   agent sentences in the specimen that used a barred word. If the owner means the barred
   words to leave the card's product strings as well, that is a further change.
8. **"Acceptance not recorded in this session"** (§5.4, reopened records; C-68) is kept: it
   is about proposal acceptance, not the removed sentence.
9. **Toast message length.** Beside an action and the close control a 320 px toast leaves
   about 190 px for the message, so I allowed three lines, not two.
10. **Settled: ROOT named the owner records, now cited in §9 row 89.** As first returned: the two later rulings were applied on ROOT's relayed word; I did not see an owner
    record for them beyond ROOT's message and ROOT's scratch copy of the ruling.
11. The validator was not re-run (§3). The 1440 and 720 renders are Chromium only.

Nothing in the brief was left unapplied. Rulings 2 and 7 are applied in their superseding
form, as ROOT directed.

## 6 Correction 1, after ROOT's review of this return

ROOT accepted the return's checks and decided the uncertainties: 1, 2, 4, 5, 7, 8 and 9
stand as written; 10 is answered by the two owner records; 3 is settled as follows. After
a model change the run stops being the current solve basis (the product clears the solve
proof and the model hash when a model commit lands), so the stale band's closing clause,
which said the run remained the basis until the next run, was wrong.

- Removed from the stale band in §5.1, its row in the §4 placements table, the vocabulary
  of §7.1 and the specimen's band. The band reads "Model changed since Run 03: *the change,
  with its time*. The values below are from the model as solved." with "Run again"; the
  compact form reads "Model changed since Run 03 · *change* · Run again".
- §8 item 17 is closed by ROOT with that wording; item 16 says ROOT confirmed the Stale
  column.
- §9 row 88 records the correction; row 89 cites the owner records for rows 51 and 58
  (`_DECISIONS/D-71_RULING_ADDENDUM_2_2026-09-18.md` for the maturity sentence,
  `_DECISIONS/D-71_RULING_ADDENDUM_2026-09-18.md` for the Checked mark).
- `tools/agree.mjs` now requires rows 88 and 89, checks rows 1 to 87 by hash against V1.2
  as first returned, and fails if the removed clause appears in the document, the token
  file or the specimen (tested with an injected copy; it fails).
- Re-run after the correction: gen, splice, agree (`problems: []`, 89 change-log rows) and
  the render check (1440 and 720, light and dark: no request, no console issue, no
  overflow, chips equal to the table). `tokens.json` is unchanged.

One thing did not apply cleanly: ROOT asked for the owner records to be cited in rows 51
and 58, and also for rows 1 to 87 to stay byte-identical. Both cannot hold, so rows 51 and
58 are untouched and row 89 carries the citations for them.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
