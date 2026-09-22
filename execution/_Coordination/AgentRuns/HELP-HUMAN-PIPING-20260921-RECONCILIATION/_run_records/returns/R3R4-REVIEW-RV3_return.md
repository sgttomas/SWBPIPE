# R3R4-REVIEW-RV3 return (verbatim)

Agent a76621eef6ae365d9; scope RV3 (B1–B12); reviewed HEAD 23599b019.

VERDICT: FINDINGS

**Scope:** RV3, packets B1–B12 (`RUN/R4/DECISION_PACKETS/P2/`, plus `INDEX.md`). I reviewed HEAD `23599b0199525c3853c740f8dd782999974e549e`. HEAD moved to `583f439a6` while I worked, but that commit only adds 8 lines to `RUN_STATE.jsonl` (the reviewer dispatch record), so the packets are the same. Brief sha256 `ccdda3ae…6a51fd` matches.

**Items checked in depth:**
- **All twelve packets (B1–B12).** I re-opened every freeze `path:line` the packets cite as "checked", and every cited governing line: `SOFTWARE_DECOMP.md` :122, :128, :223, :575, :600, :603, :611, :619, :665, :667, :672, :685, :694; annex rows 17, 22 and 23; the DEL-07-09 `_CONTEXT.md:36-37` and `_STATUS.md:3,13`; and register rows D-16, D-21 and D-29.
- **Verifier lines.** I opened the lines cited from the PKG-03, 04, 12, 13, 15, 16 and 17 verification reports and from `CORPUS_CROSS_WAVE_VERIFICATION.md`. I also opened `W1_GATE_RULING.md` Effect 5 and the W1 `RESOLUTIONS.csv` CONTESTED rows.
- **Counts and keys, by script.**
  - The T6-C04 split: B10's 20 keys and B12's 67 keys cover all 87 rows, with no overlap and no missing row.
  - T5A-C04 (19), T5A-C05 DEL-17-04 (3) and T5B-C09 DEL-07-02 (1).
  - The T12 clusters C01 (213), C02 (95), C03 (112), C04 (57) and C05 (27), including the per-deliverable route splits and the one C05/T6-C04 overlap.
  - The capability lists in B1–B6 against T1, T2 and T3.
  - Tier and disposition counts in B10 and B12.
  - Every cited key exists in `CORPUS_CLAIMS.csv`.
- **Fences.** All twelve packets have the claim fence. I found no absolute paths, no certification or compliance claims, and no equation sources.

**Findings**

1. **B9_pkg13-product-status.md:20-22 — ACTIONABLE (misquoted context).**
   - The packet quotes the verifier's "No ruling permits this, so it goes to you at R4" as support for "No ruling selects PKG-13's product role".
   - At `WAVES/W3/PKG-13/PKG-13_VERIFICATION.md:488-491` that sentence is about the Python engines against DEC-009 (§8 item 3), which is topic A1. It is not about PKG-13's product role.
   - **Fix:** drop the quote, or re-attribute it to the A1 question. State that no ruling selecting PKG-13's product role was located, as the writer's own finding.

2. **B4_load-case-editor-landing.md:99-107 — ACTIONABLE.** Three problems in the same list:
   - **Wrong reading on two rows.** The packet says all six contested rows carry the candidate reading `UNKNOWN · AUTHORITY_UNCLEAR · PROJECT_BASELINE · RECORD · OWNER`. In `WAVES/W1/RESOLUTIONS.csv`, `DEL-07-03:SOW#CLM-018` and `SOW#CLM-012/DEL-07-03-R-002` carry no candidate reading. Their note reads "Rests on the same R-005 reading as W2-W5; not counted by the verifier".
   - **Current values missing.** The packet does not give the rows' current (effective) values, which are `ACCEPTED_DIVERGENCE · OWNERSHIP_ELSEWHERE · PROJECT_BASELINE`, route NO_ACTION.
   - **Three dependent rows omitted.** Option (1b) restores R-005 *and* R-006, so three more T6-C09 rows turn on Question 1: `SOW#CLM-005.r05`, `SOW#CLM-008/DEL-07-03-R-006` and `SOW#CLM-012/DEL-07-03-R-006`.
   - **Fix:** correct the reading on the two rows, add the current values, and list the three R-006/r05 rows as dependent C7 rows.

3. **B1_product-solve-path-owner.md:83-88 (sub-question iii) — ACTIONABLE (missing evidence).**
   - Annex row 22 (`AX:126`, the DEC-094 coverage contract) names `compute_pipe_mass_per_length` in `core/product_physics` and states "mass properties from DEL-03-08".
   - That is governing text bearing directly on option (iii-a), and the packet does not cite it.
   - **Fix:** add it to §2 and §4 as evidence, without turning it into a recommendation.

4. **B6_panels-vs-no-gui-sows.md:65-66 and :103, with H4 `R5_REPAIR_ROWS.csv` — ACTIONABLE (cross-scope).**
   - B6 says H4 should mark `DEL-17-05:SOW#CLM-011` BlockedOnPacket B6.
   - The H4 row carries only `H3:T5A-C01`. Left as is, the text catch-up could silently ratify the panel.
   - **Fix:** add B6 to that H4 row.

5. **B8_model-state-and-comparison.md:76-80, with H2 `CODE_FIX_ROWS.csv` — ACTIONABLE (cross-scope).**
   - B8 says H2 should mark `DEL-14-01:SOW#CLM-004`, `CLM-011.r01` and `CLM-011.r04` BlockedOnPacket B8.
   - H2 has B8 only on `CLM-011.r01`. `CLM-004` and `CLM-011.r04` carry `A2` only, although B8 says T12 routes them with the DEC-009 cluster (A1).
   - **Fix:** either add B8 (and A1 if intended) to those two H2 rows, or narrow B8's instruction to what H2 actually carries.

6. **B10_plugin-adapter-runtime-and-grants.md:80 — MINOR.** "All 20 are PARTIALLY_IMPLEMENTED" is wrong. `DEL-17-09:STATUS#remaining/R03` is DOCUMENTED_UNIMPLEMENTED. **Fix:** "19 PARTIALLY_IMPLEMENTED, 1 DOCUMENTED_UNIMPLEMENTED".

7. **B10_plugin-adapter-runtime-and-grants.md:171-172 — MINOR.**
   - §9 "Blocks" omits the three DEL-06-02 CP-11 rows: `SOW#CLM-006.r05`, `SOW#CLM-013/REQ-06-02-010` and `SOW#CLM-016/REQ-06-02-010`.
   - These are T6-C03 rows whose no-bypass branch waits on the D2 runtime (T6 observation O5). H2 already marks them B10.
   - **Fix:** list them.

8. **B1_product-solve-path-owner.md:111-117 — MINOR.**
   - CAP-SOLVER-070 (T3-G9) is not listed. Its unowned `nonlinear_integration` README part names the "T3-G3 owner", so it follows sub-question (ii).
   - Separately, option (ii-c) at :81 presents the PKG-04 verifier's strict reading without saying the verifier itself called it "weak" (`PKG-04_VERIFICATION.md:357-358`).
   - **Fix:** add CAP-SOLVER-070 as a related item, and note "weak" beside the strict reading.

9. **B3_runtime-operation-applier.md:114-115 — MINOR.** "OtherCorrections give tier PROJECT_BASELINE and AuthorityNeeded OWNER" is only half right. OtherCorrections supply only the tier; AuthorityNeeded OWNER is the row's current (effective) value. **Fix:** reword.

10. **B5_unowned-and-disputed-panels.md:64 — MINOR.** The packet refs are given as "DEL-09-04, 09-05, 10-04". `ValidationEvidencePanel.tsx:142` lists seven: DEL-09-01, 02, 03, 04 and 05, DEL-10-04 and DEL-08-05. **Fix:** give the full list, or write "including".

11. **B9_pkg13-product-status.md:85 — MINOR.** "AuthorityNeeded REVIEW" on `DEL-13-01:SOW#CLM-005.r05` comes only from the FIRM OtherCorrections note; the current value is NO. **Fix:** say so.

**Out of scope, tripped over:**
- C7 (`P3/C7_*.md`) does not name B4 as a dependency, although B4 lists C7 as blocked by B4 (RV4).

**Items that need the owner rather than a repair**
- **B1 (ii):** confirm DEC-044 as written and have it carried out through scope-change, or amend it. (i) and (iii) are open owner calls.
- **B4 Q1:** DEC-094's re-point cannot be carried out under DEL-07-09's accepted envelope. The owner must amend the envelope, amend DEC-094, or create a deliverable. The same choice constrains B2 (iii-b), B5 items 3 and 4, and B11.
- **B5 item 3 and B12 D11:** these are readings the packets offer (confirm annex row 23; user rule packs satisfy SOW-014). They count as settled only if the owner confirms them.
- **B12 D1:** the packet leans toward (b), leaving OI-007 unruled, as the status quo. C7's cause reading (DEFERRED_BY_RULING against PARTIAL_SLICE) and the class route stay open, so the owner should weigh D1 together with C7.
- **B6:** one convention for PKG-17 and the other no-GUI panels, to be applied consistently with B5 item 2b.
- **B7, B8 and B9:** these should be ruled with A1, or after it, because option (a) in each presupposes Python in the product path.
- **Ruling pairs flagged in the INDEX coverage notes** (these are sequencing questions, not defects):
  - B12 D14(4) with A10 (PB-TBD-003);
  - B12 CF-001/002 with A8;
  - B5 item 4 with A8 (the palette row).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

END-OF-RETURN
