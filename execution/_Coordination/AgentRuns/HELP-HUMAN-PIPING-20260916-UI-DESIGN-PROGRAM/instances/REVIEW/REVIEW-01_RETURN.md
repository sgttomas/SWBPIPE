# REVIEW-01 — return (first pass)

Retained by ROOT on 2026-09-18 from the reviewer's final message, which is the return under the brief [`../../briefs/REVIEW-01_successor_records_review.md`](../../briefs/REVIEW-01_successor_records_review.md). This is an in-session transcription of that message by ROOT, not the child's own file: the reviewer had no write target. Long run-relative paths the reviewer abbreviated with `...` are left as written. The backcheck of the corrected candidate is appended below when it returns.

## Verdict: FINDINGS

Reviewed SHA: `cd3dba125da109f1cc50314874725c154296ef0a` (branch `codex/swb-ui-design-handoff-reconciliation`; 16 files, +1256/−10 against `origin/main` at `451c5f595e0488a0d6064d9b0f972c5e8fc1d09d`).

| # | Check | Result |
|---|---|---|
| 1 | No invented ruling | PASS |
| 2 | Facts match their sources | FINDING 1 (else PASS) |
| 3 | Attribution kept apart | PASS |
| 4 | History preserved | PASS |
| 5 | No circular prerequisite | PASS |
| 6 | Hygiene | PASS (with FINDING 3, 4) |
| 7 | Delegation record | PASS on all seven required fields; FINDING 1, 2 in the same section's acceptance note |

## Findings, by severity

1. **Major — the accepted row and class counts for RESEARCH-G do not match the retained return.** Three records state "38 rows: 10 observable now, 7 needing a bounded harness adaptation, 21 needing implementation first": the observation brief line 3, `briefs/_INDEX.md:207` (which adds the breakdown "24 for the rendering brief's §2 …, 6 for §3, 8 for §4") and `WORK_GRAPH.json:226`. Evidence: the retained return (hash `fcf6eb88…`, matching the index) carries 50 classified rows, 34 in §1.1, 8 in §1.2, 8 in §1.3, and the classes are 10 / 8 / 32. Class 2 is eight rows: 2.6e, 2.8a, 2.9b, 3.1b, 3.2b, 3.4a, 4.5, 4.6. Only "10 observable now" is correct. The "7" appears to be the count of observations O-1 to O-7 in the observation brief, not a row count. The index line is the acceptance statement for the return and accepts coverage the retained bytes do not have. The classification's substance is unaffected: every one of the 50 rows carries exactly one class, the observation brief asks only for class 1 and 2 behaviour, and the "six of eight §4 observations" statement in the brief and its notice is correct.
2. **Minor — `briefs/_INDEX.md:207`, "class 3 rows show their searches" overstates the return.** 12 of the 32 class 3 rows carry a search; the other 20 argue absence from cited code or by cross-reference to another row.
3. **Minor — dangling reference.** `briefs/_INDEX.md:215` and `:219` point to this file's path, which did not exist at the reviewed SHA.
4. **Trivial — spelling.** RESEARCH-G line 168 uses "normalized" in prose, echoing the product identifier `normalized_display_offset_not_physical_length`; possibly deliberate.

## What passed, with the evidence recomputed

- Hashes: all six in the reconciliation record §1 and all six in D-72's bound evidence recompute at `451c5f595`; the brief and return hashes in the index recompute; the verbatim owner message (activation record lines 19 to 295, no trailing newline) is exactly 18,684 bytes hashing to `d1c985e0ad17a771fd42dd418f0844d5038acf3dbf56a8565d82695eed4ac808`.
- PRs and SHAs: #789, #793, #794 and #795 exist with the stated merge SHAs and are ancestors of `451c5f595`.
- Arithmetic: 603 × 828 = 499,284; 1000 × 828 = 828,000; 794 × 557 = 442,258; label caps 138 and 230; 243 segments; 20 candidate and 10 baseline runs. Every metric in the reconciliation table matches the runtime report lines 11 to 16; the cohort account matches the baseline report; the operations map carries 277 rows and 29 gaps, and the five gap classes in the handoff preparation §3 partition G-01 to G-29 exactly.
- Source spot-checks: the boundary validator (`apps/desktop/e2e/ui-foundation/characterization-commands.ts:50-53`), the selection recolour (`viewportResource.ts:981-985`) and the pixel-ratio cap (`:407`) are as cited.
- No invented ruling: both register rows read AWAITING_RULING; every "owner approved" in the changed files is earlier context about the fourteen mock-review questions or is conditional.
- History: the D-71 packet is byte-identical for lines 1 to 115 with the note inserted above the fence line; the working state strikes rather than deletes; the rewritten D-71 register row quotes what it formerly read.
- Attribution: no speedup or same-profile scaling statement; trace spans, presentation feedback and boundary snapshots are distinguished; D-72 item 1 adds no CPU gate and subtracts no observer overhead.
- Hygiene: no absolute machine path in authored text; all relative links in the 15 changed Markdown files resolve; no barred word used as a product control; every governed record ends with the standard claim fence; the work graph parses with 27 nodes, 36 edges, no missing node and no orphan.

## Not checked

Original transport bytes of the owner message; 44 of RESEARCH-G's 50 rows against product source, the two absence searches and the memory-unavailable citation; any build, test, benchmark or CI state, and GitHub state beyond local ancestry; the design system, specification and frames beyond the values D-72 cites; prose quality beyond a pattern sweep; files outside the diff.

## Backcheck of the corrected candidate

Retained by ROOT on 2026-09-18 from the same reviewer's second final message; in-session transcription, condensed to its verdict, results and findings.

Reviewed SHA: `202651a09ce87c8d87332f83f2d6e78595ca07db` (one commit over `cd3dba125`, 4 files, +48/−3). Verdict: FINDINGS, one trivial; all four backcheck items PASS.

1. Finding 1 corrected: 50 rows and 10 / 8 / 32, with 34 / 8 / 8 by section, in the observation brief, the work graph and the brief index; no surviving live claim of 38, 7 or 21 anywhere in the branch diff; the correction and its cause are recorded in the index; the retained RESEARCH-G file is untouched and still hashes to `fcf6eb88bb43dbc28d32156b3766cf5a388dad0b96316ca8623c470dc0d2cc45`.
2. Finding 2 corrected: the index now says 12 class 3 rows show a search and 20 do not, and states ROOT's acceptance judgment.
3. Finding 3 resolved: this file exists, its link resolves, and the transcription is faithful in verdict, findings, order and severities. Three condensations named by the reviewer, none altering a claim: finding 3 no longer names the two index lines and the absent directory; finding 4 drops the supporting detail that RESEARCH-G spells "colour", "recognisable" and "centreline" elsewhere; the reviewer's separate not-checked item, that it did not verify the owner in fact sent the message recorded in the activation record, is folded into "original transport bytes".
4. Nothing else changed in the commit; the earlier PASS results hold for the new lines; hygiene holds; the work graph parses with 27 nodes and 36 edges.

Trivial finding, the reviewer's own error from its first return, transcribed faithfully above: "all six in the reconciliation record §1" should read five. That table has six rows and five SHA-256 values; the picking-repair row claims no hash. All five recompute. The first-pass text above is left as returned.

Scope note from the reviewer: "every hash recomputed" means every hash cited in the reconciliation record, D-72 and the observation brief, and the brief index's three new hashes; hashes in the index's earlier sections were outside its scope. Not checked in the backcheck: the rest of the branch beyond targeted sweeps, which rests on the first pass; RESEARCH-G's rows against source beyond the first pass's three citations; byte-faithfulness of this transcription by independent custody; any build, test, CI or GitHub state.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
