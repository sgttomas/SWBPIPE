# R3R4-REVIEW-RV4 return (verbatim)

Agent a87f5b276fa0b7cf2; scope RV4 (C1–C7, gate index §3–§5); reviewed HEAD 23599b019.

VERDICT: FINDINGS

**Scope:** RV4, covering packets C1–C7 and `R4/R4_GATE_INDEX.md` §3–§5. Reviewed HEAD `23599b0199525c3853c740f8dd782999974e549e`. Brief sha256 checked and it matches (`ccdda3ae…6a51fd`). I worked read-only, ran no builds or tests, and wrote no scratch files.

`RUN` = `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS`. `F:` = the freeze tree `projects/chirality-piping/`.

**Checked in depth**
- **C1:** all 41 keys, checked against `CLASS_ASSIGNMENTS.csv` and `PACKET_CLAIMS.csv`. I opened every `F:` code line cited in §2 items 1, 2, 5, 6 and 7, plus DEC-026, DEC-020, OPS-K-UNIT-1, OPS-K-DATA-2 and DEL-00-06 REQ-06-02.
- **C2:** DEC-058 (decomp and register), `IP_AND_DATA_BOUNDARY.md` lines 4, 58 and 72, the listing of `validation/evidence/`, the T7-C09 cause and surface counts (29 + 2 = 31), and the T5B-C02 keys.
- **C3:** register rows D-47, D-49, D-67 and D-68, DEC-080, the T7-C08 and T7-C07 keys, the T9 `STALE_VV:VNV-*` items, and the "seven decisions" claim in the ledger.
- **C4:** DEC-107 (full row), `claims_registry.md` lines 99 and 219, DEL-11-01 SOW lines 61 and 249, the effective values of the DEL-11-04 row, and its `BlockedOnPacket` in H2.
- **C5:** the boundary lines 52 and 84, DEL-17-07 SOW lines 227 and 228, the fixture listings, the 11 keys, and their `BlockedOnPacket` in H2 and H3.
- **C6:** the classes of every key cited for M1–M15, and the convention line citations.
- **C7:** the T8 cluster counts (87 + 23 + 3 + 7 + 12 = 132), the class split per cluster, the T6-C09 keys, and every freeze line cited in §2.
- **Gate index:** §3–§5, and U1–U10 against the `unassigned` fields of the R3-INT RETURN events and the H2 "UNASSIGNED" section.
- Every full key in backticks in the C packets exists in `CORPUS_CLAIMS.csv`.
- `check_r4_coverage.py --check` passes; the C-packet `PORTIONS` entries (T7-C05, T5A-C05, T5B-C09) match what the packets state.

**Findings**

1. **ACTIONABLE.** `C1_code-contradicts-sow-intent.md:41` misquotes the contract.
   - The packet says OPS-K-DATA-2 "forbids invented data".
   - `F:docs/CONTRACT.md:27` actually says missing solve-required or rule-check-required values are explicit findings, never silent defaults.
   - Fix: restate OPS-K-DATA-2 correctly.

2. **ACTIONABLE.** C1 is inconsistent about which options need a contract basis (lines 92–95, 226–227, 244).
   - §6 says the "adopt" options for S1 and S3 move an INVARIANT contract boundary.
   - But §3's S1 consequences and the §8 table require a contract amendment to OPS-K-UNIT-1 only for S3b.
   - S1a accepts export redaction of units, which conflicts with OPS-K-UNIT-1: "imported values and exports must be unit-aware".
   - Fix: add the units part of S1a to the contract-amendment note in §8, or state why it is exempt.

3. **ACTIONABLE (clarity).** C2 Question 2 (`C2_…md:72-83`) forces a bundle.
   - It ties the review round to the PR #787 decision. Its options are: review + restore, defer + intended, review + intended.
   - The combination "defer reviews to DEC-058 + restore the test" is missing.
   - DEL-04-04 REQ-08 is a PROTECTED_CHECK on loop-identity diagnostics, not protected content.
   - Fix: split it into Q2 (review round, a/b) and Q3 (DEL-04-04 REQ-08: restore, or rule the removal intended).

4. **ACTIONABLE.** C4 over-reads DEC-107 (`C4_…md:37-38, 61-63, 120, 135-136`).
   - DEC-107 (iii) directs only the BS-ACCEPT withdrawal edits to the guide.
   - Its "Untouched: … the live `ScopeOfWork.md` files" marks the reach of that withdrawal act (`F:execution/_Decomposition/SOFTWARE_DECOMP.md:698`). It is not a standing bar on SOW edits, and it is not a general guide-scope exception.
   - The packet treats it as preventing the 1(b) SOW catch-up and as needing a DEC-107 amendment.
   - Fix: restate DEC-107's reach accurately, and drop "DEC-107 prevents / amend DEC-107 (iii)", or reduce it to "owner may confirm DEC-107 does not bear on it".

5. **ACTIONABLE.** The C5 blockers are wrong in the handoffs.
   - C5 §5 (line 95), §9 (lines 152–153) and gate index §3 (lines 84–85) say C5 blocks the H2 row `DEL-17-07:SOW#CLM-021`. But `CODE_FIX_ROWS.csv:357` (CFB-52) leaves `BlockedOnPacket` empty, and CFB-52 lists C5 only as "context".
   - The reverse error is in H3: ER-05 (`ENGINEERING_AND_REVIEW.md:255-258`) lists `DEL-17-07:STATUS#remaining/R01` as blocked on C5. C5 does not list that row, and RF-001 concerns DAG-006 wording, not fixture provenance (freeze `Review_Findings.csv`).
   - Fix: add C5 to row 357 (or have C5 say "context only"), and remove R01 from ER-05's C5 list.

6. **ACTIONABLE.** Gate index §4 (lines 89–104) and §6 (lines 162–164): rows behind the U-items carry no blocker.
   - U6 rows are unblocked: `CODE_FIX_ROWS.csv:6`, `:15`, `:83` (CFB-14, which says "unblocked rows may proceed").
   - The U8 row is unblocked: `:182` (CFB-28).
   - The U5 rows carry only `H3[T6-C03]`.
   - 82 of 116 T6-C02 rows have no blocker, which includes the U7 remainder.
   - So the readiness figures in §6 ("20 briefs with no blocker"; partly blocked rows "can go ahead") include rows still waiting on an owner answer.
   - Fix: in §4, list the affected keys for each U-item and state they are treated as blocked on the host packet. Qualify the §6 figures to match.

7. **ACTIONABLE.** U7 is placed nowhere (gate index line 101).
   - "Needs its own owner question" leaves an owner question on no packet, with no row filter.
   - Its examples already have homes: the DEL-03-01 round trip (`REQ-03-01-007`) is C6 M2; the DEL-16-02/16-03 constraint stage is B3; the DEL-15-03 export path is C1 S1 and B7.
   - Fix: place U7 by subject in those packets, and give a reproducing filter for the rest (or add one small packet).

8. **MINOR.** U10 (gate index line 104) duplicates C6 M11 (`C6_…md:39, 104-106`): the DEL-15-02 rerun-launch hint, W3 departure 5, and the launch-message rule. Fix: mark U10 "already drafted as C6 M11".

9. **MINOR.** Gate index line 76 cites "H3 ER-12..14" for the DEL-05-03 hold, but only ER-12 is DEL-05-03. ER-13 is DEL-02-02 and ER-14 is DEL-13-04/14-04/14-05. Fix: cite "ER-12".

10. **MINOR.** The P3 `INDEX.md` table "Class portions claimed by P3" (lines 16–27) overstates what P3 claims.
    - It lists REVIEW, NO_ACTION and ENGINEERING classes (T7-C09, T5B-C02, T7-C08, T7-C07, T6-C09).
    - `PACKET_CLAIMS.csv` holds only 45 C rows (C1 41, C4 4), so the table reads as double coverage against H3.
    - Gate index §3 also omits the two NO_ACTION classes put to the owner (T7-C08 in C3, T6-C09 in C7); they appear only in §2.
    - Fix: relabel the table as "discussed; route carrier", and add both NO_ACTION classes to §3.

11. **MINOR.** Two wording problems in C3.
    - `C3_…md:151-152`: "rows stay NO_ACTION" is wrong for the DEL-05-03 rows, which are on the ENGINEERING_AUTHORITY route.
    - Lines 158–160: item 3(b) chains straight to "activation and validation through the physics-audit activation plan" without saying activation needs its own authorization.
    - Fix: correct both wordings.

12. **MINOR.** C5 item 2 (`C5_…md:18-21`) asks three things: who makes the records, to which template, and whether the fixtures are non-public until then. Its options answer only the record form. Fix: add options for the public-status question, or state that it follows C6 M5.

13. **MINOR.** Loose wording in the gate index.
    - Line 154: "C7's readings set how 132 T8 rows are repaired". 23 unit rows route to code fix and 12 are NO_ACTION, so "routed" is the right word.
    - Lines 115–117: the R5 repair of the 84 capabilities "runs through H1" should add "under separate R5 authorization".

14. **MINOR.** C7 R4(b) (`C7_…md:82-84`) bundles two different replacements: "every REQ traced to an IN item is PROJECT_BASELINE" and "all stay as sealed". Fix: split them into (b) and (c).

15. **MINOR.** C1, C2 and C3 cite `_DECISIONS/_REGISTER.md:NN`. The repository-relative path is `execution/_Coordination/_DECISIONS/_REGISTER.md` (line numbers verified).

**Out of scope, one line:** H3 ER-14 carries `BlockedOnPacket` "C3 (partial)", but C3 §3/§7 says the DEL-13-04/14-04/14-05 engineering holds need no owner act.

**Needs the owner, not a repair**
- C1: intended or not, for each of S1–S8, with S4 sequenced after B9 and S6 ruled jointly with B3.
- C2: the evidence standard (Q1), and whether the PR #787 check removal was intended.
- C3: keep or lift each of the three holds (D-68 basis, DEC-080 activation, DEL-05-03).
- C4: the reach of the R18 exception, and whether R-DEL-11-04-002 is kept, narrowed or overtaken.
- C5: whether single coinciding values fall under the table rule.
- C6: the OWNER readings M1–M8, M12, M13 and the reading part of M15.
- C7: confirm or replace R1–R6.
- Gate index: which packet hosts U7's remainder, and the U2 choice between R5 repair and a DAG rebuild, which would need separate authorization.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

END-OF-RETURN
