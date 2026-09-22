# R3R4-REVIEW-RV5 return (verbatim)

Agent a06f2d0e2f4bc09df; scope RV5 (H1, H3); reviewed HEAD 23599b019.

VERDICT: FINDINGS

**Scope:** RV5, covering H1 (`R3/SCOPE_CHANGE_HANDOFF/`) and H3 (`R3/ENGINEERING_AUTHORITY/`). Reviewed HEAD `23599b0199525c3853c740f8dd782999974e549e`. The brief's sha256 matched. H1 and H3 file hashes match their `RETURN` events in `RUN_STATE.jsonl`. The brief and topic hashes cited in the H3 header also match. I edited nothing. My only writes were check scripts under `{SCRATCH}/review/`.

**Checked in depth**
- **H1, whole population by script:**
  - 241 unique items. The 227 capabilities match the classification filter exactly (57/44/68/57/1; T1 45, T2 50, T3 132).
  - The 17 REASSIGN keys equal T6-C07, and every one is in `CORPUS_CLAIMS.csv`. AuthorityNeeded is NO on 14 and OWNER on 3, as stated.
  - Kind counts match. CRLF throughout, and the `#END` row reads 241.
  - 117 items are blocked, and the §5 per-packet counts reproduce.
  - Every capability's Proposal carries the task CSV's classification and confidence. 20 are LOW.
  - The group item counts sum to 241.
  - None of the H1 or H3 claim keys appears in `T8_ROUTE_DISAGREEMENTS.csv` except the three H3 shows.
- **H1, against the evidence:** H1-001, 007, 017, 104, 106, 108, 110, 113, 116, 117, 176, 215, 224, and 225–241.
  - Reverse answers checked: RC-04-0061 (DEL-04-01, DEL-04-04), RC-10-0256 and RC-07-0365.
  - The PKG-16 verifier text at line 356 matches.
  - `OWNER_DIRECTIONS.md` Direction 1 matches, as do the scope-change contract's three checkpoints.
  - Freeze citations all bear out: `SOFTWARE_DECOMP.md` lines 223, 600, 635, 665, 667, 685 and 694; annex rows 17 and 21–24; `lib.rs:866` and `:2003`; `tauri.conf.json:23`; `App.tsx:145-149`; `engine.py:26`; `linear_supports lib.rs:431`.
- **H3, whole coverage by script:**
  - The filter returns exactly the 981 rows of the 17 classes, and the class counts match `CLASS_INDEX.csv`.
  - Task rows in scope: T8 1, T9 12, T11 12, T12 94. `Rows` sums to 1,090.
  - The T12-C06 engine portions are 52/38/3.
  - The T6-C06 and T7-C07 split keys are correct.
  - The deliverable and package counts in ER-01, 02, 05, 06, 07, 15, 16, 17, 18, 21 and 22 match.
  - The whole-file CONTEXT row breakdown (68/1/30/3) matches, and ER-12's BaselineClass is OWNER_HOLD.
- **H3 items in depth:** ER-02, 03, 05, 06, 12, 14, 15, 16, 19, 20, 21, 22, 24, 25, 31, 32, 33, 34.
  - Every partial-block key list was checked against class membership.
  - Freeze citations bear out: `ResultsPanel.tsx:114`, `protected_content_linter lib.rs:294`, `contract.py:239`, `result_envelope_binding.rs:159-182`, `product_physics lib.rs:6321`, `calculator.py:63`, `benchmarks/mechanics/Cargo.toml:19`, and the absent `validation/evidence/releases/`.
  - The D-20, D-67 and DEC-058 register rows match.

**Findings**

1. **ACTIONABLE.** `R3/ENGINEERING_AUTHORITY/ENGINEERING_AND_REVIEW.md:466-467` (ER-21, B7 portion) and `:501-504` (ER-22, B7 portion) list the wrong DEL-15-02 keys.
   - **Evidence:** in `CLASS_ASSIGNMENTS.csv`, the T6-C03 rows for DEL-15-02 are `SOW#CLM-005.r01`, `SOW#CLM-011.r05` and `SOW#CLM-027.r02`. `SOW#CLM-005.r02` and `.r03` are T7-C06 rows.
     - ER-21 names `CLM-005.r01/.r02/.r03`, which drops two of its own rows and takes two of ER-22's.
     - ER-22's 13-key list includes `CLM-011.r05` and `CLM-027.r02` (T6-C03 rows) and omits `CLM-005.r02` and `.r03`.
     - `CODE_FIX_ROWS.csv` has these rows right.
   - **Smallest fix:** ER-21's B7 portion becomes `SOW#CLM-005.r01`, `SOW#CLM-011.r05`, `SOW#CLM-027.r02`. In ER-22's list, replace `CLM-011.r05` and `CLM-027.r02` with `CLM-005.r02` and `CLM-005.r03`.
2. **MINOR.** The H1 blocker tokens are stale against the integration record.
   - **Evidence:**
     - `SCOPE_CHANGE_ITEMS.csv` H1-241 reads "UNASSIGNED", and H1-176 reads "ENGINEERING(… no H3 item or packet)" (`SCOPE_CHANGE_HANDOFF.md:308-311`, `:332-333`, `:366-373`).
     - Meanwhile `R4_GATE_INDEX.md` §4 attaches U3 (H1-241) to B7 and CAP-PHYS-029 to C3. Neither the B7 nor the C3 packet text mentions them; the gate index says this is deliberate.
     - The four T1 REVIEW items (H1-017, 110, 116, 117) are routed "with H3's items", but H3 has no ER ID for them, so the token resolves to nothing.
   - **Smallest fix:** add a note in H1 §5 (or the gate index) mapping these tokens to U3/B7, C3 and a named review destination. Alternatively, add an H3 item for the four T1 REVIEW rows.
3. **MINOR.** `ENGINEERING_AND_REVIEW.md:149-150` (ER-02) names "Agent 0's R3 integration harmonising causes" as a co-holder.
   - **Evidence:** no R3 or R4 record (`R3_SYNTHESIS.md`, `COVERAGE_AND_QA.md`, `R4_GATE_INDEX.md`, the packets) shows this harmonisation being done. The step has no one to carry it.
   - **Smallest fix:** state that it was not performed and passes to the ER-02 reviewer, C7 or R5.
4. **MINOR.** `ENGINEERING_AND_REVIEW_ITEMS.csv` uses LF line endings.
   - **Evidence:** the R3-TASK brief's CSV format requires CRLF (line 117), and H1 follows it.
   - **Smallest fix:** rewrite the file with CRLF.
5. **MINOR.** The ER-31 CSV `Deliverables` field says "16 deliverables (reproduce via filter in ENGINEERING_AND_REVIEW.md)", but the MD (`:594-596`) gives no filter.
   - **Evidence:** 16 is the union of T11 `Deliverables` over the 7 source rows, which I confirmed.
   - **Smallest fix:** state that filter in the MD.
6. **MINOR.** Some keys are written in shorthand or with incomplete paths.
   - **Evidence:**
     - `ENGINEERING_AND_REVIEW.md:167-169` gives `DEL-03-04` and `DEL-03-05`, and `:259-260` gives `DEL-04-04` and `DEL-04-05`. Each is shorthand for a key whose suffix appears only on the following key.
     - `SCOPE_CHANGE_HANDOFF.md:93` cites `Vocabulary_Annex.md` without its folder `execution/_ScopeChange/SCA-009_2026-08-20_0000/`.
     - `:166` cites `PKG-16_VERIFICATION.md:356` without its folder `RUN/WAVES/W1/PKG-16/`.
   - **Smallest fix:** spell out the full keys and paths.

The fences hold in both handoffs. Each has the claim fence, and neither makes a certification, compliance, approval or acceptance claim. There are no equation sources, no quoted protected content and no absolute paths. Neither handoff decides or repairs anything, and each on-ruling path goes through scope-change checkpoints or separately authorized R5.

**Needs the owner, not a repair**
- H1-102, 103, 105, 107 and 224 are routed OWNER_DECISION by T3. The topic file's crosswalk placed them in H1 with no packet, so they reach the owner only at scope-change checkpoint 1. The owner should confirm that is acceptable.
- U3 (H1-241, the form of the DEL-17-02 schemas) and CAP-PHYS-029 (H1-176) reach the owner only as questions attached to the B7 and C3 sessions, with no packet text.
- The ER-31 holder is left open by design: ENGINEERING per T11, or OWNER per the DEL-00-06 RemainingWork.
- H3 reports U4 and U5 as UNASSIGNED; the gate index places them with C6 and C1.
- H1 groups A–C are each a single decision (B1, B2, B3) even though they span 14–34 items. The owner should rule each group as one amendment.

**Outside my scope, noted in passing**
- H4 carries `DEL-10-01:SOW#CLM-004` and `DEL-10-03:SOW#CLM-004` as R5 rows, following the T8 view; their class route is REVIEW (T5A-C08). RV7 should confirm that H4 shows both views.
- T12-C06 ("engines replaced by a product port") contains a `core/gui/accessibility` row, `DEL-00-03:AB#realized-artifacts.r02`, which looks misclustered. It is H4's row.
- `H3_TOKEN_MAP.csv` (both copies) and `R4/PACKET_CLAIMS.csv` are also LF.

END-OF-RETURN
