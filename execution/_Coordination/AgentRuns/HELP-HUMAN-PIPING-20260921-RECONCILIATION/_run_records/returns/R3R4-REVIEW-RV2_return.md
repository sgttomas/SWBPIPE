# R3R4-REVIEW-RV2 return (verbatim)

Agent a81f236a9717a9972; scope RV2 (A1–A10); reviewed HEAD 23599b019.

VERDICT: FINDINGS

**Scope:** RV2 (P1 packets A1–A10). **Reviewed HEAD:** `23599b0199525c3853c740f8dd782999974e549e` on `claude/piping-recon-r3-20260922`. The brief's hash `ccdda3ae…51fd` matches.

**Checked in depth:** A1, A2, A3, A4, A5, A6, A7, A8, A9, A10. That is all ten. I opened every recommendation's evidence.
- **Freeze citations.** I opened each packet's `F:` path:line (register rows DEC-009/010/017/028/027/057/081/089/094/101/105/051/058, D-68 at REG l.105, SCA-004/SCA-001, the code lines, `_STATUS.md`, AB and SOW lines, PRD, DIRECTIVE, LICENSE). I also checked the RUN conventions (CP-04, CP-10, C6(e), C9, A4, R0 item 3 and addendum item 2), Direction 8, and the W3 assessment lines 70, 83 and 88. All support the claims made.
- **Keys and portions.** A script under `{SCRATCH}/review/` confirmed:
  - every backticked key in the 10 packets exists in `CORPUS_CLAIMS.csv`;
  - every portion filter reproduces its keys over `CLASS_ASSIGNMENTS.csv`;
  - A3's 13 topic key lists equal T7-C01 exactly (50 rows, 20 deliverables, 9 packages);
  - A4's variant split (76/6/3), package counts and 80 deliverables match;
  - A9's CP-03/CP-02 split (169/44), waves, packages, 69 deliverables and 6 FIELD rows match;
  - the split-class sums agree with `PACKET_CLAIMS.csv`: T4A-C06 7+5+2, T5B-C07 16+6, T4B-C02 1+4, T4B-C01 85+1;
  - DEL-01-01's 41 class rows are fully accounted for (30 in the portion plus 11 on other routes).
- **Fences.** All ten carry the claim fence. None has an absolute path, a certification or compliance claim, or an equation source.

**Findings**

1. **ACTIONABLE.** `A4_rename-identity-residue.md:73` (recommendation) and `:77` (option 1 mechanism).
   - **Problem:** The packet recommends authorising the option-1 text sweep "over the 85 rows' surfaces". But 9 of those rows (6 PROJECT_BASELINE persistence, 3 FROZEN_CONTRACT) and the default-identifier residue name live code identifiers, so their repair depends on options 2 and 3:
     - under 2b/3b they are "re-read as not residue" (`:30`, `:80`);
     - under 2a/3a, renaming the text before the code changes makes the SOWs name identifiers that do not exist yet.
   - The packet's own risk line (`:69`) sees the problem, but the recommendation and mechanism ignore it.
   - **Smallest fix:** Limit option 1 and its recommendation to prose product-name residue. State that rows naming active identifiers (at least the 9 PROJECT_BASELINE rows) wait for the 2/3 ruling and follow it.

2. **ACTIONABLE.** A5 and H4 disagree on two rows.
   - **Problem:** `A5_pkg00-semantic-ready-lifecycle.md:66` and `:98` say the SEMANTIC_READY sentence in `DEL-02-04:CONTEXT#architecture-basis-injection` and `DEL-02-05:…` follows A5, and that H4 repair of it is blocked. `R4/R5_TRANCHE_PROPOSAL/R5_REPAIR_ROWS.csv` has both rows with an empty `BlockedOnPacket`. A T4A-C01 repair could then restate the sentence before the lifecycle choice, against A5's own "repair once" sequencing (`:85`).
   - **Smallest fix:** Add `A5` to those two H4 rows, or narrow A5 `:98`.
   - **Related (MINOR):** A1 `:86` and `:115` ask H4 to block `DEL-17-01/02:CONTEXT#architecture-basis-injection` on A1. H4 deliberately leaves them unblocked (`R5_TRANCHE_PROPOSAL.md:199`), and A1's own table (`:83-84`, RESOLVED_PAIR, no DEC-009 element) agrees with H4. Drop the two rows from A1's blocks.

3. **MINOR.** `A5…:73` says "82 downstream deliverables", but the 82-row portion spans 81 deliverables (DEL-11-05 carries two rows); 82 is the SRE-5 deliverable count. `:77` "82 contexts" is really 81 `_CONTEXT` rows plus 1 SOW row. **Fix:** 81.

4. **MINOR.** `A9_d41-current-declaration-blocks.md:48` says "SOW 208 (BLOCK)". The SOW rows are actually 207 BLOCK plus 1 SURFACE (`DEL-04-05:SOW`); the class-wide 208 BLOCK count includes the STATUS block. **Fix:** "SOW 208 (207 BLOCK, 1 SURFACE)".

5. **MINOR.** `A6_issued-del-01-01-change-path.md:107`: option 3 calls DEL-01-01 `_STATUS.md` a "non-ISSUED surface" editable by ordinary R5. The profile (`docs/RECONCILIATION_PROFILE.md:84-85`) routes any change to the ISSUED deliverable through scope change, and H4 places `DEL-01-01:STATUS` in `R5-ISSUED-DEL-01-01`, blocked on A6. **Fix:** Put the pointer edit under the A3a owner record and the ISSUED path.

6. **MINOR.** `A6…:38`: rename-routing option (b), "extend C6(d) to the five ISSUED pin rows", amends a RULED convention, but its holder is not named. **Fix:** Name it (owner, drafted via HELPS_HUMANS), as A3 does for CP-10.

7. **MINOR.** `A2_json-hash-basis.md:91` says "a label correction is needed whichever basis is chosen". Under option 1 the MBF fix is to the bytes, after which the label is accurate. **Fix:** "label and bytes must be made to agree under any option".

8. **MINOR.** `A10_baseline-rows-overtaken-by-rulings.md:66`: "(CONTESTED)" follows both DEL-12-05 keys, but only `CLM-033.r02` carries a CONTESTED correction; `CLM-011.r02` has none (`:39` has it right). **Fix:** Attach the tag to `CLM-033.r02` only.

9. **MINOR.** Two wording and path issues.
   - **Paths:** `A5:15-16`, `A6:15`, `A8:19` and `A10:20-24` cite `F:execution/PKG-xx_…/DEL-xx-xx_…/<file>`, dropping the `1_Working/` level, so the paths do not resolve as written.
   - **Wording:** `A3:73` calls `DEL-07-01:SOW#CLM-005.r05` "FIRM", but its effective Confidence is LOW; FIRM is the tag on its correction.
   - **Fix:** Add `1_Working/`; say "FIRM correction".

**Outside scope, one line each:**
- The topic file does not list T4B-C01 or T4B-C02 as split classes, though both are split across A4/A6 and A2/A6. The packets report this; it is for Agent 0 to fix in `R3_INTEGRATION_TOPICS.md`. The coverage totals are unaffected.
- The H4 mismatch in finding 2 is in RV7's area.

**Items for the owner rather than a repair**
- A4: whether Agent 0's reading of the four identifiers kept on 2026-09-18 as residue is correct. It is an AGENT reading of conditional owner words (`R0_RULING.md` addendum item 2).
- A7 question 2: only the owner can say whether `HISTORICAL_N7_ACCEPTANCE_INTAKE_V1.json` exists outside the tree. It is absent from the freeze.
- A6: whether the DEC-081 Wave 2 edit validly reached the ISSUED SOW. There is no human disposition of the post-edit bytes.
- A3 option 1b: amending the RULED convention CP-10 mid-run.
- A10 item 4, option (b): reopening DEC-051 means revisiting the tier-0 ruling D-T0-04, which lives outside the project (`_DomainEngines/_DECISIONS/D-T0-04_data_residency.md`).

END-OF-RETURN
