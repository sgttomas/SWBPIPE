# R3R4-REVIEW-RV7 return (verbatim)

Agent abaad7a6adbe1c827; scope RV7 (CFB-28..54, H4); reviewed HEAD 583f439a6.

VERDICT: FINDINGS

**Scope:** RV7 covers the H2 briefs CFB-28 to CFB-54 and H4 (`R4/R5_TRANCHE_PROPOSAL/`). I reviewed HEAD `583f439a6e2b55599b5496c96f3257eacd0b4091`. The brief's sha256 matched (ccdda3ae…51fd). The sha256 of `CODE_FIX_CANDIDATES.md` and `R5_TRANCHE_PROPOSAL.md` matches the `RETURN` records in `RUN_STATE.jsonl`. I edited nothing in the repository. The only files I wrote are check scripts under `{SCRATCH}/review/`.

**What was checked in depth**
- **H2, all 27 briefs (192 rows), by script.**
  - Every key exists in `CORPUS_CLAIMS.csv`. Class, Authority and DeliverableID all match `CLASS_ASSIGNMENTS.csv`.
  - Every T6-C03 and T7-C06 row carries an H3 token, and every token resolves.
  - Every T8 route-disagreement row is blocked. Each brief's markdown lists exactly its CSV rows.
  - The overview table's row and blocker counts match the CSV.
- **H2, evidence and code fidelity.** I opened the ledger rows, OtherCorrections entries and `{FREEZE}` lines for CFB-28, 29, 31, 32, 33, 36, 40, 43, 44, 46, 47, 51, 52 and 54. All code citations hold:
  - `target_mapping/contract.py:239`
  - `native_json/package.py:26-33`
  - `caepipe_mbf/package.py:335` and `:632-640`
  - `lib.rs:4213`
  - `pcf_export/package.py:724-725`
  - `PcfExportPanel.tsx:701`
- **H4, whole CSV (2,234 rows), by script.**
  - Keys exist and class/Authority match. Every class row routed to R5 is present.
  - Tranches follow the owning deliverable. No OWNER or REVIEW row is left unblocked.
  - Wave figures reproduce: 1,290 / 605 / 348 / 17, overlap 26.
  - The H3 token map resolves to ER-02, ER-15..20 and ER-24, which I checked against H3.
  - The freeze and profile citations hold: DEL-01-01 `_STATUS.md:3`, `init/`, `DAG_Audit.md:15`, DEL-14-01 `_CONTEXT.md:55`, profile §4 and §8, and D-73 item 4 (O-A).
- **Cross-check against packets.** I compared every packet §9 "Blocks … H2/H4" statement against the rows.

**Findings**

1. **ACTIONABLE: CFB-33 and CFB-43 are missing the B9 block.**
   - Where: `CFB-33_constraint-evaluation.md:19,43,52` and `CFB-43_runtime-constraint-stage.md:45,53`.
   - CFB-33's scope ("give `validate_constraint_envelope` a product caller or record the library reading") is B9's option (a) against (b).
   - The T12 reading of `DEL-13-03:SOW#CLM-005.r04` is OWNER_DECISION (B9 §5 lists it under "Both views"). This is the only T12 owner-route row in scope with no blocker.
   - CFB-43 wires the DEL-13-03 stage into the runtime route and changes the `lib.rs:4213` `constraint_validation` label. B9 §2 cites that same label, and B9 §9 says it blocks H2 briefs touching the desktop constraint display. B9 option (a) also needs A1.
   - Fix: add B9 to both CFB-33 rows and the three CFB-43 rows. Add B9 (and A1, where Python stays on the path) to both briefs' dependencies.
2. **ACTIONABLE: `DEL-17-07:SOW#CLM-021` is missing the C5 block.**
   - Where: `CODE_FIX_ROWS.csv:357` and `CFB-52_pcf-family-coverage.md:25,61`.
   - The row's remaining work is to document fixture provenance.
   - C5 §5 (line 95) says the handoffs carry C5 on these rows, and C5 §9 names "the H2 T6-C01 DEL-17-07 row". CFB-52 lists C5 as context only.
   - Fix: set `BlockedOnPacket` to C5 on that row and update CFB-52's blocker section.
3. **ACTIONABLE: the six DEL-14-02 T5B-C10 rows sit in Wave A, but A2 says it blocks them.**
   - Where: `R5_REPAIR_ROWS.csv:1844` onward, `R5_TRANCHE_PROPOSAL.md:149`, and `A2_json-hash-basis.md:72,105`.
   - A2 says its answer settles these rows and blocks their H4 repair.
   - Fix: mark the six rows A2 and update §3, §4 and the wave counts.
4. **ACTIONABLE: `DEL-17-05:SOW#CLM-011` carries only the H3 token.**
   - Where: `R5_REPAIR_ROWS.csv:2127`, and `B6_panels-vs-no-gui-sows.md:65,103,153`.
   - B6 says a text catch-up on this row would silently ratify the mounted panel, and asks H4 to mark it B6.
   - Fix: add B6 to the row.
5. **ACTIONABLE: H4 does not carry A1 where A1 asks for it.**
   - Where: `R5_REPAIR_ROWS.csv:2070,2082`, `R5_TRANCHE_PROPOSAL.md:199`, and `A1_dec009-python-engines.md:86,115`.
   - A1 asks for an A1 block on the two DEL-17-01/02 `CONTEXT#architecture-basis-injection` rows. It also blocks "any DEC-009 text on DEL-13-03/04, 14-03/04, 15-02 … CONTEXT rows". Those are the five T4A-C01 main injection rows, and all five are in Wave A.
   - H4 §6 decides "not blocked" for the two DEL-17 rows, which settles a disagreement A1 wants held.
   - Fix: add A1 to the two DEL-17 rows, plus either A1 or a tranche guard ("leave DEC-009/Resolved Baseline text as it stands until A1") for the T4A-C01 injection rows. Otherwise record the divergence in `R4_GATE_INDEX` §5.
6. **ACTIONABLE: the DEL-02-04 and DEL-02-05 injection rows are unblocked.**
   - Where: `R5_REPAIR_ROWS.csv:207,242`, and `A5_pkg00-semantic-ready-lifecycle.md:66,98`.
   - A5 blocks "the SEMANTIC_READY sentence in the DEL-02-04/05 parents".
   - Fix: add A5 (and C7, following the SR-1 pattern), or a guard that the pin repair leaves that sentence alone.
7. **ACTIONABLE: the tokens `UNASSIGNED-U1` (2 rows) and `UNASSIGNED-U2` (15 rows) do not resolve.**
   - Where: `R5_REPAIR_ROWS.csv:579,594` and the D-02 rows.
   - They match no packet ID, H3 item or `H3_TOKEN_MAP.csv` entry (brief check 4). `R4_GATE_INDEX.md:95-96` proposes C7 and C6 for them.
   - Fix: add the two mappings to `R4/R5_TRANCHE_PROPOSAL/H3_TOKEN_MAP.csv` or a sibling map, or note them in gate index §1.
8. **ACTIONABLE: the proposed R6 manifest omits owner-route rows that later become R5 repairs.**
   - Where: `R5_TRANCHE_PROPOSAL.md:31-34,259-262`.
   - §9 item 5 makes "this row file" the authorised manifest, but it excludes owner-route rows whose rulings authorise R5 repairs. Examples: A3 (50 rows), A4 (85), A8 (5), A10 (8); the packets say they block "H4 repair" of those rows.
   - Fix: state in §9 that owner-route rows join the manifest when their packet rules an R5 repair.
9. **MINOR: H4 describes T4A-C05 and the A5 population inconsistently.**
   - Where: `R5_TRANCHE_PROPOSAL.md:61-62,132,171`.
   - §4 shows T4A-C05 with no blocker ("—"), and Wave A lists that class wholly. But `DEL-10-05:SOW#CLM-002.r05` carries A5 and C7.
   - The A5 breakdown (81 + 6 + 8 + 3 + 3) sums to 101, not 99, because the two T9-C08 rows are counted twice.
   - Fix: correct the text.
10. **MINOR: the H3 class-token row count is off by one.**
    - Where: `R5_TRANCHE_PROPOSAL.md:169`.
    - The class tokens cover 604 rows. The figure 605 includes the DEL-13-02 R01 row, which carries only the T9-C04 token.
    - Fix: 604, or say "605 with the T9 items".
11. **MINOR: CFB-28 misses an OtherCorrections entry and has a stale note.**
    - Where: `CFB-28_user-docs.md:22,58`.
    - `DEL-11-03:SOW#CLM-004.r01` shows "(none recorded)", but its FIRM OtherCorrections entry reads "As CLM-010.s02". It should read "OC: as CLM-010.s02".
    - "No packet names it" is now superseded by U8, which is placed in C4. Fix: cite U8/C4.
12. **MINOR: CFB-29 does not point to H3 ER-20.**
    - Where: `CFB-29_local-store-export-redistribution.md:20`.
    - "Decide whether DEL-02-05 store tests count toward LFSP-REQ-011" is the same choice H3 ER-20 gives the reviewer for DEL-12-01 R01.
    - Fix: cite ER-20 as a dependency, or block on it.
13. **MINOR: `DEL-17-09:CONTEXT#description` is unblocked.**
    - Where: `CFB-54_plugin-sdk-admission.md:19`.
    - It says "implement the runtime SDK surface". The plugin runtime is B10, and sibling rows waiting on that runtime (CFB-29 REXC-REQ-014, CFB-54 STATUS R01) are blocked on B10.
    - Fix: add B10.
14. **MINOR: CFB-37, CFB-38 and CFB-39 do not reference B8.**
    - These briefs build comparison mapping and exporters for DEL-14-04/05.
    - B8 option (b) ports that work, and option (c) re-scopes DEL-14-04/05 (B8 §3 and §8).
    - Fix: add B8 as context.
15. **MINOR: two rows carry record, lifecycle-workflow or dependency-register work inside code-fix briefs.**
    - Where: `CFB-50_stress-neutral-records.md:21` ("run the owning semantic and dependency workflows") and `CFB-38_comparison-exporters.md:24` ("repoint the dependency record to DAG-010").
    - Neither carries the profile §8 guard ("Dependency graph changes" go to a separate owner-directed DAG rebuild) or a cross-reference to U2.
    - Fix: add the guard, or move both to H4.
16. **MINOR: CFB-52 and CFB-53 treat A7 as context only.**
    - The rows scoped to "plan-listed" families have no target set if A7 retires the deleted plan.
    - Fix: mark those rows A7, or state why an A7 ruling cannot change their scope.
17. **MINOR: H4 does not declare its path convention.**
    - Where: `R5_TRANCHE_PROPOSAL.md:45,81-83,219-220`.
    - It mixes project-relative paths (`execution/...`) with repository-relative ones (`projects/chirality-piping/init/`).
    - Fix: add the one-line convention the packets use ("freeze paths relative to `projects/chirality-piping/`").

**Outside scope (one line each)**
- CFB-07 marks `DEL-14-01:SOW#CLM-004` and `CLM-011.r04` only A2, but B8 §5 (line 79) asks for B8.
- `R4_GATE_INDEX.md:84-85` implies the C5 rows are marked in `CODE_FIX_ROWS.csv`. Only the C2 and C4 rows are (see finding 2).

**Items that need the owner rather than a repair**
- Where U1 (a wave-level reading of the DEL-05-02/05-03 AC-001 rows) and U2 (dependency-mirror sync: R5 record repair or DAG-path work) are placed, and how they are answered.
- Whether the two DEL-17-01/02 injection rows wait on A1 (the T8 NO_ACTION reading against A1's block). This is the substance behind finding 5.
- The terms of the R5 authorisation in H4 §9:
  - the A9 treatment for T4A-C02;
  - whether R5-ISSUED-DEL-01-01 runs, which follows A6;
  - the R6 rule for edits that have no claim key;
  - the conditional hold on T5B-C05 and T11 S-03/S-04 if C6 has not ruled.
- H2's UNASSIGNED U5 to U8 as placed in gate index §4. U7 (T6-C02 "narrow or defer by ruling" rows outside A2, B3, B7 and B8) has no packet yet.
- W3-PC-03 (desktop millimetre fallback) is worker-raised and not confirmed by a verifier. Only a code reading supports it; that reading agrees, but no test has been run.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This review makes no certification, code-compliance, professional-approval or engineering-acceptance claim.

END-OF-RETURN
