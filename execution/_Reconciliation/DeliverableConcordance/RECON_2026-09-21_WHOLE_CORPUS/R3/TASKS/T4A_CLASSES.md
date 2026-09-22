# T4A — BASIS_POINTER_STALE classes (record and pointer hygiene)

T4A classifies the 781 rows of `R3/CORPUS_CLAIMS.csv` with `Divergent = YES` and effective CauseTag `BASIS_POINTER_STALE`. Every row has tier LOCAL_DESIGN and layer RECORD; 776 have BaselineClass NONE and 5 have ISSUED (all DEL-01-01). The population splits by what a repair would have to touch, not by package. Most of it (669 rows, C01 to C04) is mechanical re-pinning of decomposition revision, DAG and amendment pointers to the frozen basis (SOFTWARE_DECOMP revision 0.12, `execution/_DAG/_LATEST.md` naming DAG-010). No decision is needed for that beyond the R5 authorisation itself, except how to treat the D-41-era "current declaration" blocks (C02). C05 (86) holds pointers to removed files, renumbered sections and relocated code anchors, and each needs an existing successor target. Three small exception classes carry the substance: C06 (14 rows tied to the DEC-009 and export-plan clusters and the release-label floor), C07 (3 rows where a product document itself carries the dead pointer) and C08 (9 rows on the ISSUED DEL-01-01). This is a routing proposal only. It re-disposes no row, and it makes no claim about engineering adequacy or compliance (F-PIP-2; claims taxonomy per DEC-081).

Frozen-basis evidence used by every class: `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md:6` (`revision: 0.12`) and `projects/chirality-piping/execution/_DAG/_LATEST.md:3` (DAG-010, approved). Both are at the freeze.

## Class table

| ClassID | Name | Rows | Pkgs | Dels | Authority | Route |
|---|---|---|---|---|---|---|
| T4A-C01 | Keyed `_CONTEXT.md` setup-block drift (CS-01, CS-04, CS-06) | 198 | 18 | 100 | NONE | R5_RECORD_REPAIR |
| T4A-C02 | D-41-era "current declaration" blocks pinning rev 0.8 and DAG-007 | 213 | 17 | 69 | OWNER (treatment choice) | R5_RECORD_REPAIR |
| T4A-C03 | Other revision, graph and amendment pins (SOW, AB, CONTEXT `.sNN`) | 190 | 18 | 81 | NONE | R5_RECORD_REPAIR |
| T4A-C04 | `_CONTEXT.md` SURFACE rows restating the keyed pins (contested split) | 68 | 16 | 68 | REVIEW | R5_RECORD_REPAIR |
| T4A-C05 | Removed-file, renumbered-section and relocated-anchor pointers | 86 | 14 | 43 | NONE | R5_RECORD_REPAIR |
| T4A-C06 | Pointer rows tied to open owner clusters (DEC-009, export plan, release-label floor) | 14 | 2 | 8 | OWNER | OWNER_DECISION |
| T4A-C07 | Product documents carrying the dead or stale pointer | 3 | 2 | 2 | NONE | CODE_FIX_CANDIDATE |
| T4A-C08 | Pins on the ISSUED DEL-01-01 | 9 | 1 | 1 | OWNER | OWNER_DECISION |
| | **Total** | **781** | | | | |

Assignment order, applied by script and first match wins: C07 (effective PARTIALLY_IMPLEMENTED), then C08 (LifecycleState ISSUED), then C06 (the named keys), then C01 (CS-01/04/06), then C02 (CP-03, or a D-41 "current declaration" summary), then C04 (whole-file `CONTEXT` SURFACE key), then C05 (a non-revision pointer in the summary, Notes or RemainingWork, with 12 hand-checked overrides listed under Coverage), and C03 for the rest.

---

### T4A-C01 — Keyed `_CONTEXT.md` setup-block drift

- **Description.** These are the canonical keyed rows on setup-era `_CONTEXT.md` files. CS-01 is the Decomposition Reference ("Accepted Revision 0.7, current_basis"), CS-04 the Architecture Basis Injection pin, and CS-06 DRIFT the Package Reference assigned-scope-item lists, which omit items that revision 0.12 adds (for example SOW-064, SOW-065, SOW-076/077). CANONICAL_SITUATIONS.md says a catch-up repairs these "with no decision (C3)". The CS-04 rows cover the pin only. Readiness, TBD and baseline parts sit on `.sNN` rows in other partitions.
- **Signature.** BASIS_POINTER_STALE · LOCAL_DESIGN · STALE_REVIEW_OR_EVIDENCE · NONE · RECORD. CS-01 100, CS-04 83, CS-06 15.
- **Population.** 198 rows, 18 packages (PKG-00 to PKG-17), 100 deliverables. It covers every deliverable except DEL-01-01 (in C08) and DEL-07-09. Waves: W1 32, W2 59, W3 107.
- **Owning authority.** NONE. The fix is mechanical, and the only gate is the R5 authorisation (D-73).
- **Route.** R5_RECORD_REPAIR.
- **On-ruling mechanism.** An R5 ruling authorises one bounded catch-up tranche across the `_CONTEXT.md` files, through the project's ordinary deliverable-edit path (chirality-change PR). Set Decomposition Reference and the Architecture Basis Injection revision to 0.12. Refresh Package Assigned Scope Items from the revision 0.12 package table. Do not touch sibling `.sNN` content, which belongs to other classes.
- **Risk if unrepaired.** Low per row but broad. Any agent brief sealed from these contexts inherits revision 0.7 as "current_basis", and scope-item lists under-report package scope.
- **Exceptions named.** Three are CONTESTED: DEL-03-01, DEL-03-02 and DEL-03-03 `CONTEXT#architecture-basis-injection`. The verifier reads a missing `.s02` row for a Still-TBD item since ruled (DEC-017) as required. The key does not exist, and repair of this row does not settle it. Four are FIELD (VerificationClass and DecisionBasis corrections only): DEL-10-04 and DEL-10-05 `CONTEXT#decomposition-reference` and `#architecture-basis-injection`.
- **Representative keys.**
  - `DEL-08-03:CONTEXT#decomposition-reference`: `…/DEL-08-03_…/_CONTEXT.md:40` "Accepted Revision: 0.7" and `:52` revision 0.7.
  - `DEL-05-01:CONTEXT#architecture-basis-injection`: `…/DEL-05-01_…/_CONTEXT.md:51` pins revision 0.7.
  - `DEL-07-01:CONTEXT#package-reference`: the ledger Notes say revision 0.12 adds SOW-076 and SOW-077 to the PKG-07 list.

### T4A-C02 — D-41-era "current declaration" blocks

- **Description.** These are the CP-03 blocks (for example "D-41 R5 T7 PDU-055 current declaration"), mostly in `ScopeOfWork.md` (208 rows). Five sit in MEMORY or STATUS: DEL-04-06 and DEL-11-04 `MEMORY`, DEL-09-05 and DEL-11-01 `MEMORY.s01`, and DEL-12-01 `STATUS#d-41-r5-t7-pdu-055-current-declaration`. Each names SOFTWARE_DECOMP revision 0.8 and DAG-007 as current authority and delegates residuals to `_STATUS.md` Remaining. Ledgers give them CP-02 fields. They record the delegation clause as not relied on (A4), and they note that the no-closure sentence is still accurate. Forty-four rows carry CanonicalSituation CP-02 but are the same blocks. Examples are DEL-17-07/08/09 CLM-002 and CLM-010, whose Notes say "CP-03 block; treatment per CP-02". They are classed here by content.
- **Signature.** BASIS_POINTER_STALE · LOCAL_DESIGN · STALE_REVIEW_OR_EVIDENCE · NONE · RECORD. CP-03 169, CP-02 44.
- **Population.** 213 rows, 17 packages (not PKG-00), 69 deliverables. Waves: W1 27, W2 25, W3 161.
- **Owning authority.** OWNER, for a single treatment choice. After that the work is mechanical.
- **Route.** R5_RECORD_REPAIR.
- **Owner decision (inside the R5 ruling).** Ledger RemainingWork gives two options:
  - **(a)** re-pin each declaration to revision 0.12 and DAG-010, for example `DEL-04-05:SOW#CLM-010` and `DEL-14-05:SOW#CLM-009`;
  - **(b)** retire the declaration at the next SOW catch-up, for example `DEL-08-03:SOW#CLM-002` ("or retire it at the next SOW catch-up").

  Option (a) keeps the blocks' Remaining delegation live, but Piping no longer selects work from Remaining (C9, 2026-09-19). Option (b) removes text that the ledgers already treat as not relied on. For the five MEMORY and STATUS units, which are history surfaces, (b) should mean marking the undated declaration as superseded, not deleting it.
- **On-ruling mechanism.** The R5 ruling names (a) or (b) for all 213 rows at once. A catch-up tranche then edits the `ScopeOfWork.md` files (plus the five MEMORY and STATUS units) through the chirality-change PR path, one PR per package or per tranche. No sibling disposition changes (C1).
- **Risk if unrepaired.** Readers take revision 0.8 and DAG-007 as the governing authority, and the Remaining delegation points to a selection mechanism that is no longer used.
- **Exceptions named.** Six are FIELD: DEL-10-04 CLM-002, CLM-010 and CLM-025, and DEL-10-05 CLM-008, CLM-016 and CLM-023. They correct VerificationClass only.
- **Representative keys.**
  - `DEL-04-05:SOW#CLM-002`: `…/DEL-04-05_…/ScopeOfWork.md:26-28`, the D-41 R5 T7 PDU-055 heading.
  - `DEL-14-05:SOW#CLM-009`: the Notes record the pins to 0.8 and DAG-007.
  - `DEL-17-07:SOW#CLM-002`: CP-02-tagged, same block.

### T4A-C03 — Other revision, graph and amendment pins

- **Description.** These are revision, graph and amendment pins outside the keyed `_CONTEXT.md` blocks and the D-41 declarations. They sit in SOW frontmatter (`decomposition_basis` at a revision 0.8 commit, often a minted `SOW.s01`), SOW reference lists and prerequisites (revision 0.7, DAG-006), identification tables, dependency-mirror labels (DAG-002/005/006), PKG-00 ArchitectureBasis headers and currency blocks (revision 0.9 with DAG-007), and `.sNN` Architecture Basis rows whose amendment list stops at SCA-004. Also here are the whole-file SOW and AB SURFACE rows whose common defect is the frontmatter pin. Every Note says the referenced files otherwise resolve.
- **Signature.** BASIS_POINTER_STALE · LOCAL_DESIGN · STALE_REVIEW_OR_EVIDENCE · NONE · RECORD. All CP-02.
- **Population.** 190 rows, 18 packages, 81 deliverables. Waves: W1 16, W2 79, W3 95.
- **Owning authority.** NONE.
- **Route.** R5_RECORD_REPAIR.
- **On-ruling mechanism.** This is the same R5 catch-up tranche as C01, applied to `ScopeOfWork.md`, `ArchitectureBasis.md` and the four `_CONTEXT.md` Architecture Basis rows with no revision pin. Re-pin to revision 0.12 and DAG-010. Where a block names an amendment list, name the accepted amendments through the latest SCA (`execution/_ScopeChange/_LATEST.md`), as the RemainingWork of `DEL-01-04:CONTEXT#architecture-basis-injection.s02` proposes. Edits go through chirality-change PRs.
- **Risk if unrepaired.** It is the same as C01. Also, two graph generations (DAG-006 and DAG-007) are cited as "approved" in different files of the same package (`DEL-13-01:SOW#CLM-004.r04`: the SOW names DAG-006, `_DEPENDENCIES.md` names DAG-007, and DAG-010 is current).
- **Exceptions named.** Two are OBSERVED: DEL-00-03 `AB.s01` and DEL-00-04 `AB`, where the FindingGroup is empty (cosmetic). Two are RESOLVED_PAIR: DEL-17-01 and DEL-17-02 `CONTEXT#architecture-basis-injection`, where AuthorityNeeded NO is confirmed and no DEC-009 `.sNN` is needed.
- **Representative keys.**
  - `DEL-00-05:AB`: the header declares revision 0.9 as authority basis (ledger Notes, header line 6).
  - `DEL-17-01:CONTEXT#architecture-basis-injection`: `…/DEL-17-0x/_CONTEXT.md` Scope Changes line "SCA-001 … as amended by SCA-003 and SCA-004". Revision 0.12 says it is amended through SCA-008 (the same text is at `…/DEL-17-08_…/_CONTEXT.md:47`).
  - `DEL-13-01:SOW#CLM-011.r07`: the DAG-006 authority label, where DAG-010 is current.

### T4A-C04 — `_CONTEXT.md` SURFACE rows restating the keyed pins

- **Description.** These are the whole-file `DEL-xx-yy:CONTEXT` SURFACE rows. They are disposed non-aligned because the file names revision 0.7 as `current_basis`, cites PKG-00 at SEMANTIC_READY, or (PKG-00) points readers to revision 0.9. That same defect is already carried by the keyed CS-01 and CS-04 rows in C01. Across the corpus, workers split on this surface: of the 102 `CONTEXT` SURFACE rows, 69 are non-aligned with BASIS_POINTER_STALE (68 here plus DEL-01-01 in C08), 30 are ALIGNED, and 3 are STALE_SETUP_SPECIFICATION with DOC_BEHIND_CODE (from a `CORPUS_CLAIMS.csv` count). The verifiers flagged the split as CONTESTED and said batch mode cannot see it. The C1 common-defect rule arguably supports ALIGNED, with the pins left to the keyed rows.
- **Signature.** BASIS_POINTER_STALE · LOCAL_DESIGN · STALE_REVIEW_OR_EVIDENCE (65) or STALE_SETUP_SPECIFICATION (3: DEL-06-01/02/03, a setup-era origin under F3) · NONE · RECORD. CanonicalSituation CP-02 53, empty 15.
- **Population.** 68 rows, 16 packages (no PKG-09 or PKG-16), 68 deliverables. Waves: W1 8, W2 25, W3 35.
- **Owning authority.** REVIEW, for the consistency reading. The file repair is the same as C01.
- **Route.** R5_RECORD_REPAIR. The C01 repair of the same files clears these rows, so they need no separate edit. Agent 0 should read the ALIGNED and non-aligned groups as one situation in `R3_SYNTHESIS.md`, as the verifier asked. No value is changed here.
- **On-ruling mechanism.** The C01 tranche covers these files. If the R5 ruling also refreshes the file-level basis statements (the setup-era SCA-001 and SEMANTIC_READY framing noted on the DEL-06-0x rows), that part depends on the SR-1 cluster outcome (T8/T9). It is not a pointer repair.
- **Risk if unrepaired.** No product risk beyond C01. The risk is to counts: the whole-file rows count the same pin defect a second time on 68 deliverables but not on 30 others. Any per-deliverable divergence figure inherits that asymmetry.
- **Exceptions named.** Six are CONTESTED: DEL-05-01, DEL-05-02, DEL-05-03, DEL-10-01, DEL-10-02 and DEL-10-03 `CONTEXT` (the G1/G2 split in each package). Two are OBSERVED: DEL-00-03 and DEL-00-04 `CONTEXT` (empty FindingGroup, cosmetic).
- **Representative keys.**
  - `DEL-10-01:CONTEXT`: the Notes say the file names revision 0.7 as current_basis. The contested note records that the CS-01 row already carries the pin.
  - `DEL-00-03:CONTEXT`: `…/DEL-00-03_…/_CONTEXT.md:6` points readers to revision 0.9.
  - `DEL-06-01:CONTEXT`: STALE_SETUP_SPECIFICATION, a setup-era basis statement (origin 7bee9ae41).

### T4A-C05 — Removed-file, renumbered-section and relocated-anchor pointers

- **Description.** These pointers name a target that no longer exists or has moved, with no behaviour change. The groups are:
  - project `INIT.md`, removed on 2026-07-04 by 9c4caf8fd (46 rows);
  - SPEC section numbers shifted by one (for example section 7 cited for the warning-class table, which is section 8), and PRD v0.2/13.x citations after the PRD v0.4 renumbering;
  - `skills/*/SKILL.md` paths that are absent;
  - PKG-00 AB `§8.4`, which does not exist;
  - OI-015 and SOW-074 source notes that never said what is cited;
  - repository-root paths predating the 2026-05-18 migration;
  - `App.tsx::handle*` anchors moved to `features/workspace/workspaceSession.ts` by PR #803 (the DEL-07-09 contract rows).

  Many rows also carry a revision pin. They are here because a non-revision pointer needs a successor target that a re-pin alone does not give.
- **Signature.** BASIS_POINTER_STALE · LOCAL_DESIGN · STALE_REVIEW_OR_EVIDENCE (57) or STALE_SETUP_SPECIFICATION (29) · NONE · RECORD. CP-02 83, empty 3.
- **Population.** 86 rows, 14 packages, 43 deliverables. Waves: W1 17, W2 5, W3 64.
- **Owning authority.** NONE. Choosing a successor is editorial, but it must name a path that exists at the time of the repair.
- **Route.** R5_RECORD_REPAIR.
- **On-ruling mechanism.** The R5 catch-up tranche makes per-row repoints. For INIT.md, the successor must be an existing surface. At the freeze these are `projects/chirality-piping/AGENTS.md` and `loop/LOOP_INIT.md`, and `init/` holds only `dev-loop-init-prompt.md` and `taskmgmt-init-prompt.md`. For SPEC and PRD, re-point to the current section numbers. For DEL-07-09, repoint the anchors to `workspaceSession.ts`, and add the undo/redo branch per the `common-executable-route.s01` RemainingWork. Edits go through chirality-change PRs.
- **Risk if unrepaired.** Readers following source-basis cells hit dead links. Requirement rows appear to rest on a removed authority file, although the Notes find the substance held in current documents.
- **Exceptions named.**
  - Five CONTESTED rows turn on the F3 class split (STALE_SETUP_SPECIFICATION or STALE_REVIEW_OR_EVIDENCE for a setup-era file pointer): DEL-06-02 `SOW.s02` and `SOW#CLM-008.r01`, DEL-06-03 `SOW#CLM-007.r01`, DEL-09-03 `SOW#CLM-007.r01` and DEL-13-02 `SOW#CLM-017.r03`.
  - One CONTESTED row turns on its cause (BASIS_POINTER_STALE, RECORD_DRIFT or DOC_BEHIND_CODE): DEL-10-05 `SOW#CLM-002.r05`.
  - Four FIELD rows: DEL-10-04 `SOW#CLM-007` and `SOW#CLM-014.s01`, and DEL-10-05 `SOW#CLM-003.r06`, correct VerificationClass and DecisionBasis. DEL-11-01 `SOW#CLM-007` corrects its RemainingWork, which names `init/init-prompt.md`, a path that does not exist.
  - One OBSERVED row: DEL-11-01 `SOW#CLM-005.r02` (paired with an ALIGNED sibling on the same pointer).

  None of these changes the route.
- **Representative keys.**
  - `DEL-11-04:SOW#CLM-007.r01`: INIT.md is absent at the freeze (`projects/chirality-piping/INIT.md` does not exist).
  - `DEL-07-09:ROWS/ROW-9`: `…/DEL-07-09_…/Palette_Operation_Routing.md:13` still names `App.tsx::handleToolkitCommand`.
  - `DEL-09-05:SOW#CLM-012/RQG-002`: `…/DEL-09-05_…/ScopeOfWork.md:147` cites `docs/SPEC.md#4.5`, which is now Plugin and extension domain contracts according to the ledger Notes.

### T4A-C06 — Pointer rows tied to open owner clusters

- **Description.** On these rows the pointer is stale, but the substance behind it is an open owner question, so a record repair alone would pre-empt a ruling. There are three groups:
  - **(i) DEC-009 (7 rows).** DEL-17-03 to DEL-17-09 `CONTEXT#architecture-basis-injection`. The resolved baseline restates DEC-009 (Rust core services), while the PCF and review-geometry exports ship as Python core modules plus TypeScript packet builders. WEAK resolutions move DEL-17-04/05/06 from ALIGNED, and DEL-17-07/08/09 from IMPLEMENTED_DIFFERENTLY · AUTHORITY_UNCLEAR, to CP-02 main-row values. They also direct a DEC-009 `.sNN` to be split off (IMPLEMENTED_DIFFERENTLY · PROJECT_BASELINE · OWNER, cause per Agent 0's corpus-wide settlement). That `.sNN` does not yet exist.
  - **(ii) Export plan (5 rows).** DEL-17-07 `SOW#CLM-008` and `SOW#CLM-046/DEL-17-07-CF-001`, DEL-17-08 `SOW#CLM-004` and `SOW#CLM-008`, and DEL-17-09 `SOW#CLM-043` cite `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md`, which was deleted on 2026-06-03 (349a2ab33). Only the DEL-17-07 pair is marked CONTESTED. The verifier reading is CP-08 · RECORD_DRIFT · OWNER. The DEL-17-08/09 rows are the same situation and are placed here by hand.
  - **(iii) Release-label floor (2 rows).** DEL-09-05 `SOW#CLM-012/RQG-007` and `SOW#CLM-029` cite VALIDATION_STRATEGY section 4, which lost its engineering-beta minimum condition on 2026-06-07 (c8748a04a). The replacement floor is open under PB-TBD-003.
- **Signature.** BASIS_POINTER_STALE · LOCAL_DESIGN · STALE_REVIEW_OR_EVIDENCE (7) or STALE_SETUP_SPECIFICATION (7) · NONE · RECORD. AuthorityNeeded is OWNER on 5 rows (DEL-17-07/08/09 ABI, DEL-09-05 ×2). The WEAK resolutions for DEL-17-07/08/09 say AuthorityNeeded should be NO on the main row, with OWNER moving to the new `.sNN`. That correction is in OtherCorrections and is not applied.
- **Population.** 14 rows, 2 packages (PKG-09, PKG-17), 8 deliverables (DEL-09-05, DEL-17-03 to DEL-17-09). All W3.
- **Owning authority.** OWNER.
- **Route.** OWNER_DECISION, through the T8 clusters DEC-009 and Export plan (W3_ASSESSMENT.md, contested-cluster table) and PB-TBD-003.
- **Owner decisions and options (as they stand in the evidence).**
  - **DEC-009.** Either accept Python core export builders and TypeScript packet builders under DEC-009, or require them to move to Rust core services (RemainingWork on DEL-17-07/08). The cause of the split `.sNN` is POSSIBLE_DEFECT or AUTHORITY_UNCLEAR, per the W3 cluster. Once ruled, the pointer part of each ABI row is an ordinary C03 amendment-list repoint.
  - **Export plan.** Either route the deleted-plan citations as OWNER items (CP-08 · RECORD_DRIFT, the G1/G2 reading) or as stale pointers to re-source (the sealed G3 reading). For CF-001, the PCF target version and profile remain "ruling TBD" while the code follows the proposal.
  - **Release-label floor.** Settle the floor under PB-TBD-003, then align RQG-007 and checklist section 8. The CONTESTED resolution proposes the cause AUTHORITY_UNCLEAR, with tier INVARIANT or PROJECT_BASELINE (two options).
- **On-ruling mechanism.** Each ruling goes into the R4 decision packets. After it:
  - R5 repairs the pointers under the chosen reading;
  - a DEC-009 ruling that requires a Rust port becomes a CODE_FIX_CANDIDATE or ENGINEERING_AUTHORITY brief;
  - an export-plan ruling that re-scopes CF-001 goes to the scope-change workflow;
  - the PB-TBD-003 ruling goes through the owner's decision record (DEC), then a record repair.
- **Risk if unrepaired.** A mechanical repoint here would silently settle an owner question. For example, re-sourcing CF-001 away from the deleted plan would lose the open conflict, and repinning the ABI block would hide the DEC-009 divergence that the not-yet-split `.sNN` should carry.
- **Exceptions named.** Four are CONTESTED (DEL-09-05 ×2, DEL-17-07 CLM-008 and CF-001) and seven WEAK (the DEL-17-03 to 09 ABI rows). Every row in this class is an exception.
- **Representative keys.**
  - `DEL-17-08:CONTEXT#architecture-basis-injection`: `…/DEL-17-08_…/_CONTEXT.md:47`.
  - `DEL-17-07:SOW#CLM-008`: `…/DEL-17-07_…/ScopeOfWork.md:132` PLAN-EXPORT-INTEROP row. The plan is absent at the freeze.
  - `DEL-09-05:SOW#CLM-012/RQG-007`: `…/DEL-09-05_…/ScopeOfWork.md:106` cites `docs/VALIDATION_STRATEGY.md` release gate.

### T4A-C07 — Product documents carrying the dead or stale pointer

- **Description.** On these rows the divergence is in a product document the deliverable delivers, not only in its record, so the ledgers disposed them PARTIALLY_IMPLEMENTED. There are two:
  - The contributor guide (DEL-11-05) starts its first-hour path at a dead link: `projects/chirality-piping/docs/contributor_guide/index.md:37` links `../../INIT.md`, which is absent at the freeze.
  - `projects/chirality-piping/governance/MAINTAINERS.md:20-21,31-32` names revision 0.7 and DAG-007 as current authority (DEL-01-01).
- **Signature.** BASIS_POINTER_STALE · LOCAL_DESIGN · PARTIALLY_IMPLEMENTED · NONE · RECORD.
- **Population.** 3 rows, 2 packages (PKG-01, PKG-11), 2 deliverables: `DEL-11-05:SOW#CLM-028`, `DEL-11-05:SOW#CLM-011/REQ-11-05-01` and `DEL-01-01:SOW#CLM-024.s01`.
- **Owning authority.** NONE for the guide. For MAINTAINERS.md, OWNER may apply: DEL-01-01 is ISSUED (see C08 and the R3 observations), although this row carries BaselineClass NONE.
- **Route.** CODE_FIX_CANDIDATE: a bounded documentation-file change brief, not executed.
- **On-ruling mechanism.** A candidate brief in `R3/CODE_FIX_BRIEF_CANDIDATES/` repoints guide step 1 to an existing bootstrap entry (`AGENTS.md` or `loop/LOOP_INIT.md`) and updates the two MAINTAINERS.md authority lines to revision 0.12 and DAG-010. It is executed only under R5 authorisation through a chirality-change PR. If the owner treats MAINTAINERS.md as ISSUED content, it goes through the ISSUED change path with C08. The DEL-11-05 SOW rows then take the same repoint (C05 mechanism).
- **Risk if unrepaired.** New contributors and maintainers are sent to a missing file or a superseded authority. This is the only place in T4A where the stale pointer reaches a user-facing document.
- **Exceptions named.** None.
- **Representative keys.** All three rows are listed above, with their freeze evidence.

### T4A-C08 — Pins on the ISSUED DEL-01-01

- **Description.** These are revision, graph and package-scope pins on DEL-01-01, whose lifecycle is ISSUED. Five SOW rows record a CANONICAL_DEPARTURE: BaselineClass ISSUED and AuthorityNeeded OWNER, because an edit to ISSUED text needs the owner's ISSUED change path. These are CLM-002.r09, CLM-002.r10, CLM-007, CLM-013.s01 and CLM-017. Four `_CONTEXT.md` rows on the same ISSUED deliverable keep the CS defaults (BaselineClass NONE, AuthorityNeeded NO): `CONTEXT`, `#package-reference` (CS-06, SOW-064 missing), `#decomposition-reference` (CS-01) and `#architecture-basis-injection` (CS-04).
- **Signature.** BASIS_POINTER_STALE · LOCAL_DESIGN · STALE_REVIEW_OR_EVIDENCE · ISSUED (5) or NONE (4) · RECORD.
- **Population.** 9 rows, PKG-01, DEL-01-01. W2.
- **Owning authority.** OWNER.
- **Route.** OWNER_DECISION.
- **Owner decision.** Whether to open the ISSUED change path for DEL-01-01 to repair its pins, which were true at issuance. The options in the evidence:
  - **(a)** repair through the ISSUED change path, per the RemainingWork on the five SOW rows;
  - **(b)** leave the issued text as issued and record the frozen basis elsewhere. No ledger proposes this; it is listed only as the do-nothing alternative.

  The same decision should cover the four `_CONTEXT.md` rows, so one ISSUED deliverable is not edited in two ways.
- **On-ruling mechanism.** Under (a), an R4 owner ruling authorises an ISSUED change for DEL-01-01 (the ruling-item-4 ISSUED path in CONVENTIONS C6(d)). R5 then makes the text repair through that path, together with the MAINTAINERS.md brief (C07) if the owner includes it.
- **Risk if unrepaired.** It is the same as C01, on the one deliverable that is formally issued. A non-owner catch-up tranche covering C01 could also edit DEL-01-01's `_CONTEXT.md` by default, since those rows carry NONE/NO.
- **Exceptions named.** None have resolutions. The mixed BaselineClass is recorded as an R3 observation.
- **Representative keys.**
  - `DEL-01-01:SOW#CLM-002.r09`: `…/DEL-01-01_…/ScopeOfWork.md:39` Decomposition Basis revision 0.7.
  - `DEL-01-01:SOW#CLM-002.r10`: `…/ScopeOfWork.md:40`, DAG-006 approved active graph authority.
  - `DEL-01-01:CONTEXT#decomposition-reference`: CS-01, BaselineClass NONE.

## Coverage

- **Population.** `CORPUS_CLAIMS.csv` has 9,889 claim rows. The filter `Divergent == 'YES' and CauseTag == 'BASIS_POINTER_STALE'` (effective values) gives 781 rows.
- **Output.** `T4A_CLASSES.csv` has 781 body rows plus `#END` (last column 781).
- **Checks.** A script check confirms:
  - the set of ClaimKeys in the CSV equals the filtered set;
  - there are no duplicates;
  - every DeliverableID matches the corpus row;
  - every ClassID is one of T4A-C01 to C08.
- **Class counts.** C01 198, C02 213, C03 190, C04 68, C05 86, C06 14, C07 3, C08 9; total 781.
- **Ledger join.** All 781 keys were found exactly once in the sealed `WAVES/W*/PKG-*/DEL-*/DEL-*_forward.csv` files, with `superseded_*` excluded.
- **Overrides.** The C05/C03 split uses a text pattern with 12 hand-checked overrides:
  - to C05: DEL-15-02 `SOW#CLM-004.r04`, DEL-03-05 `SOW#CLM-016`, DEL-08-02 `SOW#CLM-007`, DEL-15-01 `SOW#CLM-004`, DEL-11-05 `CONTEXT#anticipated-artifacts`, DEL-07-03 `SOW#CLM-024` and DEL-17-07 `SOW#CLM-008/DEL-15-02`;
  - to C03: DEL-02-01 `SOW#CLM-003`, DEL-12-04 `SOW#CLM-008` and `SOW#CLM-012`, DEL-06-04 `SOW` and DEL-01-04 `CONTEXT#architecture-basis-injection.s02`.

  The DEL-17-08/09 export-plan rows were placed in C06 by hand.
- **Resolutions.** 47 rows carry resolutions: CONTESTED 19, FIELD 14, WEAK 7, OBSERVED 5, RESOLVED_PAIR 2. Each is named in its class. Draft resolution files (`RESOLUTIONS_DRAFT*.csv`, `RESOLUTIONS_MERGED_DRAFT.csv`) were not read.
- **Limits.** C03 and C05 cannot be separated purely by field values. The split is by what the repair touches. It does not change any route except C06, C07 and C08.

## R3 observations

1. **DEL-01-01 is recorded two ways (C08, C07).** The same ISSUED deliverable carries BaselineClass ISSUED and AuthorityNeeded OWNER on 5 SOW pin rows, but NONE/NO on its 4 `_CONTEXT.md` pin rows and on the MAINTAINERS.md row. Evidence: the ledger Notes on `DEL-01-01:SOW#CLM-002.r09` (CANONICAL_DEPARTURE) and `DEL-01-01:CONTEXT#decomposition-reference` (CS-01 default). This is not a correction, but T9's ISSUED-lifecycle review and the R4 packet should treat all DEL-01-01 edits as one ISSUED-path decision.
2. **The F3 class split on setup-era file pointers (C05) affects labels only.** The same INIT.md pointer situation is STALE_REVIEW_OR_EVIDENCE on 26 C05 rows (for example DEL-07-01 `SOW#CLM-007.r03`, DEL-15-04 `SOW#CLM-007`) and STALE_SETUP_SPECIFICATION on 20 (for example DEL-11-04 `SOW#CLM-007.r01`, DEL-12-01 `SOW#CLM-007`). 26 C05 rows carry a CANONICAL_DEPARTURE citing F3. Routing is identical either way, so an R4 consistency note is enough. No repair depends on it.
3. **A successor-path error recurs.** Ledger Notes on 5 rows say INIT.md was "superseded by init/init-prompt.md": DEL-07-01 `SOW#CLM-007.r03`, DEL-06-04 `SOW#CLM-006`, DEL-09-04 `SOW#CLM-005`, DEL-11-01 `SOW#CLM-007` and DEL-11-02 `SOW#CLM-007.r01`. That file does not exist at the freeze (`projects/chirality-piping/init/` holds `dev-loop-init-prompt.md` and `taskmgmt-init-prompt.md`). The W-level FIELD resolution caught this only on DEL-11-01 `SOW#CLM-007`. Any R5 brief must not copy the path from those Notes.
4. **The DEL-17-04/05/06 ABI rows may carry an ill-fitting cause (C06).** These rows took CP-02 values from WEAK resolutions, although their own Notes say "No revision pin, so CS-04 does not apply" and judge the basis statement accurate. Their only pointer-like defect is the SCA amendment list, which DEL-17-01/02 record as BASIS_POINTER_STALE. The substantive divergence is DEC-009, which belongs on the `.sNN` the resolutions direct and which does not exist yet. The effective values are left as they are.
5. **The CONTEXT SURFACE asymmetry (C04) affects counts corpus-wide.** 30 ALIGNED and 69 non-aligned rows describe the same situation. Any per-deliverable or per-package divergence figure in R3 includes this double count for 69 deliverables.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
