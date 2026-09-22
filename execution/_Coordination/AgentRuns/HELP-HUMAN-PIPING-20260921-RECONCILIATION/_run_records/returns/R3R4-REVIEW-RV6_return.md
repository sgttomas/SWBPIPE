# R3R4-REVIEW-RV6 return (verbatim)

Agent a9fda342d3de6b36f; scope RV6 (CFB-01..27, CODE_FIX_CANDIDATES.md); reviewed HEAD 23599b019.

VERDICT: FINDINGS

**Scope and HEAD.** RV6: H2 briefs CFB-01 to CFB-27, and `CODE_FIX_CANDIDATES.md`. Reviewed HEAD is `23599b0199525c3853c740f8dd782999974e549e`. The brief's sha256 matches the launch message (`ccdda3ae…51fd`). I edited nothing in the repository. My check scripts are in `{SCRATCH}/review/`.

**Items checked in depth.**
- **Whole file set.** I read all of CFB-01 to CFB-27.
- **`CODE_FIX_CANDIDATES.md`, recomputed from the CSVs:**
  - totals (380 + 6 = 386, no duplicate keys);
  - all 375 class rows, 17 T8 rows and 17 T12 rows are present;
  - every key exists in `CORPUS_CLAIMS.csv`;
  - ClassID and Authority match `CLASS_ASSIGNMENTS.csv`;
  - the blocker table (140 rows + 5 items), the 16/20/18 brief split and the 17 protected-subject briefs;
  - every row of the "Briefs by area" table (rows, class portions, blocker counts, protected flag);
  - every `BlockedOnPacket` token resolves to a real packet or to `H3_TOKEN_MAP` (ER-21, ER-22);
  - each CFB file lists exactly its own CSV keys;
  - the six W3 items match `W3_ASSESSMENT.md`.
- **Frozen-code citations, confirmed at `00115c719`:**
  - CFB-02: `MAINTAINERS.md:20-21,31-32`
  - CFB-03: `contributor_guide/index.md:37` (the INIT.md link target is absent)
  - CFB-05: the boundary Diagnostic has no code field
  - CFB-06: the fixture fails the schema on manifest_kind, id patterns and checksums
  - CFB-07: `service.py:28`
  - CFB-12: b43cc00c4 is in PR #787; VERIFICATION_REMOVED is the only such row
  - CFB-14: commands return `Result<…, String>`
  - CFB-17: `reportPackageRequest.ts:286,318`, `records.py:132-133`, `analysisRunCompatibility.ts:67`
  - CFB-21: `linter lib.rs:639-642`
  - CFB-22: `linter lib.rs:4,294,366`
  - CFB-25: `gate_adapter_runtime_dispatch`
  - CFB-26: panel lines 204 and 116 against `tauri.conf.json:27`
- **Ledger rows** read for CFB-02, 03, 05, 12, 14, 20, 26 and 27.
- **Rulings** checked: DEC-025, DEC-058, DEC-059, DEC-089.
- **Cross-checks** of packet "H2 should mark BlockedOnPacket" instructions in A1, A2, A6 and B8.

**Findings**

1. **ACTIONABLE — B8's block instruction was not applied to three CFB-07 rows.**
   - **Where:** `CFB-07_jcs-canonicalization.md:26-28` and `CODE_FIX_ROWS.csv:224,225,227`.
   - **Evidence:**
     - `B8_model-state-and-comparison.md:76-80` says H2 should mark `DEL-14-01:SOW#CLM-004`, `SOW#CLM-011.r01` and `SOW#CLM-011.r04` `BlockedOnPacket` B8.
     - H2 applied B8 only to CLM-011.r01 (CFB-34).
     - CLM-004 and CLM-011.r04 carry A2 only. CLM-005 carries A2;A1. All three have T12 view T12-C02 OWNER_DECISION, which CFB-07 itself displays.
     - As a result, a B8 ruling would not gate these rows.
   - **Smallest fix:** add B8 to those three rows in the CSV and in the CFB-07 table, and update the blocker counts (B8 would go from 6 to 9). Or Agent 0 records in `R4_GATE_INDEX.md` §5 why B8 does not apply.

2. **ACTIONABLE — CFB-14 goes ahead on an owner question that is still unassigned.**
   - **Where:** `CFB-14_command-result-envelopes.md:7,19-21,40-41`, and `CODE_FIX_ROWS.csv:6,15,83`.
   - **Evidence:**
     - The RemainingWork of all four rows begins "Decide whether…". The DEL-00-06 OC notes that exempting the commands would make the authority OWNER.
     - The Scope and the acceptance checks assume one answer: conform every command.
     - Three rows carry no block, so the index counts them as rows that "can go ahead separately".
     - This is U6, which `R4_GATE_INDEX.md:100` places with C1.
   - **Smallest fix:**
     - Mark REQ-03-02, REQ-06-03 and CLM-004.r05 `BlockedOnPacket` C1 (U6), or the token Agent 0 uses for U6.
     - Rewrite the Scope as "conform, or record the owner's exemption".
     - Make the acceptance checks depend on the ruling.

3. **ACTIONABLE — U7 says the rows are listed per brief, but no such list exists.**
   - **Where:** `R4_GATE_INDEX.md:101` and `CODE_FIX_CANDIDATES.md:289-293`.
   - **Evidence:**
     - U7's placement reads "listed per brief in `CODE_FIX_CANDIDATES.md`", but that file gives only examples.
     - Two of its three examples are already covered: DEL-16-02/16-03 by B3 (CFB-43), and DEL-15-03 by B7 (CFB-41).
     - My scan over the unblocked PROJECT_BASELINE, INVARIANT and FROZEN_CONTRACT rows found the true remainder: `DEL-03-01:SOW#CLM-011/REQ-03-01-007` and `DEL-03-01:SOW#production-and-verification-method-praxeology/VER-001` (CFB-09). The CFB-14 rows belong to U6.
   - **Smallest fix:** replace the examples with those two keys, and point U7 at them.

4. **MINOR — CFB-20 treats the V-7 row as review-gated code work, but it is the owner's scan.**
   - **Where:** `CFB-20_result-export-writer.md:19,41,51`.
   - **Evidence:** the V-7 OC says to run the DEC-058 scan over AC-5 "(owner act)", and DEC-058 makes the owner the scan owner. The brief blocks the row on H3 only, and its acceptance check has WORKING_ITEMS running a scan.
   - **Smallest fix:** say the V-7 remainder is an owner act under DEC-058, outside the code-fix scope. The scanner-tooling part stays with H3 review.

5. **MINOR — CFB-12 offers an option that DEC-059 fences off.**
   - **Where:** `CFB-12_solver-mechanics-verification.md:7,19,55`.
   - **Evidence:** the brief offers "add solver cargo tests to hosted CI" as an option. DEC-059 keeps the DEC-025 local sweep as the merge gate under every branch, and says any hosted merge-gating needs a new decision packet. DEC-025 also prohibits Actions on the private monorepo.
   - **Smallest fix:** state that the hosted-CI branch needs a new decision packet and is not executable under this brief.

6. **MINOR — Rows show "(none recorded)" even where a FIRM OtherCorrections entry exists.**
   - **Where:** `CFB-04_domain-units-schema-binding.md:22` (CLM-020), and `CFB-08_project-package-roundtrip.md` (CLM-019.r04, CLM-030.r03).
   - **Evidence:** CFB-04 CLM-020 carries a FIRM OC naming three files with literal DimensionId sets. The Scope uses it, but the table row gives no `OC:` marker.
   - **Smallest fix:** mark the row with `OC:` and the three-file summary.

7. **MINOR — The protected-subject lines list layers that are not protected.**
   - **Where:** CFB-05:40, CFB-10, CFB-11:39, CFB-12.
   - **Evidence:** these lines say "carry protected layers (BASELINE…)" and list BASELINE or VALIDATION. The protected set is INVARIANT tier or IP_DATA, CLAIMS or SECURITY. The counts are correct.
   - **Smallest fix:** word it as "INVARIANT tier or protected layers; row layers: …".

8. **MINOR — The index's list of open disagreements omits the T12 disagreements.**
   - **Where:** `CODE_FIX_CANDIDATES.md:226-236,297-302`.
   - **Evidence:** the two-views and "Disagreements left open" sections cover the 13 T8 rows only. Six H2 rows have a T12 view of OWNER_DECISION against the code-fix class route:
     - five T12-C02 DEL-14-01 rows;
     - `DEL-13-03:SOW#CLM-005.r04`.
   - The briefs show both views; the index does not.
   - **Smallest fix:** add those rows to the two-views table.

9. **MINOR — Narrowing alternatives on LOCAL_DESIGN rows have no stated holder or change path.**
   - **Where:** CFB-03:21,36 (REQ-11-05-07), CFB-27 (DEL-07-04 CLM-013 and CLM-022), and `CODE_FIX_CANDIDATES.md:105-108`.
   - **Evidence:** the index names the owner only for PROJECT_BASELINE and INVARIANT narrowing.
   - **Smallest fix:** add one line naming the holder of LOCAL_DESIGN narrowing (for example WORKING_ITEMS scope-change) and saying it runs as a separately authorized R5 repair.

**Outside my scope, noted in passing**
- `A2_json-hash-basis.md:71` lists 7 H2 hash-basis rows, but H2 blocks 12 rows on A2 (CFB-07). A2's list is short by five rows: both DEL-02-05 CONTEXT rows, CLM-021.r02, CLM-030.r05 and DEL-14-01 CLM-004.
- `B8…:79-80` says CLM-004 and CLM-011.r04 route with the DEC-009 cluster (A1). `A1…:86` does not include them.
- CFB-33 `DEL-13-03:SOW#CLM-005.r04` has T12-C03 OWNER_DECISION and no `BlockedOnPacket` (RV7).

**Needs the owner rather than a repair**
- U6: whether storage, rule-pack and library commands are exempt from the diagnostics envelope (CFB-14).
- U7: whether the DEL-03-01 material round-trip may be deferred by ruling (CFB-09).
- U5: narrowing the DEL-03-08 and DEL-06-01 INVARIANT restatements (CFB-10, CFB-15).
- The DEL-04-01 REQ-010 CI-gate reading, where it would go beyond DEC-025 and DEC-059 (CFB-12).
- The DEC-058 scan over AC-5 (CFB-20 V-7), which is the owner's act.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

END-OF-RETURN
