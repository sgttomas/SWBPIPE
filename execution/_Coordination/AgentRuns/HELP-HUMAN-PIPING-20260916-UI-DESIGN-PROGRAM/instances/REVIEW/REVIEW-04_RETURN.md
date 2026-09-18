# REVIEW-04 — retained return

Retained by ROOT (HELP_HUMAN, successor) on 2026-09-18. What follows under "The child's return" is the reviewer's final message as the host stored it, with the machine's repository path replaced by `{REPO_ROOT}`; nothing else is changed. Brief: `{RUN}/briefs/REVIEW-04_contradictions_pass_review.md`, SHA-256 `440e6a386b1283ee6d68f6af508da1cd6cd56acd60dc789dff10fa1999d0e605`. Model that ran, by the child's own statement: Claude Opus 5. Candidate: `80f122300bff3101570beb96b001231350589cd4`.

## ROOT's dispositions

| Finding | Disposition |
|---|---|
| M-1 | Accepted. The owner ruled the form "name, then the key in parentheses" and nothing about where a content summary sits. The ordering clause is the specification child's addition. It goes back to that child to be logged in §13 as its own addition, not ruled. The two frame tooltips that put the control first ("Agent (⌘⇧G) · 0 open proposals · opens the column", "Node labels (L): Budget") satisfy the ruled form and are not redrawn in this pass; the difference is carried in the handoff's §7 as an open item for whoever next touches the frames. |
| T-1 | Accepted; ROOT adds `model_ran` to the MOCKS-04 node. |
| T-2 | Accepted; to the specification child. |
| T-3 | Accepted; to the design system child, as a new change-log row saying the owner's confirmation is of the six functions and that the copy stays the child's, open to replacement. |
| T-4 | Accepted; ROOT labels the reading in the direction record. |

## The child's return

### REVIEW-04 — return

Brief SHA-256 verified: `440e6a386b1283ee6d68f6af508da1cd6cd56acd60dc789dff10fa1999d0e605` (matches). Candidate reviewed: `80f122300bff3101570beb96b001231350589cd4` on `codex/swb-ui-contradictions`, against `origin/main`. The working tree equals the candidate for every file in scope; only `WORK_GRAPH.json`, `briefs/_INDEX.md` and this brief differ at `HEAD` (`e5c05d84d`), as the brief foresaw.

## Verdict: FINDINGS — no blocking, no major, one minor, four trivial

### Minor

**M-1. The specification states a tooltip-copy rule the design system does not carry, and two frame tooltips do not follow it.**
- `…/instances/UX-SPEC/UX_SPEC_V1.md:156` (§2.8) adds, in this pass: "a tooltip that also summarises content puts the summary first and the control with its key last."
- `…/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md:1043` (§7.6) and its §4 state only the ruled form ("no tooltip writes a bare key or puts the key first… the control's name and then its accelerator in parentheses"). Neither carries the ordering clause, and §13 row 45 (`UX_SPEC_V1.md:1072`) logs only the ruled form, so the sub-rule is a specification-only addition that no change-log row names.
- Two tooltips the pass regenerated break it: `Agent (⌘⇧G) · 0 open proposals · opens the column` (source `…/instances/MOCKS/tools/ui.mjs:155`; rendered in sixteen frames, e.g. `…/frames/s1_both_light.html:978`) and `Node labels (L): Budget` (source `…/instances/MOCKS/tools/ui.mjs:410`; every frame with a HUD). Both put the control and key first and the content summary after. The marks tooltips, by contrast, were changed to summary-first (`Restraint · +Y, gap 3 mm, μ 0.30 · Open the row (⌘↩)`), so the frames are internally inconsistent on the point. Not blocking: the ruled variant-A form itself is satisfied everywhere.

### Trivial

**T-1. `WORK_GRAPH.json:360`** — the `MOCKS-04` node omits `model_ran`, which `DESIGN-SYSTEM-04`, `UX-SPEC-03` and `MOCKS-03` all carry. `briefs/_INDEX.md` records it ("Claude Fable 5.1, the MOCKS-03 child resumed"), so no fact is lost.

**T-2. `UX_SPEC_V1.md:1076`** (§13 row 49) — "rows 36 and 38 keep the phrase as written", of the phrase "in revision". Row 38 carries it; row 36 does not (it reads only "The Kind menu's button reads 'Kind: all' with nothing chosen"). The body fix itself is correct: "in revision" survives at row 38 only.

**T-3. `DESIGN_SYSTEM_V1.md:1069`** (§8 item 19) and row 111 say the owner "has since confirmed" the six additions. The confirming message is timestamped 17:44:12.949Z, before the V1.3 text existed, so it confirms the six *functions*, not the child's copy for them. That provenance is recorded elsewhere (`…/instances/DESIGN-SYSTEM/RETURN.md` §8 item 4; handoff §7), but not at item 19 where the closure is asserted.

**T-4. `…/instances/ROOT/OWNER_DIRECTION_2026-09-18_CONTRADICTIONS.md:11`** — the record states as fact, unlabelled, that "your approach" and "the first 13 items" mean the thirteen ROOT had sorted, not items 1–13. The reading is sound (the message's next sentence treats 1, 2, 4 and 13 separately, and the effect table's "adopted" rows number exactly thirteen), but it is a reading and the record labels ROOT's other readings explicitly.

## What passed

- **Scope.** All 64 changed paths are under `{RUN}`. No product source, test, schema or root file changed.
- **The direction record.** All three owner messages reproduce their recorded SHA-256 and byte counts exactly from the verbatim quotes (429/44/72 bytes). "Apply" is marked ROOT's choice in the record, DS row 102, DS §8 item 20, spec §13 row 40 and handoff §7; the six additions' copy is marked the design system child's in its return §8 and handoff §7. I found no statement attributing to the owner anything the three messages do not say, beyond T-3/T-4.
- **The twenty-five items.** Each is carried: DS rows 91–111 and spec §13 rows 35–49 account for all of 1–16 and C-17–C-25 (items 3, 6, 7, C-18, C-19, C-21, C-22 correctly need no DS row). The three rulings are applied as ruled (15 A, C-20 A, C-23 B); C-22 opens in Both view; C-25 reads Run 03 everywhere (the four remaining "Run 02" mentions in the specimen are the failed run, as DS row 103 explains); item 5 leaves no control reading "Commit" in either document, the specimen or the frames. DS change-log rows 1–90 and spec §13 rows 1–34 are byte-identical to `origin/main` by diff.
- **Document agreement.** Checked pairwise on the six kept functions, the four rules, M-04's trigger, "Kind: all"/"All kinds", the hanger table, report readiness, the run menu, the stress components, the View menu and the drawer filters — they agree. All 26 distinct design-system section references in the specification resolve to real V1.3 headings. C-26 is recorded at spec §13 row 48 with rows 36 and 37 left as written, and the crossing is real and correctly diagnosed (C-18 is the Kind button, C-21 the State column).
- **The frames.** Case-insensitively: no "Commit", no "Run 3", no retired maturity or acceptance sentence, no machine path, no external URL, no "Approve". No `title` attribute writes a key outside parentheses; the paste band's faces read "Paste 3 rows" and "Cancel" with the keys in tooltips. `s1_table_light` is gone from frames, shots and index; `s1_both_light` exists in all three and draws Both view (screenshot read). Status-bar chips: three on both `s9` frames, at most two on every other (the historical frame's two labels are in the band popover, not the bar). `s8_table_light`'s hanger table has no State column; the state is in the footer counts and the expansion caption. All 21 hashes in `MOCKS/RETURN.md` §2 recomputed and match.
- **The handoff.** All 19 SHA-256 values in §1 recomputed and match. The diff touches only line 3 (status), §1 (lines 11–35) and §7 (96–105); §2–§6 and §8 are untouched. §3 carries ten constraints and constraint 5 retains "unless separately justified and authorized". §7's open items match `MOCKS_V4.md` §6–§7 exactly (Q-23–Q-25, Q-27–Q-33 open; G-13–G-17, G-19, G-21 open; Q-26, G-18, G-20 closed). It still says it starts and authorizes no implementation, and §8 asks one decision with stated alternatives.
- **Records.** The three sealed brief hashes match their files; model, mechanism and return are recorded for each; every recorded return hash matches the candidate file, with superseded hashes marked as superseded rather than rewritten. `WORK_GRAPH.json` parses, statuses agree with the index, and the prior frontier was moved verbatim into `frontier_history` (7 entries, none dropped). No authored file carries an absolute machine path; all 14 changed Markdown files end with the claim fence line.
- **Copy rules.** "Apply" is in the allowed column and "Commit" barred; no barred word appears as a control (the hits are barred-word lists, the registered "Review/signoff block", source identifiers, and a `.signoff` CSS class). Canadian spelling holds; the only US-spelled hits are quotations of source identifiers in an untouched §12 row.
- **Agreement check.** `node tools/agree.mjs DESIGN_SYSTEM_V1.md tokens.json specimen.html` reports `"problems": []`, 111 change-log rows, and hashes matching the candidate files.

## What I did not check

I did not open the other seventeen screenshots (only `s1_both_light.png`), and I verified no pixel measurement, wrap threshold, width or contrast ratio — I read the widths and the 737 px wrap only as prose claims. I did not re-render, re-build or re-lint the frames (`tools/render.mjs`, `tools/build.mjs`), and I did not diff the six `tools/*.mjs` files line by line beyond the tooltip and copy strings they emit. I did not re-verify the specification's source path-and-line citations against the product source, nor the operations map's rows. I did not read `MOCKS_V1`–`V3`, the earlier returns, the D-71/D-72 rulings or `DEC-099`–`DEC-105` beyond the quotations the candidate carries. I did not check the owner messages against any host transcript — I verified only that the quoted bytes reproduce the hashes the record states. I ran no build, test, dev server or state-changing git command; the only command I ran beyond reads was the brief's permitted `agree.mjs`.

I am Claude Opus 5 (1M context), model ID `claude-opus-5[1m]`, running as a TASK read-only reviewer, working alone, with no delegation.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
