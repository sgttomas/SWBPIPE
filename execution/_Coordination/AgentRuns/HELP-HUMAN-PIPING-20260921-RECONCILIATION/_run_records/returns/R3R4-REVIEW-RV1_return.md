# R3R4-REVIEW-RV1 return (verbatim)

Agent a9987925af4b0ce54; scope RV1 (deterministic layer and reports); reviewed HEAD 23599b019.

VERDICT: FINDINGS

**Scope:** RV1 (deterministic layer and reports). **Reviewed HEAD:** `23599b0199525c3853c740f8dd782999974e549e` on `claude/piping-recon-r3-20260922`. The brief's sha256 matched `ccdda3ae…6a51fd`. I made no writes to the repository or `{FREEZE}`, and the working tree stayed clean.

**Items checked in depth**
- **Tool runs** (all with `PYTHONDONTWRITEBYTECODE=1`):
  - `synthesize_r3.py --run-dir RUN --check` → CHECK PASS.
  - `index_r3_classes.py --run-dir RUN --check` → CHECK PASS.
  - `check_r4_coverage.py --check` → `PACKET_CLAIMS OK`, exit 0.
  - The first two need `--run-dir`, which the brief's command omitted. That is a note on the brief, not a candidate defect.
- **Tool logic:**
  - Value substitution, the setter-consistency assert, the NEUTRAL set (checked against CONVENTIONS: NOT_ASSESSED only with NON_NORMATIVE) and the capability status ladder.
  - Class-table parsing: one qualifying table per task file.
  - Register counts made with `splitlines`: no fields contain newlines, so the counts are exact.
- **`check_r4_coverage.py` against every packet's §5:** A1–A8, A10, B4, B6, B10, B12, C1, C4.
- **Mention predicates:** I recomputed them independently.
  - For all 14 T4A-C06 rows, the A1/A7/A10 mention predicates give the same result as each packet's stated filter, and no key is mentioned by two packets.
  - B10's 20-key list equals the mention set, and every key in it is T6-C04.
  - The 24 STATED counts sum to 444, and the packet headline totals also sum to 444.
- **Every figure checked against the tables:**
  - `R3_SYNTHESIS.md` §1–§9: dispositions; route totals; the 130/57/38/34/1 and 68/6 capability splits; per-package divergence (25%–41%); 666 rows, which equal the T12 keys exactly; 436 and 118; 151/65/37; 29 mismatch rows; 83 keys in 81 deliverables; 1,522/737/169; 14 = T7-C08 10 + T7-C07 4; 408 NO_ACTION with 15 routed to another class; 113; T10 and T11 figures. I also confirmed the D-42 = DEC-076 ruling of 2026-07-15 in the freeze's July `RUN_BASIS.md`.
  - `COVERAGE_AND_QA.md`: wave splits (1,318/2,394/6,177; 420/773/2,106; 47/122/425/5); 137; 176; 45; Remaining counts match the inventory for all 102 deliverables; 84.
  - `R4_GATE_INDEX.md`:
    - §1: H1 241/227, H2 54/386, H3 34, H4 2,234/20.
    - §2: H4 carries every one of the 2,099 R5 rows once, and its 43 repeated keys are ITEM rows (one per deliverable). H2 carries all 375 code-fix rows. H3 ER-01..07 hold the 7 REVIEW classes and ER-08..14 the 11 ENGINEERING rows. H1 carries all 17 T6-C07 rows. The split table matches.
    - §6: A5 82; C7 132; A4+A9 298; 1,290 / 20 / 124.

**Findings**

1. **ACTIONABLE** — `RUN/tools/check_r4_coverage.py:29,73`, as relied on by `R4/R4_GATE_INDEX.md:41,53-55` and `R3/COVERAGE_AND_QA.md:158-162`.
   - **The problem:** "first match wins" means the "exactly one packet / no row claimed twice" check can never fail. Each split's later portion uses `whole`, which matches everything, so a key claimed in two packets' portions is silently given to the first. Only the count comparison can catch it, and only when the counts don't happen to offset each other.
   - **Predicates differ from the packets:** for T4A-C06 the tool tests whether the key is mentioned anywhere in A1/A7/A10. The packets state filters instead: A1 by key suffix, A7 by deliverable and suffix, A10 by `DeliverableID == 'DEL-09-05'`. B4 is coded as a single-key equality while B4 states `DeliverableID == 'DEL-07-02'`.
   - **Evidence:** my independent recomputation found no actual double claim and no disagreement, so the coverage result holds. The gap is in the check and in the records' description of it.
   - **Smallest fix:** code each portion as the packet states it, including complements such as A4 `ClaimKey != 'DEL-01-01:SOW'` and B12 as "not in B10's list". Evaluate every predicate and raise an error when a row matches more than one. Alternatively, reword the two records to say that uniqueness comes from the ordering plus the portion counts.
2. **MINOR** — `R3/R3_SYNTHESIS.md:283-284` and `R3/COVERAGE_AND_QA.md:138-139`.
   - **The problem:** both say "666 is a lower bound: 98 more ALIGNED rows cite the same engines".
   - **Evidence:** `R3/TASKS/T12_UNREACHED.md:479-488` says the rows it sampled are absence or restriction claims, or rows that also cite desktop paths. It concludes only that the population "may therefore exceed 666".
   - **Fix:** "98 further ALIGNED rows cite these engines without the marker (unsampled or absence claims), so rows relying on them may exceed 666."
3. **MINOR** — `R4/R4_GATE_INDEX.md:157-158`.
   - **The problem:** it says B1, B2 and B7 "gate most H1 CREATE and ASSIGN items and the largest T12 clusters".
   - **Evidence:** B1 and B2 block 64 of the 167 CREATE/ASSIGN items (59 of the 167 have no blocker). B7 blocks no H1 item. B7 does carry the largest T12 cluster (T12-C01, 213 rows), but B1 and B2 carry none.
   - **Fix:** "B1 and B2 block 64 of 167 H1 CREATE/ASSIGN items; B7 carries T12-C01 (213 rows)."
4. **MINOR** — `R3/R3_SYNTHESIS.md:44`.
   - **The problem:** it says "about 12 holds the code settled".
   - **Evidence:** A3 §1 names 13 hold topics.
   - **Fix:** "13 hold topics".
5. **MINOR** — `R3/R3_SYNTHESIS.md:296-297`.
   - **The problem:** it says the eleven possible defects are "routed to review".
   - **Evidence:** T10 only proposes REVIEW (`T10_JULY.md:100`).
   - **Fix:** "proposed for review".

**Outside RV1, one line:** `R4/DECISION_PACKETS/P1/A5_pkg00-semantic-ready-lifecycle.md:73` says "82 downstream deliverables across PKG-01 to PKG-16". The 82 portion rows span 81 deliverables. The 82 figure is SRE-5's, and it includes DEL-17-02, which is in PKG-17.

**Items that need the owner rather than a repair:** none arising in RV1.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
END-OF-RETURN
