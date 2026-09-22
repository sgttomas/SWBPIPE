# T9 — Lifecycle and Remaining (R3, run HELP-HUMAN-PIPING-20260921-RECONCILIATION)

T9 reads the effective R2 values in `R3/CORPUS_CLAIMS.csv` together with the sealed ledgers, the adopted resolutions and the frozen `_STATUS.md` records. It looks at four things. (1) The 29 `REMAINING_STATE_MISMATCH` rows, in 20 deliverables across 9 packages. Most are Remaining items whose condition has passed or been ruled (CP-07); a few trail the code, cite a file that does not exist, or misstate open work. (2) The 16 `LIFECYCLE_REASSESSMENT_REQUIRED` rows. All of them sit in the ISSUED DEL-01-01 SOW. (3) Lifecycle state against evidence. The inventory records only three lifecycle values: 100 deliverables IN_PROGRESS, DEL-01-01 ISSUED and DEL-07-09 OPEN. None is SEMANTIC_READY. Yet 83 architecture-basis-injection keys in 81 deliverables still say "PKG-00 at SEMANTIC_READY", while all eight PKG-00 `_STATUS.md` files read IN_PROGRESS (D-40). Direction 8 has confirmed that this status is outdated and that advancing it is an owner decision. (4) Stale verification and validation. This covers the 14 `VERIFIED_NOT_VALIDATED` rows and the 1,522 `STALE_REVIEW_OR_EVIDENCE` rows, summarised by class. The Remaining census covers all 102 deliverables: 151 items in 65 deliverables, and 37 deliverables that record NONE. Owner decisions are needed on: the PKG-00 lifecycle advance; the DEL-01-01 ISSUED change path (three linked items); the DEL-07-09 lifecycle; two missing-file confirmations; and the owner-held validation bases. T9 changes no row, lifecycle state or record. Every route is a proposal for R4.

## Classes

Each CSV row gives its class ID first in `Finding`. The population figures reproduce from `CORPUS_CLAIMS.csv` with the filters stated.

### T9-C01 — Remaining condition passed or ruled (CP-07)

- **Description.** The Remaining text is otherwise accurate, but a gate, trigger or pre-merge condition it names has since been met or ruled, so it still reads as open. Examples: PR #789 merged on 2026-09-17; D-72 ruled the "prospective" criteria on 2026-09-18 (item 5 unruled); the N7 V2 rereview returned PASS; D-07b ruled (DEC-079); the catalogue and constants accepted (DEC-018) and the CI posture ruled (DEC-025); D-40/DEC-072 left no deliverable at CHECKING; D-12 ruled (DEC-078); D-06b ruled (DEC-089); Receipt 87 and its commit-bound sweep exist.
- **Signature.** REMAINING_STATE_MISMATCH · RECORD_DRIFT · LOCAL_DESIGN. BaselineClass is RULED_CRITERION on 12 rows and NONE on 7. RECORD. AuthorityNeeded NO.
- **Population.** 19 rows in 14 deliverables (PKG-01, 05, 07, 09, 10, 11):
  - DEL-07-01 R03 and R04; DEL-07-02 R02 and R03; DEL-07-03 R03 and R04; DEL-07-06 R02 and R03; DEL-07-09 R02 and R03;
  - DEL-01-03 R01; DEL-05-02 R01; DEL-09-01 R02; DEL-09-02 R01; DEL-09-05 R03; DEL-10-02 R03; DEL-10-03 R01; DEL-10-04 R02; DEL-11-02 R01.
- **Exceptions kept visible.** DEL-10-04 R02 carries a **FIELD** resolution: VerificationClass STATIC_CHECK against NOT_APPLICABLE evidence.
- **Owner.** NONE. The governing rulings and events already exist.
- **Route.** `R5_RECORD_REPAIR`.
- **On-ruling mechanism.** An R5 ruling would authorise rewriting each Remaining item in the deliverable's `_STATUS.md` to cite the ruling or event and drop the condition. The ledger RemainingWork gives the replacement text. Under A4, the census invariant "Remaining is executable truth" applies at R6.
- **Risk if unrepaired.** Readers treat met gates as open. The PKG-07 shared bodies go on disagreeing with D-70's revision table, which updated DEL-07-01, 07-02 and 07-06 but not DEL-07-09.
- **Representative keys.**
  - `DEL-07-02:STATUS#remaining/R03`: the pre-merge condition, overtaken by the PR #789 merge (8468a33c).
  - `DEL-07-09:STATUS#remaining/R02`: D-70 and D-72 not applied, a reproduction of the R0 finding.
  - `DEL-10-03:STATUS#remaining/R01`: the D-12 either/or, overtaken by DEC-078 (FR-025 is implemented post-beta).

### T9-C02 — Remaining text behind the code

- **Description.** The item describes a landed baseline that is older than the code at the freeze.
  - The D-68 foundation and the SWBPIPE canvas slices are missing from the stated landed baseline (DEL-07-01 R06, DEL-07-02 R08).
  - The ratio viewer is described as showing supplied rows. Since PR #787 the code deliberately always shows "unavailable" (DEL-07-05 R02).
  - Report-package state/run and comparison sections have been bound since d2d8975ef (DEL-08-06 R01).
  - export-results is called a runner stub. It has been bound since f82bb28e2 (DEL-09-04 R01).
  - LFSP-REQ-011 test families are called absent. Tests exist (DEL-12-01 R01).
- **Signature.** REMAINING_STATE_MISMATCH · DOC_BEHIND_CODE · LOCAL_DESIGN · NONE · RECORD.
- **Population.** 6 rows in 6 deliverables (PKG-07, 08, 09, 12).
- **Owner.** NONE for five rows. REVIEW for DEL-12-01 R01, where the owner or a reviewer must decide which store tests satisfy LFSP-REQ-011.
- **Route.** `R5_RECORD_REPAIR` for five rows; `REVIEW` for DEL-12-01 R01.
- **On-ruling mechanism.** R5 restates each item against the code evidence the ledger cites. For DEL-12-01, a review under the review workflow first names the qualifying tests; R5 then rewrites or closes the item.
- **Risk if unrepaired.** Open work is overstated. DEL-07-05 R02 in particular misdescribes result-integrity behaviour that was changed deliberately.
- **Representative keys.** `DEL-07-05:STATUS#remaining/R02` (ResultsPanel `ratioCount={0}`); `DEL-09-04:STATUS#remaining/R01`.

### T9-C03 — Remaining cites evidence absent from the frozen tree (CP-08)

- **Description.** Both items cite `HISTORICAL_N7_ACCEPTANCE_INTAKE_V1.json`. The file is not in `git ls-files` at the freeze. The only matches for the name are the two `_STATUS.md` files that cite it (rechecked by T9).
- **Signature.** REMAINING_STATE_MISMATCH · RECORD_DRIFT · LOCAL_DESIGN · NONE · RECORD. AuthorityNeeded OWNER.
- **Population.** 2 rows: DEL-07-02 R11 and DEL-07-09 R09 (PKG-07).
- **Owner.** OWNER.
- **Route.** `OWNER_DECISION`.
- **Decision.** Does the intake record exist outside the frozen tree? The options, as the evidence stands:
  - (a) The owner or Agent 0 names its location; R5 re-points the citation.
  - (b) It does not exist; R5 removes the citation, keeping the V2 review return and the parent fan-in, which do exist.
- **On-ruling mechanism.** An R4 confirmation, then an R5 record repair of both `_STATUS.md` files.
- **Risk if unrepaired.** An accepted historical review basis rests on a missing artifact.

### T9-C04 — Remaining misstates the open work

- **Description.** In each case the text is inaccurate about what remains open:
  - DEL-05-04 R01 declares "None." But the deliverable's own CLM-008 and CLM-023 hold the stale-hash human-acceptance negative (FG-DEL-05-04-01), and four-document residue is open.
  - DEL-13-02 R01 keeps an open human disposition, which is accurate. Its premise ("despite current unit-vocabulary evidence") no longer holds: the accepted vocabulary gained `force_per_length` on 2026-05-17, and the constraint enum did not follow.
- **Signature.** REMAINING_STATE_MISMATCH · RECORD_DRIFT · LOCAL_DESIGN · NONE · RECORD. AuthorityNeeded is NO for DEL-05-04 and REVIEW for DEL-13-02.
- **Population.** 2 rows: DEL-05-04 R01 and DEL-13-02 R01.
- **Owner.** NONE for DEL-05-04; REVIEW for DEL-13-02.
- **Route.** `R5_RECORD_REPAIR` for DEL-05-04 R01 (record the held negative, or have it ruled out of scope). `REVIEW` for DEL-13-02 R01 (the human disposition of finding PKG13-DEL-13-02-PKG02-001, read against the current enum).
- **On-ruling mechanism.**
  - DEL-05-04: an R5 rewrite of the item.
  - DEL-13-02: a review-workflow disposition, then an R5 restatement.
- **Risk if unrepaired.** DEL-05-04 appears complete while a negative is held. DEL-13-02's disposition would be made on a false premise.

### T9-C05 — ISSUED DEL-01-01 TBD group overtaken by rulings (FG-DEL-01-01-01)

- **Description.** DEL-01-01 was ISSUED on 2026-06-03. The human approval left the remaining TBDs to later discretion. Later rulings then settled most of them:
  - DEC-027 (2026-06-11): sole maintainer and release authority, quorum one.
  - DEC-057 and DEC-089: v0.1 unsigned, with checksum, sweep and release record, and a policy-only future Developer ID target.
  - DEC-081: validation wording and labels.
- **Still open.** The legal review process (DEC-079, CLM-018.s02) and the maturity sentence.
- **Signature.** LIFECYCLE_REASSESSMENT_REQUIRED · SCOPE_REDIRECTED_BY_RULING · PROJECT_BASELINE · ISSUED · RECORD;LIFECYCLE. AuthorityNeeded OWNER.
- **Population.** 15 rows, all in DEL-01-01: CLM-004.r04, r05 and r06; CLM-005.r02, r03, r04 and r05; CLM-006; CLM-013.s02; CLM-014; CLM-018.s02; CLM-023; CLM-024.s04; CLM-025.r02; CLM-027/C-01-01-002.
- **Exceptions kept visible (FIELD resolutions).**
  - CLM-004.r05: the DecisionBasis should be DEC-081 and DEC-105, not DEC-027, DEC-057 and DEC-089. The FindingGroup should be a separate group, or FG-01 with the extension disclosed to the owner.
  - CLM-014: add DEC-081 to the DecisionBasis.
  - T9 proposes that the owner be told CLM-004.r05 rests on a different ruling set.
- **Owner.** OWNER.
- **Route.** `OWNER_DECISION`.
- **Decision.** One ISSUED-change-path decision for the whole group, per C6(d) and ruling item 4. The options, as the evidence states them: (a) reissue; (b) amend under the ISSUED change path; (c) leave the TBD text as history.
- **On-ruling mechanism.** The ruling authorises the chosen path through the scope-change workflow (the profile routes ISSUED changes through scope change). A reissue or amendment is then carried out under R5 or R6 authority. D-73 permits no write in this run.
- **Risk if unrepaired.** The ISSUED governance baseline keeps reading TBD against recorded owner rulings. MAINTAINERS L42 carries the same lag.

### T9-C06 — ISSUED DEL-01-01 SOW surface: rename residue, contested

- **Description.** CLM-009 L133 still names OpenPipeStress. The rename (DEC-101/SCA-010, 2026-09-18) came after issuance, and ruling item 3 says DEC-101 does not reach SOWs.
- **Signature.** LIFECYCLE_REASSESSMENT_REQUIRED · RENAME_OR_IDENTITY · LOCAL_DESIGN · ISSUED · RECORD, with no LIFECYCLE layer. AuthorityNeeded OWNER.
- **Population.** 1 row, `DEL-01-01:SOW`.
- **Exceptions kept visible (CONTESTED resolution).** The conventions do not rank C6(d) against CP-02 and CP-04 for ISSUED text. The two readings are:
  - (a) CP-04 as written, with BaselineClass ISSUED;
  - (b) C6(d) extended to the ISSUED revision and DAG pins as well. Those pins are the five `DEL-01-01:SOW` rows (CLM-002.r09, CLM-002.r10, CLM-007, CLM-013.s01, CLM-017), which are now STALE_REVIEW_OR_EVIDENCE · CP-02 · BaselineClass ISSUED.
- **Double routing.** Today the row goes to R4 twice: under FG-DEL-01-01-02 and in the rename class.
- **Owner.** OWNER.
- **Route.** `OWNER_DECISION`.
- **Decision.** How rename residue and pins in ISSUED text are routed. This is also a W2 owner item. The options are (a) or (b) above. With either, T9 proposes one routing path, taken with the R4 rename-class ruling (T4B).
- **On-ruling mechanism.** The ruling fixes the classification, and the ISSUED change path then carries any edit (scope-change workflow).
- **Risk if unrepaired.** The same text is decided twice, possibly inconsistently.

### T9-C07 — PKG-00 "SEMANTIC_READY" against the PKG-00 lifecycle (Direction 8)

- **Description.** The PKG-00 deliverables' lifecycle history reads:
  - SEMANTIC_READY on 2026-04-30;
  - CHECKING on 2026-06-04;
  - IN_PROGRESS on 2026-07-11, by the D-40 rebaseline (register l.74). DEL-00-01 `_STATUS.md` l.11–13 shows the sequence.
- **The mismatch.** All eight PKG-00 `_STATUS.md` files read IN_PROGRESS at l.3. Yet 83 `CONTEXT#architecture-basis-injection(.s01)` keys in 81 deliverables still state "PKG-00 at SEMANTIC_READY". All 83 are effectively STALE_REVIEW_OR_EVIDENCE, 81 of them OWNER_CONFIRMED.
- **Direction 8.** The owner confirmed the reading, called the status outdated, and said it "should be advanced upon my subsequent approval in the proper workflow".
- **Signature.** STALE_REVIEW_OR_EVIDENCE · LOCAL_DESIGN · NONE · RECORD. The cause is split into RECORD_DRIFT 42, SCOPE_REDIRECTED_BY_RULING 39 and BASIS_POINTER_STALE 2. That split is the SR-1 cluster, owned by T8.
- **Population.** The 8 PKG-00 deliverables plus 83 injection keys. The CSV has 8 per-deliverable rows and one corpus row.
- **Owner.** OWNER.
- **Route.** `OWNER_DECISION`.
- **Decision.** The target lifecycle state for PKG-00 and the workflow that advances it. The evidence leaves the target open. The options are:
  - (a) Advance the PKG-00 deliverables through the lifecycle workflow (for example toward CHECKING, the state D-40 reversed), then restate the 83 injections to match.
  - (b) Keep PKG-00 at IN_PROGRESS and restate the 83 injections to that state.
  - Either way, the injection text follows the lifecycle decision.
- **On-ruling mechanism.** The owner approves the lifecycle change through the proper lifecycle workflow (docs/TYPES.md lifecycle, as D-40 used §9). An R5 record repair of the 81 `_CONTEXT.md` injection blocks follows. This run changes no status.
- **Risk if unrepaired.** Every downstream context cites a PKG-00 readiness state that PKG-00's own record contradicts. Architecture-gate reasoning (CS-05, 8 rows) keeps a false premise.

### T9-C08 — Own lifecycle declarations that disagree with `_STATUS.md`

- **Description.** A SOW states its own current state as SEMANTIC_READY while `_STATUS.md` reads IN_PROGRESS. DEL-01-01 is the reverse case: its SOW says "IN_PROGRESS governance baseline refresh" while `_STATUS.md` reads ISSUED.
- **Scope.** Setup-procedure text that merely names SEMANTIC_READY as a target is not included. It is setup residue in T4A–T5B.
- **Signature.** STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT or DOC_BEHIND_CODE · LOCAL_DESIGN · NONE, or ISSUED on DEL-01-01 · RECORD. The DEL-06-02 rows also carry the LIFECYCLE layer.
- **Population.** 9 rows: DEL-06-02 CLM-024.r05 and r06; DEL-08-01 CLM-003; DEL-10-04 CLM-003.r08; DEL-12-05 CLM-003.r11, CLM-020.r09 and CLM-028.r06; DEL-17-02 CLM-003.r04; DEL-01-01 CLM-002.r11.
- **Exceptions kept visible.**
  - DEL-06-02 r05 and r06 are **WEAK**. The r05 correction: `validate_enum.py` exists at the repository root.
  - DEL-10-04 CLM-003.r08 is **OBSERVED** (the cause question is in the SR-1 cluster).
- **Owner.** NONE for 8 rows; OWNER for DEL-01-01 (ISSUED).
- **Route.** `R5_RECORD_REPAIR` for 8 rows. `OWNER_DECISION` for DEL-01-01 CLM-002.r11, which goes with C05, C06 and C10 on the ISSUED change path.
- **On-ruling mechanism.** R5 restates the status field. `_STATUS.md` stays the lifecycle record, and no lifecycle change is implied.
- **Risk if unrepaired.** Contradictory state declarations inside a single deliverable.

### T9-C09 — DEL-07-09 OPEN against its evidence

- **Description.** PREPARATION initialised DEL-07-09 as OPEN on 2026-08-21 (`_STATUS.md` l.3, l.23). Since then it has accumulated a contract and a coverage ledger: 87 of its 111 rows are ALIGNED.
- **Ledger reading.** The ledger finds OPEN accurate (`STATUS#remaining/R06`: "State OPEN matches the census"). `_STATUS.md` l.13 says lifecycle review or promotion is separately human-gated and that implementation evidence does not move OPEN.
- **Signature.** None of its rows is lifecycle-divergent. This is a lifecycle-currency observation, not a record defect.
- **Population.** 1 deliverable.
- **Owner.** OWNER.
- **Route.** `OWNER_DECISION`, as a candidate only.
- **Decision.** Whether to open lifecycle review or promotion of DEL-07-09. The options are (a) advance through the lifecycle workflow, or (b) keep OPEN as intended, if the deliverable is meant to stay a control surface.
- **On-ruling mechanism.** The lifecycle workflow, on owner approval. No record repair is needed under option (b).
- **Risk if unrepaired.** Low. OPEN understates the maturity of the contract artifacts.

### T9-C10 — DEL-01-01 ISSUED state against post-issuance edits

- **Description.** ISSUED (`_STATUS.md` l.3, l.14) matches the human-approved closeout. However, the ISSUED SOW bytes changed after issuance:
  - The 2026-07-14 representation conversion. It had a parity PASS at the time of conversion (RECON-I0-PKG01).
  - The DEC-081 Wave 2 edit (commit 8fac6631a, history l.15), which changed six lines. It is recorded as AC-001 ACCEPTED_DIVERGENCE, and the ledger asks the owner to confirm that DEC-081 reached an ISSUED SOW.
- **Other records on the same SOW.** OUT-001 parity binds the pre-DEC-081 production (CP-09). CLM-009.s01 is AUTHORITY_CONFLICT (free and open-source against source-available noncommercial). CLM-003 is CONTESTED and tied to that conflict. STATUS Last Updated (2026-06-03) is older than the history (CP-05).
- **Owner.** OWNER.
- **Route.** `OWNER_DECISION`. This is a W2 gate owner item.
- **Decision.** Did DEC-081 validly reach the ISSUED SOW without an SCA? DEC-081 says it is not an SCA, while the profile routes ISSUED changes through scope change. The options are:
  - (a) Confirm, and keep ISSUED with the edit recorded.
  - (b) Route the edit through the ISSUED change path.
  - T9 proposes one R4 packet, "DEL-01-01 ISSUED change path", covering C05, C06, C08 (CLM-002.r11) and C10. The posture conflict stays as its own ruling.
- **On-ruling mechanism.** The owner's confirmation, or a scope-change-workflow amendment and reissue.
- **Risk if unrepaired.** An ISSUED baseline whose current bytes have no issuance or parity record.

### T9-C11 — VERIFIED_NOT_VALIDATED (14 rows, summarised by class)

- **C11a. Owner-held independent validation.**
  - Population: 9 rows (DEL-07-03 ×3, DEL-07-06 ×6).
  - Signature: VALIDATION_GAP · INVARIANT · OWNER_HOLD.
  - What is missing: PDU-045, 046 and 049 are verified by project-owned tests and review only. The independent usability and security validation basis is held under D-68.
  - Route: `OWNER_DECISION` (authorise and scope the independent basis).
- **C11b. Engineering validation basis.**
  - Population: 4 rows (DEL-13-04 CLM-017; DEL-14-04 CLM-008.r02 and CLM-017.s02; DEL-14-05 Remaining R01).
  - Signature: VALIDATION_GAP · INVARIANT · NONE. AuthorityNeeded ENGINEERING.
  - What is missing: no named benchmark, witness or vetted source for comparison, tolerance or transform-target suitability (A5).
  - Route: `ENGINEERING_AUTHORITY`.
- **C11c. External prover activation.**
  - Population: 1 row (DEL-17-05 CLM-019, ACC-006).
  - What is missing: live validation waits on the owner-gated external-prover activation (DEC-080).
  - Route: `OWNER_DECISION`.
- **Risk if unrepaired.** Verification may be read as validation, which A5 forbids. No row claims validation today.

### T9-C12 — STALE_REVIEW_OR_EVIDENCE (1,522 rows in 102 deliverables, summarised by class)

The seven subclasses below partition all 1,522 rows. The script asserts that the counts sum to the total.

| Subclass | Rows | Route | Note |
|---|---|---|---|
| SRE-1: parity evidence not bound to the frozen SOW (CP-09) | 169 | `REVIEW` | PASS records exist, but `AnyPassMatchesFrozen=NO`. These are the core stale-verification rows. |
| SRE-2: protected-content review overtaken | 7 | `REVIEW` | EVIDENCE_OVERTAKEN · INVARIANT · IP_DATA;RECORD, in PKG-04 and PKG-05. AuthorityNeeded is REVIEW on 6 rows. DEL-05-01 VER-001 carries a **FIRM** resolution. Tied to the W2 item that no current protected-content review of the solver crates exists. |
| SRE-3: other evidence overtaken | 25 | `R5_RECORD_REPAIR` | Superseded run records, counts or returns. Nothing was rerun (A6). |
| SRE-4: basis pins | 737 | `R5_RECORD_REPAIR` | CP-02, CS-01, CS-04 and CS-06, owned by T4A. The 5 ISSUED DEL-01-01 pins fall under the C06 contest. |
| SRE-5: review and lifecycle states | 92 | `OWNER_DECISION` | The 83 C07 injections plus the 9 C08 rows. |
| SRE-6: `_STATUS` Last Updated older than its history (CP-05) | 56 | `R5_RECORD_REPAIR` | |
| SRE-7: other overtaken declarations | 436 | `R5_RECORD_REPAIR` | By cause: DOC_BEHIND_CODE 127, SCOPE_REDIRECTED_BY_RULING 111, RENAME_OR_IDENTITY 85, RECORD_DRIFT 54, REPRESENTATION_MIGRATED 52, CONTRACT_VERSION_ADVANCED 7. T4A–T5B class them. The rename rows go to the R4 rename-class ruling. |

- **Exceptions.** Across all 1,522 rows, 120 carry a non-LOCAL tier, a non-NONE baseline or AuthorityNeeded OWNER. They include the PROJECT_BASELINE `.opsproj` and hash-profile identifier rows, 3 of them FROZEN_CONTRACT.
- **Resolution classes on these rows.** OWNER_CONFIRMED 81, OBSERVED 25, FIELD 24, CONTESTED 20, WEAK 12, RESOLVED_PAIR 10 and FIRM 2. They stay attached to their rows in the T4A–T7 partition.

## Remaining census (every deliverable)

The CSV has one `REMAINING_CENSUS` row for each of the 102 deliverables.

**Items.** There are 151 items: ALIGNED 65, DOCUMENTED_UNIMPLEMENTED 31, REMAINING_STATE_MISMATCH 29, PARTIALLY_IMPLEMENTED 19, UNKNOWN 6 and VERIFIED_NOT_VALIDATED 1. Every deliverable's inventory count equals its ledger Remaining rows. The 37 deliverables that record NONE get NO_ACTION: under A4, an absent Remaining section is not evidence that work is done.

**Census routes.** Each census row takes the most demanding route among its items, in this order: OWNER, then ENGINEERING, then REVIEW, then R5, then NO_ACTION.
- NO_ACTION: 80.
- R5_RECORD_REPAIR: 13.
- REVIEW: 6. The UNKNOWN items need the smallest next check (C6(h)): DEL-07-01 R05, DEL-07-02 R04, DEL-07-06 R04, DEL-07-09 R04, DEL-01-03 R02 and DEL-02-02 R01.
- OWNER_DECISION: 2 (DEL-07-02 and DEL-07-09, from C03).
- ENGINEERING_AUTHORITY: 1 (DEL-14-05).

**Exceptions kept visible.**
- DEL-07-06 R04 is **RESOLVED_PAIR**: sealed ALIGNED, effective UNKNOWN.
- DEL-16-02 R01 is **OBSERVED**: the worker asked whether CP-10 applies, and the verifier did not decide.
- DEL-10-04 R02 is **FIELD**.

## Coverage

A script (`_scratch_T9/gen.py`, since deleted) builds the CSV from `CORPUS_CLAIMS.csv`, the sealed forward ledgers (excluding `superseded_*`) and `REMAINING_CENSUS.csv`.

**Row counts.**
- `REMAINING_STATE_MISMATCH`: 29 of 29 rows, one CSV row each (C01 19, C02 6, C03 2, C04 2). The script stops on any unclassified row.
- `LIFECYCLE_REASSESSMENT_REQUIRED`: 16 of 16 rows (C05 15, C06 1).
- Census: 102 of 102 deliverables. For each one, the script checks the disposition string, the row count and the inventory count against `REMAINING_CENSUS.csv`. All match.
- Lifecycle findings: 8 PKG-00 rows and 1 corpus row (C07), 9 C08 rows, 1 C09 row and 1 C10 row.
- Stale V/V: 3 class rows for the 14 VNV rows (9 + 4 + 1), and 7 class rows for the 1,522 SRE rows. The script asserts that the SRE subclasses sum to 1,522.

**Totals.** 177 body rows. `#END` records 177. Routes: NO_ACTION 80, R5_RECORD_REPAIR 50, OWNER_DECISION 35, REVIEW 10, ENGINEERING_AUTHORITY 2.

**Inputs ignored, as directed.** All `RESOLUTIONS_DRAFT*.csv` files and `CROSS_PACKAGE/RESOLUTIONS_MERGED_DRAFT.csv`. No other task's files were read.

## R3 observations

1. **W1 ALIGNED Remaining rows without an F2 marker (R3_OBSERVATION; not a correction).**
   - What: 17 ALIGNED Remaining rows, all sealed in W1 (PKG-07 and PKG-16), carry neither `OPEN_ACTION:` nor `NO_OPEN_ACTION`. F2 applies from W2 onward.
   - Why it matters: several of them record open actions and name no governing row. Examples: `DEL-07-02:STATUS#remaining/R05`, R06 and R07; `DEL-07-06:STATUS#remaining/R05` and R06; `DEL-16-01:STATUS#remaining/R01`; `DEL-16-02:STATUS#remaining/R02`. Under F2 these might take a gap disposition.
   - Proposal: at R6, the census should read them as open work, not as closed work.
2. **Marker detection.** A plain substring match on `OPEN_ACTION` also matches `NO_OPEN_ACTION`. The T9 count uses a negative lookbehind. Anyone re-counting the census markers should do the same. The correct counts are OPEN_ACTION 37, NO_OPEN_ACTION 11 and unmarked 17.
3. **DEL-01-01:SOW has no LIFECYCLE layer.** It is the one LIFECYCLE_REASSESSMENT_REQUIRED row without that layer (layers RECORD only). The other 15 carry RECORD;LIFECYCLE. The difference follows from its contested CP-04/C6(d) reading and is left to the C06 decision.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). Nothing here asserts certification, code compliance, professional approval or engineering acceptance.
