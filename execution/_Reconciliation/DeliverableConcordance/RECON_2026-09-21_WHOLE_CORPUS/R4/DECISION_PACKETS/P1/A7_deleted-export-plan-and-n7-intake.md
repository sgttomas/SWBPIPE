# A7 — The deleted export plan, and the missing N7 intake evidence

PROPOSAL for the R4 gate (R3 integration, TASK P1). It decides nothing and changes no row.

Conventions: `F:` is `{FREEZE}/projects/chirality-piping/` at `00115c719`. `RUN/` is the run folder.

## 1. Decision

Two related source questions:
1. **Export plan.** What becomes of `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` (source ID PLAN-EXPORT-INTEROP), a governing source for PKG-17 that was deleted with no archived copy: restore, re-point, or retire?
2. **N7 intake.** Does `HISTORICAL_N7_ACCEPTANCE_INTAKE_V1.json` exist outside the frozen tree, or should the two Remaining items that cite it be restated?

**Holder: OWNER.** If the owner re-scopes DEL-17-07 CF-001 as part of question 1, that part goes to **WORKING_ITEMS (workflow: scope-change)**.

## 2. Background

- **The plan was a governing source.** SCA-004's brief lists it as source basis (`F:execution/_ScopeChange/SCA-004_2026-05-18_0000/Brief.md:12`); SCA-004 created PKG-17 (`F:execution/_Decomposition/SOFTWARE_DECOMP.md:33`).
- **Deleted.** The file is absent at the freeze (no match for `EXPORT_FORMAT_INTEROPERABILITY_PLAN*`; `F:plans/` holds no export plan). The ledgers record its deletion by commit 349a2ab33 on 2026-06-03, titled "Retire DEV-001 and archive legacy coordination files" (T8-K5; from the ledgers and PKG-17 W-2, not re-derived with git). A commit message is not a ruling (A2 covers merged PRs).
- **Code still requires it.** `F:core/handoff/pcf_export/package.py:29`, `:422`, `:533` and `F:apps/desktop/src/features/pcf-export/PcfExportPanel.tsx:236` carry PLAN-EXPORT-INTEROP as a required source-basis reference, so every PCF export package names a document that does not exist (T8-K5; lines read by this task).
- **CF-001.** The DEL-17-07 PCF target version and profile remain "ruling TBD" while the code follows the proposal; the conflict rests on a section of the deleted plan (T4A-C06; T8-K5).
- **N7 intake.** `DEL-07-02:STATUS#remaining/R11` and `DEL-07-09:STATUS#remaining/R09` cite `HISTORICAL_N7_ACCEPTANCE_INTAKE_V1.json`; no file of that name exists in the freeze (0 matches, rechecked here). The companions `REVIEW_RETURN_V2.md` (`F:execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N7_FINAL_REVIEW/V2_BACKCHECK/REVIEW_RETURN_V2.md`) and `PRECOMMIT_PARENT_FAN_IN_V1.md` (same run folder) exist.

## 3. Options

**Question 1 (export plan)**, as in T8-K5 and T4B-C04:
1. **Restore** the plan from history as a governed source, with an owner adoption record.
   - Deliverables: citations stand; `Source_Basis_Register.md` entries point to the restored file.
   - Code: none (the source-basis refs become resolvable).
   - CF-001 keeps its basis and stays an open owner conflict.
2. **Re-point** the source ID and its citations to a successor, such as the DEL-17-01 registers and SCA-004 itself (the W2 assessment notes that SCA-004 names the export formats; DEL-17-04 implements the first solver-specific target, `DEL-17-01:SOW#CLM-005/F-17-01-001`).
   - Deliverables: R5 repair of the citations and registers.
   - Code: a small brief to change the required reference in `pcf_export` and `PcfExportPanel.tsx`.
   - CF-001: its basis must be re-derived; if that changes scope, a scope-change handoff.
3. **Retire** the source ID: remove it from the SOWs, `Source_Basis_Register.md` and the code's required refs; rows that relied on it for strategy or ordering (for example `DEL-17-02` REQ-026, which grants a citation permission against it) re-derive their basis.
   - Deliverables and code: as option 2, plus re-basing REQ-026-type rows.

**Routing reading (contested; T8-K5 against the sealed G3 reading).** The G1/G2 ledgers apply CP-08 (RECORD_DRIFT · LOCAL_DESIGN · RECORD · OWNER). G3 (DEL-17-07/08/09) reads the citations as stale pointers (STALE_SETUP_SPECIFICATION · BASIS_POINTER_STALE · NO). T8 proposes CP-08 fields for the plan element, with the disposition class set by F3 origin, and records an R3_OBSERVATION on the five G3 rows. The owner's choice among options 1–3 settles the repair either way; the reading changes labels only. This packet shows both and does not choose.

**Question 2 (N7 intake)**, as in T4B-C04 and T9-C03:
- **(a)** The owner or Agent 0 names where the intake record lives, if outside the tree; R5 re-points both citations.
- **(b)** It does not exist; R5 restates both Remaining items to cite the existing V2 review return and parent fan-in only.

## 4. Evidence and reliability

| Source | Shows | Reliability |
|---|---|---|
| SCA-004 brief `:12`; `SOFTWARE_DECOMP.md:33` | Plan was source basis for the PKG-17 amendment | Governing source |
| Freeze search; code lines above | Plan absent; code requires it | Frozen tree, checked by this task |
| Deletion commit and title | Deleted 2026-06-03 | Ledger records and PKG-17 W-2 (worker/verifier); not re-derived here (no git) |
| T8-K5, T4B-C04, T4A-C06 (ii), T9-C03 (`RUN/R3/TASKS/`) | 24 cluster rows; options; the routing split | R3 task proposals |
| N7 companions' paths | Two of three cited N7 records exist | Frozen tree, checked by this task |

## 5. Affected claims

**This packet's portion: 22 rows.**

| Class | Class rows | Portion | Filter |
|---|---|---|---|
| T4B-C04 Declarations citing evidence absent from the freeze (CP-08) | 17 | 17 (whole) | `ClassID == 'T4B-C04'` |
| T4A-C06 Pointer rows tied to open owner clusters (split class) | 14 | 5 | `ClassID == 'T4A-C06'` and DeliverableID in DEL-17-07/08/09 and key not ending `CONTEXT#architecture-basis-injection` |

Portion keys:
- T4B-C04, export plan (15, G1/G2, CP-08): `DEL-17-01:SOW#CLM-004.r01`, `DEL-17-01:SOW#CLM-005/F-17-01-001`, `DEL-17-01:SOW#CLM-005/F-17-01-002`, `DEL-17-01:SOW#CLM-005/F-17-01-003`, `DEL-17-01:SOW#CLM-005/F-17-01-004`, `DEL-17-01:SOW#CLM-005/F-17-01-005`, `DEL-17-01:SOW#CLM-005/F-17-01-006`, `DEL-17-01:SOW#CLM-027`, `DEL-17-02:SOW#CLM-007.r07`, `DEL-17-02:SOW#CLM-020/DEL-17-02-REQ-026`, `DEL-17-02:SOW#CLM-041`, `DEL-17-05:SOW#CLM-007.r01`, `DEL-17-06:SOW#CLM-004`, `DEL-17-06:SOW#CLM-008.r01`, `DEL-17-06:SOW#CLM-013.s01`.
- T4B-C04, N7 intake (2; also T9-C03): `DEL-07-02:STATUS#remaining/R11`, `DEL-07-09:STATUS#remaining/R09`.
- T4A-C06 export-plan group (ii) (5, G3): `DEL-17-07:SOW#CLM-008` (CONTESTED), `DEL-17-07:SOW#CLM-046/DEL-17-07-CF-001` (CONTESTED), `DEL-17-08:SOW#CLM-004`, `DEL-17-08:SOW#CLM-008`, `DEL-17-09:SOW#CLM-043`.

The other T4A-C06 rows belong to A1 (7, DEC-009) and A10 (2, release-label floor).

**T8 rows on other routes (both views shown; not claimed).** From `RUN/R3/T8_ROUTE_DISAGREEMENTS.csv` (cluster EXPORT_PLAN): these rows' own cause wins under the one-cause rule, and T8 says the plan citation still joins the owner decision.

| Key | Class (route) | T8 route |
|---|---|---|
| `DEL-17-07:SOW#CLM-004` | T4B-C05 (R5_RECORD_REPAIR) | OWNER_DECISION |
| `DEL-17-07:SOW#CLM-027` | T4B-C05 (R5_RECORD_REPAIR) | OWNER_DECISION |
| `DEL-17-09:SOW#CLM-007` | T4B-C05 (R5_RECORD_REPAIR) | OWNER_DECISION |
| `DEL-17-08:SOW#CLM-007/X-002` | T5A-C02 (R5_RECORD_REPAIR) | OWNER_DECISION |

H4 should mark them `BlockedOnPacket = A7` for the plan citation.

**Non-divergent row.** `DEL-17-07:SOW#CLM-014/DEL-17-07-REQ-015` is ALIGNED on `REQUIRED_SOURCE_BASIS_REFS`, one of which is PLAN-EXPORT-INTEROP (T8 observation 3). Options 2 and 3 change what it requires.

No `OtherCorrections` text on any row names the plan (script search: 0 hits for the plan name or source ID).

**Packages and deliverables.** PKG-17 (DEL-17-01, 02, 05, 06, 07, 08, 09) and PKG-07 (DEL-07-02, 07-09).

## 6. Risks

- **Undecided.** PKG-17 SOWs, source registers and every PCF export package cite a basis nobody can read; REQ-026 grants a permission against it; reviewers cannot check the basis for export strategy and target ordering; the CF-001 conflict stays open on a missing section. An accepted historical review basis (N7) rests on a missing artifact.
- **Option 1.** Re-adopts a document whose deletion title suggests retirement was intended; its content may be stale against SCA-004 and later rulings.
- **Option 2.** The successor may not carry the strategy and ordering content that some rows relied on.
- **Option 3.** Rows that drew requirements from the plan lose their basis and must be re-based, which may reopen scope.
- **N7 (b).** Loses the intake record as evidence if it does exist somewhere.
- **Any mechanical repoint before the ruling** would silently settle it (T4A-C06).

## 7. Recommended routing

No recommendation on the plan; owner's call. For N7, the evidence (two of three records present, the third absent everywhere in the tree) supports asking first whether the owner holds the intake record elsewhere; if not, option (b).

## 8. On-ruling mechanism

- **Option 1.** An owner adoption record for the restored plan (a DEC or register entry), then an R5 repair of the Source Basis Register and `_REFERENCES.md` entries that point to it.
- **Option 2 or 3.** An R5 record-repair tranche over the 22 portion rows and the four T8 rows' plan citations (Source Basis Register and `_REFERENCES.md` included); a CODE_FIX_CANDIDATE brief (H2) for the required source-basis reference in `core/handoff/pcf_export/package.py` and `PcfExportPanel.tsx`; if CF-001 is re-scoped, a scope-change handoff.
- **N7 (a) or (b).** An R4 confirmation, then an R5 repair of the two `_STATUS.md` Remaining items.

Nothing executes until the owner acts. R5 needs separate authorisation.

## 9. Dependencies

- **Depends on:** none.
- **Interacts with:** A10 (the DEL-00-07 export-format list against SCA-004 is a separate baseline question); A1 (same PKG-17 deliverables); H1 if CF-001 is re-scoped.
- **Blocks:** H4 repair of the 22 portion rows and the four T8 rows' plan citation; the H2 source-basis-reference brief; any repoint of CF-001.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). No certification, code-compliance, professional-approval or engineering-acceptance claim is made.
