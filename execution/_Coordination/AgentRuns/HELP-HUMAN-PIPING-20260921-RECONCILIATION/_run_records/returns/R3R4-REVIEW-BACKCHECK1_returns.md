# R3R4 review backcheck 1 returns (summary; new findings quoted verbatim; full texts in the session transcript)

All seven reviewers were resumed with the fix HEAD `a1f31adf0`. Verdicts:

| Reviewer | Agent | Verdict | New findings |
|---|---|---|---|
| RV1 | a9987925af4b0ce54 | FINDINGS | 1 ACTIONABLE (U8 row unblocked; U4 not on C6) |
| RV2 | a81f236a9717a9972 | PASS | none |
| RV3 | a76621eef6ae365d9 | PASS | 1 MINOR (B10 §9 blocks list incomplete) |
| RV4 | a87f5b276fa0b7cf2 | FINDINGS | N1 ACTIONABLE (U8), N2 MINOR (U4), N3 MINOR (U6 key missing in gate index) |
| RV5 | a06f2d0e2f4bc09df | PASS | none (note: ER-35 cites `ScopeOfWork.md:183` without folder, carried from T1) |
| RV6 | a9fda342d3de6b36f | FINDINGS | 2 MINOR (same U8 and U6 items) |
| RV7 | abaad7a6adbe1c827 | PASS | none |

Every original finding (RV1 1–5, RV2 1–9, RV3 1–11, RV4 1–15, RV5 1–6,
RV6 1–9, RV7 1–17) was reported FIXED.

Agent 0 fixes: commit bf14bb60b (B10 §9 as a filter) and 20c0822f5 (U8 row
`DEL-11-03:SOW#CLM-004.r01` blocked on C4 with counts 156 / C4 3; ER-02
gains `C6 (partial)` for U4; gate index U6 cell lists the four keys).

## RV1 (new finding, verbatim)

1. **ACTIONABLE** — `R4/R4_GATE_INDEX.md` §4, the intro paragraph and rows U4 and U8.
   - **The problem:** the intro says "Every row behind a U-item is marked blocked on its host packet in the handoffs, so none of them counts as ready." Two rows break this.
   - **U8:** `DEL-11-03:SOW#CLM-004.r01` is named in the U8 row and in C4 §5 (lines 154–155). But it has an empty `BlockedOnPacket` in `R3/CODE_FIX_BRIEF_CANDIDATES/CODE_FIX_ROWS.csv`, and the table in `CFB-28_user-docs.md:22` shows "—" for it. So it currently counts as ready.
   - **U4:** `DEL-12-02:SOW#CLM-037/REXC-OI-002` is carried only through H3 ER-02. ER-02's `BlockedOnPacket` is "C7 (partial);A5 (partial);A3 (partial)", with no C6, and the key has no row-level mark in any handoff.
   - **Smallest fix:** U8: add `C4` to the CLM-004.r01 row in `CODE_FIX_ROWS.csv` and in CFB-28's table. U4: add `C6 (partial)` to ER-02's `BlockedOnPacket`, or name the key in ER-02's text as blocked on C6. Alternatively, soften the intro sentence.

## RV3 (new finding, verbatim)

1. **B10_plugin-adapter-runtime-and-grants.md:172-180 — MINOR.** §9 "Blocks" names some H2 rows marked B10 but not others (omits `DEL-17-09:STATUS#remaining/R01`, CFB-06 `DEL-02-04:SOW#CLM-014/REQ-13` and `CLM-015.r03`, CFB-14 `DEL-06-04:SOW#CLM-010/R-06-04-012`, CFB-22 `DEL-08-05:SOW#CLM-011/DEL-08-05-REQ-012`, CFB-29 `DEL-12-02:SOW#CLM-012/REXC-REQ-014`). **Smallest fix:** name them, or replace the named rows with "every H2 row whose BlockedOnPacket includes B10 (filter on `CODE_FIX_ROWS.csv`)".

## RV4 (new findings, verbatim)

- **N1. ACTIONABLE.** One U8 row is not blocked. `CODE_FIX_ROWS.csv:181` is `DEL-11-03:SOW#CLM-004.r01` (CFB-28), and its `BlockedOnPacket` is empty. Gate index §4 (line 108) and C4 (line 154) both name it as a U8 row. Smallest fix: add `C4` to row 181 and refresh CFB-28's table. The readiness figures don't change, because CFB-28 is already partly blocked.
- **N2. MINOR.** The U4 row is not blocked on its host packet. ER-02's `BlockedOnPacket` is `C7 (partial);A5 (partial);A3 (partial)`, with no C6. Fix: add `C6 (partial)` to ER-02, or qualify the §4 sentence.
- **N3. MINOR.** One U6 key is missing from the gate index. The U6 cell in §4 (line 106) lists three rows. C1 (line 268) and H2 also include `DEL-06-04:SOW#CLM-010/R-06-04-012`, which carries `B10;C1`. Fix: add that key to the §4 cell.

## RV6 (new findings, verbatim)

1. **MINOR: U8 names a row that does not carry C4.** `R4_GATE_INDEX.md:108` and `C4_user-guide-exception-and-boundary-list.md:155`, against `CODE_FIX_ROWS.csv:181` and `CFB-28_user-docs.md:22`. **Smallest fix:** add C4 to that row in the CSV and in the CFB-28 table. Then update the counts: C4 goes to 3, the total to 156 claim rows, and the CFB-28 index row and summary change to match.
2. **MINOR: the gate index's U6 row lists three keys, not four.** `R4_GATE_INDEX.md:106`. `DEL-06-04:SOW#CLM-010/R-06-04-012` is the one missing. **Smallest fix:** add that key to line 106.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
