# A5 — PKG-00 SEMANTIC_READY: target lifecycle state and workflow

PROPOSAL for the R4 gate (R3 integration, TASK P1). It decides nothing and changes no row or state.

Conventions: `F:` is `{FREEZE}/projects/chirality-piping/` at `00115c719`. `RUN/` is the run folder.

## 1. Decision

Direction 8 says the PKG-00 "SEMANTIC_READY" status is outdated and "should be advanced upon my subsequent approval in the proper workflow". The owner decides (i) the target lifecycle state for the eight PKG-00 deliverables and the workflow that moves them, and (ii) how the 82 injection statements (and the other review and lifecycle-state declarations counted in SRE-5) are then restated.

**Holder: OWNER**, acting through the lifecycle workflow. The SR-1 **cause** reading (RECORD_DRIFT or SCOPE_REDIRECTED_BY_RULING) is **not** decided here; it belongs to C7. Under T5B-C04 and T4B-C03 the cause changes the label only, not the option.

## 2. Background

- **Lifecycle history.** DEL-00-01 `_STATUS.md` shows SEMANTIC_READY on 2026-04-30, CHECKING on 2026-06-04, and IN_PROGRESS on 2026-07-11 by the D-40 rebaseline under the TYPES §9 regime model (`F:execution/PKG-00_…/1_Working/DEL-00-01_…/_STATUS.md:3`, `:11-13`). All eight PKG-00 `_STATUS.md` files read IN_PROGRESS at l.3 (T9-C07). Rulings: D-40 / DEC-072 (register row D-40; `F:execution/_Decomposition/SOFTWARE_DECOMP.md:663`).
- **The injection statement.** Each downstream `_CONTEXT.md` says PKG-00 "at `SEMANTIC_READY` supplies dispatchable architecture-basis constraints" (for example `F:execution/PKG-04_…/1_Working/DEL-04-01_…/_CONTEXT.md:51`). It restates SCA-001 (`SOFTWARE_DECOMP.md:432`).
- **Consolidation.** D-43 / SCA-006 consolidated the PKG-00 kits into per-member `ArchitectureBasis.md`, recorded as a "scope-neutral currency amendment" (`SOFTWARE_DECOMP.md:35`). The PKG-00 readiness gate was superseded (D-43, HUMAN-STEER-PKG00-EXCLUSION-001; T4B-C03).
- **Owner confirmation.** Direction 8 (`…/HELP-HUMAN-PIPING-20260921-RECONCILIATION/OWNER_DIRECTIONS.md:139-157`) confirmed the disposition STALE_REVIEW_OR_EVIDENCE and routed the status advance to R4. CONVENTIONS F3 records the same. This run changes no status (D-73).
- **DAG.** The 580 architecture-basis edges into PKG-00 are satisfied at RequiredMaturity SEMANTIC_READY on the recorded lifecycle reading (T11 D-07); a state change may bear on that reading.

## 3. Options

As they stand in the evidence (T4B-C03; T5B-C04; T9-C07):

1. **Advance, then restate.** Approve a PKG-00 lifecycle change in the lifecycle workflow (T9 names CHECKING, the state D-40 reversed, as one target), then restate each injection against the new state.
   - Deliverables: eight PKG-00 `_STATUS.md` state changes (owner act), then an R5 repair of the 82 statements.
   - Code: none.
   - Other packets: DAG satisfaction readings (T11 D-07) may need re-checking; C3's validation holds are unaffected.
2. **Restate without a readiness state.** Keep PKG-00 at IN_PROGRESS and restate the statements to cite `ArchitectureBasis.md` and SOFTWARE_DECOMP rev 0.12 as the basis (the ledgers' RemainingWork: "Restate the PKG-00 basis as the consolidated ArchitectureBasis.md members (IN_PROGRESS)"). This reflects D-43's superseded gate and makes the text independent of PKG-00's lifecycle.
   - Deliverables: R5 repair of the 82 statements only; no status change.
   - Direction 8 said the status "should be advanced"; this option defers that advance rather than rejecting it.
3. **Both.** Restate now without a readiness state (option 2), and advance PKG-00 separately later (option 1's workflow) when the owner approves.
4. **Mark as setup-era history** (T5B-C04 option (c)). Annotate the block as setup-era, as the `_CONTEXT.md` preamble already does for the gate rule. R5 edit only.

## 4. Evidence and reliability

| Source | Shows | Reliability |
|---|---|---|
| Direction 8 | Disposition confirmed; advance needs later owner approval | Owner-direction record, hash-bound (A3a) |
| `DEL-00-01/_STATUS.md:3`, `:11-13`; D-40/DEC-072 (`SOFTWARE_DECOMP.md:663`) | Current state IN_PROGRESS and its history | Lifecycle record and governing source |
| `SOFTWARE_DECOMP.md:35`, `:432` | SCA-006 consolidation; SCA-001 injection authority | Governing source |
| T4B-C03, T5B-C04 (`RUN/R3/TASKS/`) | 42 + 40 rows; options | R3 task proposals; 81 rows OWNER_CONFIRMED |
| T9-C07, T9-C12 SRE-5 (`RUN/R3/TASKS/T9_LIFECYCLE.*`) | 8 PKG-00 items + 83 injection keys; SRE-5 = 92 rows in 82 deliverables | R3 task proposal |
| T8-K1 (`RUN/R3/TASKS/T8_CLUSTERS.md`) | Cause split 42/39; route R5 after the owner acts | R3 task proposal (cause is C7's) |
| T11 S-01, S-02 | 81 sub-claims agree on disposition; DEL-07-03, 07-04, 07-06 carry the statement with no sub-claim | R3 screen |

Verified: the lifecycle lines and ruling rows above. Not decided here: the cause label.

## 5. Affected claims

**This packet's portion: 82 rows**, two whole classes (both Authority OWNER).

| Class | Rows | Filter |
|---|---|---|
| T4B-C03 PKG-00 SEMANTIC_READY injection sub-claims | 42 | `ClassID == 'T4B-C03'` |
| T5B-C04 SR-1 "PKG-00 at SEMANTIC_READY supplies the architecture basis" | 40 | `ClassID == 'T5B-C04'` |

T4B-C03: 42 `CONTEXT#architecture-basis-injection.s01` keys (cause RECORD_DRIFT), 12 packages (PKG-02, 03, 04, 07 to 15). T5B-C04: 39 injection sub-claims (`.s01`; `.s02` for DEL-16-0x; cause SCOPE_REDIRECTED_BY_RULING) plus `DEL-11-05:SOW#CLM-031` (same premise in a SOW row; no resolution row, so Direction 8 does not formally cover it; T5B observation 4). Together they are the 81 OWNER_CONFIRMED sub-claims plus that SOW row. AuthorityNeeded is OWNER on DEL-13-01, 13-03, 13-04 and NO on the other 39 T4B-C03 rows (T4B observation 1).

**Subject-keyed items (T9).**
- T9-C07: `LIFECYCLE_STATE:PKG-00_SEMANTIC_READY` for DEL-00-01 to DEL-00-08 (8 items) and `LIFECYCLE_STATE:PKG-00_SEMANTIC_READY_INJECTIONS` (1 corpus item).
- T9-C12: `STALE_VV:SRE-5_review_and_lifecycle_states` (1 item; 92 rows, 82 deliverables).

**Rows on other routes (both views; not claimed).**
- T8 SR-1 route disagreement (`RUN/R3/T8_ROUTE_DISAGREEMENTS.csv`): the 81 sub-claims are OWNER_DECISION by class and R5_RECORD_REPAIR by T8 (T8: "R5, after the owner acts on the status advance"). Both views agree the text repair waits on this packet.
- Two further injection parents T9-C07 counts (cause BASIS_POINTER_STALE): `DEL-02-04:CONTEXT#architecture-basis-injection` and `DEL-02-05:CONTEXT#architecture-basis-injection` (T4A-C01, H4). Their pin repair is mechanical; the SEMANTIC_READY sentence in them follows this packet. H4 carries both with `BlockedOnPacket = A5;C7`, so a T4A-C01 pin repair does not restate the sentence before the lifecycle choice.
- T8-K1 adjacent rows (a deliverable's own readiness target): `DEL-06-02:SOW#CLM-024.r06` (T4B-C08), `DEL-09-05:SOW#CLM-003` (T5B-C08), `DEL-10-05:SOW#CLM-002.r05` (T4A-C05), `DEL-10-04:SOW#CLM-003.r08` (T4B-C08) on R5 both ways; `DEL-10-01:SOW#CLM-004`, `DEL-10-03:SOW#CLM-004` (T5A-C08, REVIEW by class, R5 by T8).
- T9-C08 own-state declarations, 8 rows on R5 (H4): `DEL-06-02:SOW#CLM-024.r05`, `.r06`, `DEL-08-01:SOW#CLM-003`, `DEL-10-04:SOW#CLM-003.r08`, `DEL-12-05:SOW#CLM-003.r11`, `#CLM-020.r09`, `#CLM-028.r06`, `DEL-17-02:SOW#CLM-003.r04`. They are in SRE-5; they restate each deliverable's own state against its `_STATUS.md` and imply no lifecycle change. The ninth, `DEL-01-01:SOW#CLM-002.r11`, is A6's.
- Unassessed statements: DEL-07-03, 07-04, 07-06 carry the sentence with no sub-claim (T11 S-02). The R5 tranche should include them.

**Rows known only from `OtherCorrections`:** the CONTESTED `DEL-09-05:SOW#CLM-003` verifier split (lifecycle target "SEMANTIC_READY for setup review" as a `.sNN`, not minted) and the OBSERVED `DEL-10-01:SOW#CLM-004`, `DEL-10-03:SOW#CLM-004`, `DEL-10-04:SOW#CLM-003.r08` cluster notes.

**Packages and deliverables.** PKG-00 (8, state only) and 81 downstream deliverables across PKG-01 to PKG-16 (82 rows; DEL-11-05 carries two: its injection sub-claim and `DEL-11-05:SOW#CLM-031`). SRE-5's figure of 82 deliverables is a different population (it adds the T9-C08 rows).

## 6. Risks

- **Undecided.** 81 `_CONTEXT.md` statements (plus one SOW row) tell agents a readiness gate exists that was superseded; sealed briefs may treat PKG-00 constraints as more settled than IN_PROGRESS warrants. Dispatch logic that reads the statement may wait on, or assert, a state that does not exist.
- **Option 1.** A state change on eight architecture deliverables that D-40 deliberately reversed; the target and its evidence need the lifecycle workflow's own review.
- **Option 2.** Leaves Direction 8's "should be advanced" open with no date.
- **Option 3.** Two acts instead of one; the text must not be restated twice.
- **Option 4.** Keeps a false present-tense sentence under a history marker.

## 7. Recommended routing

The evidence supports one sequencing point: the text repair should follow the owner's lifecycle choice, so it is done once (T4B-C03, T9-C07, T8-K1 all say so). On the target state itself: no recommendation; owner's call.

## 8. On-ruling mechanism

- **Option 1 or 3 (advance).** The owner approves the PKG-00 lifecycle change through the lifecycle workflow (docs/TYPES.md §9 regime, as D-40 used). This run never makes that change (D-73; F3). An R5 record-repair tranche then restates the 82 statements (plus the two DEL-02-04/05 parents and the three unassessed contexts) to the approved state.
- **Option 2 or 4 (restate only).** An R5 record-repair tranche of the 82 `_CONTEXT.md`/SOW blocks under one instrument, citing `ArchitectureBasis.md` and SOFTWARE_DECOMP rev 0.12 (option 2) or marking the block as setup-era history (option 4).

Nothing executes until the owner acts. R5 needs separate authorisation.

## 9. Dependencies

- **Depends on:** C7 only for the cause label to write in the repaired rows (not for the option).
- **Interacts with:** A6 (the DEL-01-01 injection sub-claim `DEL-01-01:CONTEXT#architecture-basis-injection.s01` is in T5B-C04 here, but any edit to DEL-01-01 goes through A6's ISSUED path); A9 (the same `_CONTEXT.md` and SOW files carry the D-41 blocks); T4A-C04's file-level SEMANTIC_READY framing on the DEL-06-0x CONTEXT rows (T4A-C04 mechanism).
- **Blocks:** H4 repair of the 82 portion rows, the T8-K1 adjacent rows' readiness element, and the SEMANTIC_READY sentence in the DEL-02-04/05 parents.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). No certification, code-compliance, professional-approval or engineering-acceptance claim is made.
