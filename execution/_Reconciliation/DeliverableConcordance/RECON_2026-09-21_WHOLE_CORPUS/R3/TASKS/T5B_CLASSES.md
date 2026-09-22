# T5B classes — evidence overtaken, scope redirected, scope grown, contract advanced

T5B classifies the 583 divergent rows of `R3/CORPUS_CLAIMS.csv` whose effective CauseTag is EVIDENCE_OVERTAKEN (202), SCOPE_REDIRECTED_BY_RULING (359), SCOPE_GREW_BY_DIRECTION (14) or CONTRACT_VERSION_ADVANCED (8). The partition is dominated by record currency, not product gaps: 558 of 583 rows carry DivergenceLayers `RECORD` alone, 564 have BaselineClass `NONE`, and 552 have AuthorityNeeded `NO`. Ten classes result. Three need a review (C01, C02) or a record repair (C03) because review and verification records no longer bind the frozen bytes. Four are record repairs of text overtaken by later rulings or directions (C05, C06, C08, C10). Three need the owner: the SR-1 SEMANTIC_READY statements (C04), the PROJECT_BASELINE rows overtaken by rulings, including the ISSUED DEL-01-01 governance group (C07), and two scope-grown items that rest on owner records or need an owner line-drawing (C09). The count sits above the brief's "about 3–8" because exceptions are kept visible as their own classes (Rule 3): C03, C08, C09 and C10 are small classes with distinct routes. Effective values are used throughout. Nothing here re-disposes a row, and no claim of certification, code compliance, professional approval or engineering acceptance is made (F-PIP-2; claims taxonomy per DEC-081).

**Sequencing note, applies to C01 with C03, C05–C08 and C10.** Every SOW text repair changes the ScopeOfWork.md bytes. A fresh parity record made before those repairs would go stale again under CP-09. Any C01 re-review should therefore follow the SOW repairs of the same deliverable, not precede them.

## Class table

| ClassID | Name | Rows | Pkgs | Dels | Owning authority | Route |
|---|---|---|---|---|---|---|
| T5B-C01 | SOW parity and verification-method records no longer bind the frozen SOW (CP-09) | 175 | 17 | 84 | REVIEW | REVIEW |
| T5B-C02 | Protected-content and boundary review records predate later changes | 12 | 3 | 8 | REVIEW | REVIEW |
| T5B-C03 | Review-pending declarations and test-count snapshots overtaken | 15 | 4 | 4 | NONE | R5_RECORD_REPAIR |
| T5B-C04 | SR-1: "PKG-00 at SEMANTIC_READY supplies the architecture basis" | 40 | 11 | 40 | OWNER | OWNER_DECISION |
| T5B-C05 | Injected `_CONTEXT.md` blocks overtaken by rulings (Still-TBD lists, architecture gate rule) | 71 | 16 | 66 | NONE | R5_RECORD_REPAIR |
| T5B-C06 | Deliverable TBDs, holds and open questions settled by later rulings | 226 | 17 | 45 | NONE | R5_RECORD_REPAIR |
| T5B-C07 | PROJECT_BASELINE claims overtaken by rulings (ISSUED DEL-01-01 group and four holds) | 22 | 4 | 5 | OWNER | OWNER_DECISION |
| T5B-C08 | Write scope and boundary text outgrown under recorded direction | 9 | 3 | 4 | NONE | R5_RECORD_REPAIR |
| T5B-C09 | Scope grown on owner records or awaiting an owner boundary | 5 | 2 | 2 | OWNER | OWNER_DECISION |
| T5B-C10 | Contract version advanced: analysis-record 0.2 and the schema dispatcher | 8 | 2 | 2 | NONE | R5_RECORD_REPAIR |
| | **Total** | **583** | | | | |

Full populations are in `T5B_CLASSES.csv` (one row per key). Deliverable lists below are complete where a class touches 45 deliverables or fewer.

## T5B-C01 — SOW parity and verification-method records no longer bind the frozen SOW (CP-09)

**Description.** Output-matrix `OUT-001` rows, SOW `VER-001` verification-method rows and a few related acceptance rows claim the contract was checked by a SOW parity review and claim map. `EVIDENCE_MAP.csv` lists PASS parity records for each deliverable, but no PASS record's production hash matches the frozen ScopeOfWork.md (`AnyPassMatchesFrozen=NO`). The SOW moved after the parity run, usually through the 2026-07-14 batch adoption, DEC-081 / D-48 Wave 2 edits (2026-07-17) and later declarations. The contract substance mostly exists in code. What is stale is the review binding. 169 rows inherit CP-09 directly. Six `VER-001` rows (DEL-03-03, DEL-05-02, DEL-05-03, DEL-15-02, DEL-17-05, DEL-17-06) were judged as verification-method claims with the CP-09 outcome.

**Signature.** EVIDENCE_OVERTAKEN · LOCAL_DESIGN · STALE_REVIEW_OR_EVIDENCE (174), plus one PARTIALLY_IMPLEMENTED (DEL-15-02 VER-001, which also lacks any review of the no-silent-default and provenance behaviour). Layers `RECORD`; BaselineClass `NONE`; AuthorityNeeded `NO` on all 175. Surfaces: 93 `OUT-001`, 76 `VER-001`, 6 others.

**Population.** 175 rows; 84 deliverables in 17 packages (every package except PKG-00).

**Owning authority.** REVIEW. **Route.** REVIEW.

**On-ruling mechanism.** An R4 ruling would authorise, in R5, a SOW parity and claim-map re-review of each affected ScopeOfWork.md against its current bytes. The method is the one that produced the existing records, for example the SOW-STAGE2 parity and claim map under `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/`. Each new PASS record would carry a production hash equal to the reviewed SOW. The `OUT-001` and `VER-001` rows then cite it. The re-review runs after that deliverable's SOW text repairs (see the sequencing note). DEL-15-02 VER-001 also needs the verification of defaulted record inputs and provenance that its RemainingWork names, which touches the W3 owner item on DEL-15-02 silent defaults.

**Risk if unrepaired.** Every contract in these 84 deliverables has no review record of its current text. Later readers could take a stale PASS as binding the current SOW. The risk is to records, not behaviour: the code evidence on these rows is unaffected.

**Representative keys.**
- `DEL-02-02:SOW#output-and-evaluation-matrix/OUT-001`: its parity records (`…/SOW-PACKAGE-BATCH-ADOPTION-20260714-01/…/DEL-02-02/parity-2.md` and siblings) have production hash `3fd8885d…` against frozen SOW `d84a099a…` (`EVIDENCE_MAP.csv` lines 10–11, 24–25, 34); OUT-001 at freeze `PKG-02_…/DEL-02-02_…/ScopeOfWork.md:16`.
- `DEL-15-02:SOW#production-and-verification-method-praxeology/VER-001`: parity bound to `9fc7f3bb…` against frozen `4c2c8b23…`; the one PARTIALLY_IMPLEMENTED row (FG-DEL-15-02-01).
- `DEL-05-02:SOW#purpose-and-objective-traceability/OUT-001`: **CONTESTED**. The verifier asked whether a purpose-block OUT-001 is CP-09 at all; read literally, it would be judged on substance.

**Exceptions kept visible.** CONTESTED (2): `DEL-05-02:SOW#purpose-and-objective-traceability/OUT-001`, `DEL-05-03:SOW#purpose-and-objective-traceability/OUT-001`, with one package reading of the CP-09 scope. OBSERVED (4), DecisionBasis empty, no disposition change: `DEL-10-04` and `DEL-10-05` `…/VER-001` and `…/OUT-001`.

## T5B-C02 — Protected-content and boundary review records predate later changes

**Description.** These rows claim a protected-content, IP/data or professional-boundary review of a crate, fixture set, guide or SOW. The last dedicated review located is older than later changes to the reviewed bytes. Examples are the 2026-06-05 lifecycle-readiness QA row "Protected/private data boundary PASS", `_REVIEW.md` PB-001 of 2026-05-16, and the R18 agent review of the 2026-07-25 user guide. The later changes include kernel edits from 2026-06-11 onward, the 2026-09-05 P4 repair, the DEC-068 generator and the 2026-09-18 guide rename. Later tranches carry self-attestations only, and no DEC-058 scan binds the frozen bytes. On the five LOCAL_DESIGN rows (PKG-11 guides) the workers' own reads found no protected content, so the gap's tier follows F8. The seven INVARIANT rows (PKG-04, PKG-05) carry the IP_DATA layer. Workers' reads are not reviews, and nothing here asserts that the boundary holds.

**Signature.** EVIDENCE_OVERTAKEN · INVARIANT · STALE_REVIEW_OR_EVIDENCE (7, layers `IP_DATA;RECORD`, AuthorityNeeded REVIEW on 6) and EVIDENCE_OVERTAKEN · LOCAL_DESIGN · STALE_REVIEW_OR_EVIDENCE (5, `RECORD`).

**Population.** 12 rows. Packages PKG-04, PKG-05, PKG-11. Deliverables DEL-04-01, DEL-04-02, DEL-04-03, DEL-04-05, DEL-05-01, DEL-11-01, DEL-11-02, DEL-11-03.

**Owning authority.** REVIEW. The FIRM resolution on `DEL-05-01:…/VER-001` corrects AuthorityNeeded to REVIEW, which this routing adopts. **Route.** REVIEW.

**On-ruling mechanism.** An R4 ruling would authorise a protected-content review under the review workflow, run over each named crate, fixture set, guide and SOW at current bytes. It would use the DEC-058 scan owner and signatory arrangement where the release-scan path applies. The review writes a dated record binding the reviewed hashes, and the rows then cite it. This is evidence work only. It does not change the IP boundary.

**Risk if unrepaired.** This is the only class in T5B that touches a protected layer (IP_DATA). Protected data could have entered after the last review and no record would show it. The W3 owner list already carries a related PKG-14 item: six protected-content review records not located. That item is outside this partition.

**Representative keys.**
- `DEL-04-01:SOW#CLM-012/DEL-04-01-REQ-012` (INVARIANT, REVIEW): the requirement at freeze is `PKG-04_…/DEL-04-01_…/ScopeOfWork.md:157`; the last check is the 2026-06-05 QA report, followed by kernel changes.
- `DEL-05-01:SOW#production-and-verification-method-praxeology/VER-001` (INVARIANT, **FIRM**, sealed ALIGNED): the cited SOW-STAGE2 parity is a 34-check source-mapping parity, not a protected-content review. The FIRM resolution asks for a shared FindingGroup with `CLM-016/REQ-05-01-003`.
- `DEL-11-01:SOW#CLM-013.r03` (LOCAL_DESIGN): the R18 agent review of 2026-07-25 does not bind the guide renamed and trimmed on 2026-09-18. The data-boundary row is at freeze `PKG-11_…/DEL-11-01_…/ScopeOfWork.md:64`.

**Exceptions kept visible.** FIRM (1): `DEL-05-01:SOW#production-and-verification-method-praxeology/VER-001`.

## T5B-C03 — Review-pending declarations and test-count snapshots overtaken

**Description.** These are declared states that a completed review or a later test run has overtaken. In DEL-07-09 (9 rows), the palette routing and organization contracts still say that the root N7 rereview and the parent fan-in are pending. `REVIEW_RETURN_V2.md` (PASS) and `PRECOMMIT_PARENT_FAN_IN_V1.md` exist at the freeze, and `_STATUS.md` already records them (`PKG-07_…/DEL-07-09_…/_STATUS.md:15-16`). In DEL-04-06, DEL-05-01 and DEL-17-02 (6 rows), dated test-count or validator snapshots are presented as current. Examples: 19 tests where the crate now has 24; 40 tests where it has 49 (`PKG-05_…/DEL-05-01_…/ScopeOfWork.md:123`); a 2026-07-12 validator pass over four-document files that were replaced on 2026-07-14. The current state is shown by GATE records, which were not rerun.

**Signature.** EVIDENCE_OVERTAKEN · LOCAL_DESIGN · STALE_REVIEW_OR_EVIDENCE; `RECORD`; `NONE`; AuthorityNeeded `NO`.

**Population.** 15 rows. Packages PKG-04, PKG-05, PKG-07, PKG-17. Deliverables DEL-04-06, DEL-05-01, DEL-07-09, DEL-17-02.

**Owning authority.** NONE (a catch-up with no decision). **Route.** R5_RECORD_REPAIR.

**On-ruling mechanism.** An R5 record repair under a later owner authorisation of R5. It restates each pending clause as completed, citing the review return and fan-in. It re-dates or relabels test-count snapshots as history and points current state at the GATE record. Per A5, `DEL-05-01:SOW#CLM-008` must stop calling a unit-test run "validation evidence".

**Risk if unrepaired.** Low. Readers may think DEL-07-09 acceptance is still open, or may rely on stale test counts.

**Representative keys.**
- `DEL-07-09:PALETTE_OPERATION_ROUTING#n7-f1-f2-f3-repair-amendment-final-rereview-pend.s04`: the amendment still says the rereview and fan-in are pending; both records exist at the freeze.
- `DEL-04-06:SOW#CLM-015/REQ-04-06-001`: the June 5 record counts 19 tests; the crate has 24 at the freeze (`PKG-04_…/DEL-04-06_…/ScopeOfWork.md:116`).
- `DEL-17-02:SOW#CLM-033`: a 2026-07-12 "Current" validator pass over files replaced on 2026-07-14.

**Exceptions kept visible.** FIELD (3), each correcting CauseTag to RECORD_DRIFT (not applied; see R3 observation 1): `DEL-07-09:PALETTE_OPERATION_ROUTING#all-row-routing-and-bounded-residuals.r02`, `DEL-07-09:PALETTE_ORGANIZATION_CONTRACT#evidence-disposition`, `DEL-07-09:PALETTE_ORGANIZATION_CONTRACT#n7-f1-f2-f3-repair-amendment-final-rereview-pend.s04`.

## T5B-C04 — SR-1: "PKG-00 at SEMANTIC_READY supplies the architecture basis"

**Description.** Each deliverable's `_CONTEXT.md` architecture-basis injection says that PKG-00 at SEMANTIC_READY supplies dispatchable architecture-basis constraints. For example, `PKG-04_…/DEL-04-01_…/_CONTEXT.md:51`. At the freeze all eight PKG-00 deliverables are IN_PROGRESS (for example `PKG-00_…/DEL-00-01_…/_STATUS.md:3`, set by D-40 on 2026-07-11), and D-43 with HUMAN-STEER-PKG00-EXCLUSION-001 consolidated PKG-00 into `ArchitectureBasis.md`. The owner confirmed the disposition under Direction 8: a readiness state counts as a review state, hence STALE_REVIEW_OR_EVIDENCE. CONVENTIONS F3 says advancing that status is a lifecycle change for the owner's later approval, routed to R4. The CauseTag split between RECORD_DRIFT and SCOPE_REDIRECTED_BY_RULING is the contested corpus cluster SR-1, which T8 addresses. The sealed cause is kept here. The class also holds `DEL-11-05:SOW#CLM-031`, a contributor-guide SOW row whose premise is the same SEMANTIC_READY statement (SCA-001).

**Signature.** SCOPE_REDIRECTED_BY_RULING · LOCAL_DESIGN · STALE_REVIEW_OR_EVIDENCE (40); `RECORD`; `NONE`. 39 rows are OWNER_CONFIRMED; 11 of those were sealed STALE_SETUP_SPECIFICATION and moved by the resolution.

**Population.** 40 rows, one per deliverable. Packages PKG-01, 03, 04, 05, 06, 08, 09, 11, 12, 14, 16. Deliverables DEL-01-01, DEL-01-02, DEL-01-03, DEL-01-04, DEL-03-01, DEL-03-02, DEL-03-03, DEL-03-04, DEL-03-05, DEL-03-06, DEL-03-08, DEL-04-01, DEL-04-02, DEL-04-03, DEL-05-01, DEL-05-02, DEL-05-03, DEL-05-04, DEL-05-05, DEL-06-01, DEL-06-02, DEL-06-03, DEL-06-04, DEL-06-05, DEL-08-01, DEL-08-02, DEL-08-03, DEL-09-01, DEL-09-02, DEL-09-03, DEL-11-05, DEL-12-01, DEL-12-02, DEL-12-03, DEL-14-04, DEL-14-05, DEL-16-01, DEL-16-02, DEL-16-03, DEL-16-04.

**Owning authority.** OWNER. **Route.** OWNER_DECISION.

**Owner decision.** How the statement is made true. The options as they stand in the evidence:
- (a) Restate the injection as the D-43 consolidated `ArchitectureBasis.md` members at their current state (IN_PROGRESS). The ledgers' RemainingWork proposes this, for example "Restate the PKG-00 basis as the consolidated ArchitectureBasis.md members (IN_PROGRESS)".
- (b) Advance the PKG-00 lifecycle through its proper workflow, after which a readiness statement could be reissued (F3; OWNER_DIRECTIONS Direction 8).
- (c) Mark the block as setup-era history, as the `_CONTEXT.md` preamble already does for the gate rule (compare C05).

The SR-1 cause choice (T8) changes the cause label only. It does not change which option applies.

**On-ruling mechanism.** Option (a) or (c): an R5 record repair of 40 `_CONTEXT.md` or SOW blocks under one instrument. Option (b): a lifecycle change for PKG-00 in the lifecycle workflow, outside this run under D-73, followed by the text repair.

**Risk if unrepaired.** Every sealed context in 40 deliverables points to a readiness state that does not exist. Dispatched briefs could treat PKG-00 constraints as more settled than their IN_PROGRESS state warrants.

**Representative keys.**
- `DEL-04-01:CONTEXT#architecture-basis-injection.s01`: `_CONTEXT.md:51` against `DEL-00-01/_STATUS.md:3` (IN_PROGRESS) and D-43.
- `DEL-16-01:CONTEXT#architecture-basis-injection.s02`: OWNER_CONFIRMED; OtherCorrections name SR-1.
- `DEL-11-05:SOW#CLM-031`: the same premise in a SOW row (SCA-001; D-43); no resolution row.

**Exceptions kept visible.** 39 OWNER_CONFIRMED rows (all but `DEL-11-05:SOW#CLM-031`). All sit in cluster SR-1.

## T5B-C05 — Injected `_CONTEXT.md` blocks overtaken by rulings (Still-TBD lists, architecture gate rule)

**Description.** These are shared setup-era blocks injected into each deliverable's `_CONTEXT.md`, mostly at the initial migration (`7bee9ae41`).
- **Still-TBD lists (58 rows).** The architecture-basis Still-TBD list names items that are ruled since: the physical project package or container (DEC-017, DEC-028), the rule grammar (DEC-022), the solver library (DEC-023), the export formats (SCA-004, OI-004), CI (DEC-025), and so on. Example: `PKG-14_…/DEL-14-01_…/_CONTEXT.md:55`.
- **Architecture gate rule (8 rows).** The gate rule that PKG-01 to PKG-12 wait for PKG-00 readiness (CS-05; `PKG-00_…/DEL-00-01_…/_CONTEXT.md:44`) was superseded by D-43 and HUMAN-STEER-PKG00-EXCLUSION-001.
- **Other injected blocks (5 rows).** Anticipated-artifact, SCA-003 storage-profile and context-envelope blocks.

Each ledger notes that a catch-up needs no decision, and that the items not yet ruled stay open.

**Signature.** SCOPE_REDIRECTED_BY_RULING · LOCAL_DESIGN · STALE_SETUP_SPECIFICATION (57) or STALE_REVIEW_OR_EVIDENCE (14); `RECORD`; `NONE`; AuthorityNeeded `NO`. The most frequent DecisionBasis tokens are DEC-028 (30), DEC-017 (24), DEC-022 (22), SCA-004 (18), DEC-025 (16), SCA-003 (12) and D-43 (11).

**Population.** 71 rows; 66 deliverables in 16 packages (all but PKG-04 and PKG-17).

**Owning authority.** NONE. **Route.** R5_RECORD_REPAIR.

**On-ruling mechanism.** An R5 record repair under one cross-package instrument. It removes the ruled items from each Still-TBD list, citing the ruling. The FIRM resolution asks for a full refresh, not only the item the worker named: DEC-023, DEC-022, DEC-028, SCA-003 and AB-00-04. It marks the gate rule superseded by D-43, and leaves the unruled items (for example public API transport) open. Because the blocks share one body, a single pass can repair them, with per-deliverable variation.

**Risk if unrepaired.** Briefs dispatched from these contexts would present settled decisions as open. That invites relitigating DEC-017, DEC-022 and DEC-028, or divergent local choices.

**Representative keys.**
- `DEL-14-01:CONTEXT#architecture-basis-injection.s02`: `_CONTEXT.md:55` still lists the container and the grammar; these were ruled by DEC-017/DEC-028 and DEC-022.
- `DEL-00-01:CONTEXT#architecture-gate-rule`: CS-05 inherited; `_CONTEXT.md:44`.
- `DEL-15-02:CONTEXT#architecture-basis-injection.s02`: **CONTESTED** tier. LOCAL_DESIGN is sealed, but PROJECT_BASELINE is a candidate because the substance conflicts with DEC-028. The verifier notes that the conventions do not say which tier a "Still TBD item since ruled" takes.

**Exceptions kept visible.** CONTESTED (1): `DEL-15-02:CONTEXT#architecture-basis-injection.s02`. FIRM (2), AuthorityNeeded NO, refresh the whole list: `DEL-11-04:CONTEXT#architecture-basis-injection.s02`, `DEL-11-05:CONTEXT#architecture-basis-injection.s02`. FIELD (3), VerificationClass hygiene: `DEL-10-04:CONTEXT#context-envelope`, `DEL-10-04:CONTEXT#architecture-basis-injection.s02`, `DEL-10-05:CONTEXT#architecture-basis-injection.s02`.

## T5B-C06 — Deliverable TBDs, holds and open questions settled by later rulings

**Description.** This is the bulk of SCOPE_REDIRECTED_BY_RULING. Deliverable-local ScopeOfWork text (219 rows), plus 4 MEMORY rows, 2 ArchitectureBasis rows and 1 STATUS row, records TBDs, open questions, conflicts, holds, trade-offs and gates that a later ruling has settled in whole or in part. Examples:
- the unit catalog and numeric representation, settled by DEC-018 (the D-01 ruling);
- the project container, settled by DEC-028 and DEC-017;
- the expression grammar, settled by DEC-022, with binding members from DEC-038 and DEC-039;
- CI and release items, settled by DEC-025, DEC-057, DEC-059, DEC-089 and DEC-093;
- governance roles, settled by DEC-027 and DEC-079;
- tolerance policy, settled by DEC-024 and DEC-026;
- load extents, settled by DEC-068;
- export formats, settled by SCA-004;
- WCAG selection for touched controls, settled by D-68.

Text first present at the initial migration is STALE_SETUP_SPECIFICATION (F3). Text declared later is STALE_REVIEW_OR_EVIDENCE. The claim's authority is local, and each RemainingWork is a catch-up that cites the ruling and keeps the unruled remainder open.

**Signature.** SCOPE_REDIRECTED_BY_RULING · LOCAL_DESIGN · STALE_SETUP_SPECIFICATION (132) or STALE_REVIEW_OR_EVIDENCE (94); `RECORD`; `NONE`; AuthorityNeeded `NO`. Claim types: DECLARED_STATE 137, REQUIREMENT 36, ACCEPTANCE 33, CONTEXT 17, EXCLUSION 3. The most frequent DecisionBasis tokens are DEC-018 (39), DEC-028 (26), DEC-022 (24), DEC-057 (20), DEC-017 (20), D-01 (20), DEC-027 (18) and DEC-025 (17).

**Population.** 226 rows in 17 packages (all but PKG-13). Deliverables: DEL-00-01, DEL-00-08, DEL-01-02, DEL-01-03, DEL-02-01, DEL-02-02, DEL-02-04, DEL-02-05, DEL-03-01, DEL-03-05, DEL-03-08, DEL-04-04, DEL-04-05, DEL-04-06, DEL-05-01, DEL-05-02, DEL-05-03, DEL-06-01, DEL-06-02, DEL-06-03, DEL-06-04, DEL-06-05, DEL-07-06, DEL-08-01, DEL-08-02, DEL-08-03, DEL-09-01, DEL-09-02, DEL-09-03, DEL-09-05, DEL-10-01, DEL-10-04, DEL-10-05, DEL-11-01, DEL-11-02, DEL-12-01, DEL-12-02, DEL-12-04, DEL-12-05, DEL-14-03, DEL-14-04, DEL-15-02, DEL-16-01, DEL-16-04, DEL-17-06. The heaviest are DEL-02-02 (21 SOW rows), DEL-02-05, DEL-10-04 and DEL-06-02.

**Owning authority.** NONE. **Route.** R5_RECORD_REPAIR.

**On-ruling mechanism.** An R5 record repair, run per deliverable at its next SOW catch-up under a later owner authorisation of R5. Each TBD, open question or gate is restated as settled, citing the ruling named in the row's DecisionBasis, and the unruled remainder stays open. Examples of a remainder: encryption and access policy with PKG-12 (DEL-06-04); the CI-guard severity (see C07). Each repaired SOW then needs the C01 parity re-review.

**Risk if unrepaired.** Contracts keep telling implementers that settled choices are open. That invites rework or local choices that depart from DEC-018, DEC-022 and DEC-028. It also weakens trust in the SOW as the operative contract.

**Representative keys.**
- `DEL-02-02:SOW#CLM-037` (`PKG-02_…/DEL-02-02_…/ScopeOfWork.md:493-501`): "no authoritative catalog; record the minimal set as TBD" is overtaken by DEC-018 (D-01, 2026-06-10).
- `DEL-06-02:SOW#CLM-022.r06` (`…/DEL-06-02_…/ScopeOfWork.md:275`): the grammar decision is TBD and required before implementation; DEC-022 has since settled it.
- `DEL-05-02:SOW#completion-and-reliance-basis-epistemology/AC-001`: **CONTESTED**. Under a boundary reading ("where policies are unresolved, invent nothing") the row is met; under a declaration reading it is stale. It needs one wave-level reading.

**Exceptions kept visible.**
- CONTESTED (2): `DEL-05-02:SOW#completion-and-reliance-basis-epistemology/AC-001`, `DEL-05-03:SOW#completion-and-reliance-basis-epistemology/AC-001`.
- OBSERVED (4), FG-06 rows that extend the contested C05 tier question: `DEL-15-02:SOW#CLM-004.r07`, `DEL-15-02:SOW#CLM-012.r02`, `DEL-15-02:SOW#CLM-019.r09`, `DEL-15-02:SOW#CLM-020.s03`.
- WEAK (2), which ask the rows to name the boundary in Notes: `DEL-16-04:SOW#CLM-009/REQ-16-04-08`, `DEL-16-04:SOW#completion-and-reliance-basis-epistemology/AC-001`.
- FIRM (1), AuthorityNeeded NO, cite SCA-004 and OI-004: `DEL-10-05:SOW#CLM-004.r04`.
- FIELD (19), evidence and VerificationClass hygiene: `DEL-07-06:SOW#CLM-026`, `DEL-01-03:SOW#CLM-006.r11`, the DEL-10-04 SOW and MEMORY rows (`CLM-004.r05`, `CLM-005.r01`, `CLM-005.r02`, `CLM-005.r06`, `CLM-008.s01`, `CLM-012/REQ-10-04-03`, `CLM-014.s02`, `CLM-015.r05`, `completion-and-reliance-basis-epistemology/AC-001`, `CLM-028.s02`, `CLM-031.s01`, `MEMORY.s01`), and the DEL-10-05 SOW rows (`CLM-004.r05`, `CLM-011.r02`, `CLM-014`, `CLM-027.r03`, `CLM-027.r04`).

## T5B-C07 — PROJECT_BASELINE claims overtaken by rulings (ISSUED DEL-01-01 group and four holds)

**Description.** These are rulings that overtook claims at baseline tier, where a catch-up is not a no-decision repair (A2; C6(d)).
- **(i) ISSUED DEL-01-01 governance baseline, 16 rows.** Fifteen LIFECYCLE_REASSESSMENT_REQUIRED rows in FG-DEL-01-01-01 keep the maintainer roster, quorum, release authority, signing and human project authority as TBD (`PKG-01_…/DEL-01-01_…/ScopeOfWork.md:68`, `:80`, `:82`, `:381`). The text was true at issuance (2026-06-03) and was overtaken by DEC-027 (2026-06-11: sole human project authority, quorum one) and by DEC-057 / DEC-089 (v0.1 unsigned with checksum and release-record attestation). The sixteenth row, `…/AC-001` (`ScopeOfWork.md:215`), is ACCEPTED_DIVERGENCE: DEC-081 / D-48 Wave 2 substituted GF-TOKEN text into the ISSUED SOW at six lines (commit `8fac6631a`).
- **(ii) Four non-ISSUED holds, 6 rows.**
  - DEL-00-05: the accessibility-target hold is "never ruled" (`PKG-00_…/DEL-00-05_…/ArchitectureBasis.md:50`), but D-68 (2026-09-15) adopted bounded criteria.
  - DEL-00-07: the external format list is TBD (`…/DEL-00-07_…/ArchitectureBasis.md:80`), yet SCA-004 admitted formats and PKG-17 export packages exist.
  - DEL-08-05 (2 rows): OQ-003 and OQ-005 are TBD (`…/DEL-08-05_…/ScopeOfWork.md:472`), but DEC-058 ruled release-scan severity and ownership. The CI-guard part stays open.
  - DEL-12-05 (2 rows): the explicit-disclosure principle (`…/DEL-12-05_…/ScopeOfWork.md:148`, `:452`) conflicts with DEC-051's owner-configured model-provider channel.

**Signature.** SCOPE_REDIRECTED_BY_RULING · PROJECT_BASELINE with the dispositions LIFECYCLE_REASSESSMENT_REQUIRED (15), STALE_SETUP_SPECIFICATION (4), STALE_REVIEW_OR_EVIDENCE (2) and ACCEPTED_DIVERGENCE (1). BaselineClass ISSUED 16, RULED_CRITERION 3, NONE 3. Layers `LIFECYCLE;RECORD` 15, `SECURITY;RECORD` 2, `CLAIMS;RECORD` 1, `RECORD` 4. AuthorityNeeded OWNER 20, NO 2 (the DEL-08-05 rows).

**Population.** 22 rows. Packages PKG-00, PKG-01, PKG-08, PKG-12. Deliverables DEL-00-05, DEL-00-07, DEL-01-01, DEL-08-05, DEL-12-05.

**Owning authority.** OWNER. **Route.** OWNER_DECISION.

**Owner decisions, with the options in the evidence.**
1. **FG-DEL-01-01-01 (15 rows), one decision on the ISSUED change path** (R0 ruling item 4). The options: reissue DEL-01-01 with the DEC-027 / DEC-057 / DEC-089 values; amend it on the ISSUED change path; or leave the TBD text as issuance history with a pointer to the rulings. The FIELD resolutions on `CLM-004.r05` and `CLM-014` add DEC-081 and DEC-105 to the basis. They also ask whether `CLM-004.r05` (maturity labels and validation disclosure) belongs in a separate group or in FG-01 with the extension disclosed to the owner.
2. **DEL-01-01 AC-001 (1 row), confirmation, not decision.** Did DEC-081 Wave 2 validly reach this ISSUED SOW? The profile routes ISSUED changes through scope change, while DEC-081 says it is not an SCA. The options: confirm, or route the six-line change through scope change.
3. **DEL-00-05.** How far D-68 closes the cross-cutting accessibility hold: fully; in part, with independent usability still held; or not at all, with the target left to DEL-07-06.
4. **DEL-00-07 (OBSERVED).** Whether SCA-004 is the resolving ruling for the boundary documents' format list. The options: yes, and repair the hold; or no. The OBSERVED note says AB-00-07 rev 0.12 still keeps formats TBD, which argues for AUTHORITY_CONFLICT.
5. **DEL-08-05 (2 rows).** Confirm that DEC-058 closes the release-scan parts of OQ-003 and OQ-005, with CI-guard severity still open. The ruling appears unambiguous, so this is likely a confirmation; see R3 observation 5.
6. **DEL-12-05 (2 rows, one CONTESTED).** How to reconcile the explicit-disclosure principle with DEC-051. The options: amend the principle to carve out the owner-configured provider channel; or reopen DEC-051. This is already on the W3 owner list as "DEC-051 provider disclosure". The contested point is the tier: INVARIANT with layer SECURITY under F8, or PROJECT_BASELINE as sealed.

**On-ruling mechanism.** Decision 1, and decision 2 if not confirmed: the ISSUED change path (scope-change workflow, or the reissue path the owner names), then a record repair. Decisions 3–6: an R5 record repair of the named ArchitectureBasis or SOW rows, citing the owner's reading. For decision 6, a change to DEC-051 or to the invariant would go through its own change path.

**Risk if unrepaired.** The ISSUED governance baseline keeps asserting TBD roles that rulings have fixed, so the ISSUED record and the decision register disagree at baseline tier. The DEL-12-05 principle and DEC-051 stay in tension on a security-layer subject. No provider channel is live at the freeze.

**Representative keys.**
- `DEL-01-01:SOW#CLM-005.r04`: "Release authority: TBD" (`ScopeOfWork.md:82`) against DEC-027.
- `DEL-01-01:SOW#completion-and-reliance-basis-epistemology/AC-001`: ACCEPTED_DIVERGENCE on DEC-081, for owner confirmation.
- `DEL-12-05:SOW#CLM-033.r02`: **CONTESTED** tier; DEC-051 (register D-24).

**Exceptions kept visible.** CONTESTED (1): `DEL-12-05:SOW#CLM-033.r02`. OBSERVED (1): `DEL-00-07:AB#open-holds-and-routed-questions.s02`. FIELD (2): `DEL-01-01:SOW#CLM-004.r05`, `DEL-01-01:SOW#CLM-014`.

## T5B-C08 — Write scope and boundary text outgrown under recorded direction

**Description.** Setup-era write-scope and exclusion text says "deliverable folder only" or "no CI/test/repo edits". Later directed work has contradicted it:
- DEL-09-05 owns `docs/RELEASE_QUALITY_GATES.md`, the gate-record and coverage-telemetry tools, and their tests;
- DEL-09-04 edited `docs/VALIDATION_STRATEGY.md` under a sealed-brief override and the 2026-08-11 owner-gates application.

Other directed work went beyond a claim's framing:
- DEL-07-03 GUI now authors private rule packs, where the SOW says reference editing (`…/DEL-07-03_…/ScopeOfWork.md:504`);
- the DEL-11-01 guide gained sections that have no outline slot;
- the DEL-11-01 guide states that shipped surfaces exist while their backing deliverables are IN_PROGRESS.

Under A2 these departures are owner intent carried by merged PRs and directions, at LOCAL_DESIGN tier. The likely resolution is that the deliverable text catches up.

**Signature.** SCOPE_GREW_BY_DIRECTION · LOCAL_DESIGN · STALE_SETUP_SPECIFICATION (6), IMPLEMENTED_UNDOCUMENTED (2) or IMPLEMENTED_DIFFERENTLY (1); `RECORD`; `NONE`; AuthorityNeeded `NO`.

**Population.** 9 rows. Packages PKG-07, PKG-09, PKG-11. Deliverables DEL-07-03, DEL-09-04, DEL-09-05, DEL-11-01.

**Owning authority.** NONE. **Route.** R5_RECORD_REPAIR.

**On-ruling mechanism.** An R5 record repair. Restate each write scope as the paths the deliverable actually owns, citing the directing records. Record the rule-pack authoring surface in the DEL-07-03 SOW, with evaluation still in PKG-06. Add the outline slots for DEL-11-01. For DEL-11-01 CLM-025 (IMPLEMENTED_DIFFERENTLY, "divergent only under a lifecycle reading"), bound the guide's wording to the backing deliverables' state.

**Risk if unrepaired.** Low to moderate. Deliverable boundaries understate what each deliverable owns, which misleads ownership reads (compare T3/T11) and later scope checks.

**Representative keys.**
- `DEL-09-05:SOW#CLM-005` (`…/DEL-09-05_…/ScopeOfWork.md:64-73`): "no CI/test/repo edits" is contradicted by the tools and tests the deliverable owns.
- `DEL-09-04:SOW#CLM-010` (`…/DEL-09-04_…/ScopeOfWork.md:139-152`): the `VALIDATION_STRATEGY.md` exclusion is contradicted by the Tranche B edit.
- `DEL-07-03:SOW#CLM-036.r05` (`ScopeOfWork.md:504`): rule-pack authoring now exists in the GUI.

**Exceptions kept visible.** CONTESTED (1): `DEL-09-05:SOW#CLM-003`. The verifier would split the row: the write boundary keeps STALE_SETUP_SPECIFICATION, while the lifecycle target "SEMANTIC_READY for setup review" becomes STALE_REVIEW_OR_EVIDENCE (the F3 exception, related to SR-1). No split keys were minted.

## T5B-C09 — Scope grown on owner records or awaiting an owner boundary

**Description.** These are two scope-grown findings that a record repair cannot close without the owner.
- **DEL-11-01, 4 rows, FG-DEL-11-01-01.** The SOW calls `docs/user_guide/index.md` read-only for the deliverable (`…/DEL-11-01_…/ScopeOfWork.md:61`, `:249`). The guide was nevertheless created and edited under it. An A3a owner-direction record (`…/HELP-HUMAN-PIPING-20260725-CANDIDATE-BRIEFS-R18/instances/WI-PKG11-DEL1101-EXECUTION/OWNER_ADOPTION.md`, hash-bound, a "guide-only SOW exception") and DEC-107 (iii) support ACCEPTED_DIVERGENCE. The guide predates both acts (2026-05-09), so R4 confirms how far they reach.
- **DEL-07-02 CLM-034, 1 row.** The PROJECT_BASELINE boundary rationale says the inspector is a convenience surface (`…/DEL-07-02_…/ScopeOfWork.md:389-397`). Entity creation forms and direct table editing landed under DEL-07-02 (PRs #832–#834). The SCA-009 annex lands the material and section editors in DEL-07-03, and no record draws the line.

**Signature.** SCOPE_GREW_BY_DIRECTION · LOCAL_DESIGN · ACCEPTED_DIVERGENCE (4) and SCOPE_GREW_BY_DIRECTION · PROJECT_BASELINE · IMPLEMENTED_UNDOCUMENTED (1); `RECORD`; `NONE`; AuthorityNeeded OWNER on all 5.

**Population.** 5 rows. Packages PKG-07, PKG-11. Deliverables DEL-07-02, DEL-11-01.

**Owning authority.** OWNER. **Route.** OWNER_DECISION.

**Owner decisions, with the options in the evidence.**
1. **DEL-11-01 (confirmation under A3a).** Do the R18 guide-only exception and DEC-107 cover the SOW's read-only text, including the pre-exception edits? The options: confirm, and the rows stand as accepted divergence; or direct a SOW catch-up. The latter is a record repair that DEC-107's leave-SOW-untouched instruction currently prevents. This matches the W3 owner item "the DEL-11-01 user-guide exception scope".
2. **DEL-07-02.** Where do creation forms and direct table editing sit? The options: DEL-07-02, and its SOW records it; or DEL-07-03, consistent with the SCA-009 annex rows 3 and 17. Reassigning ownership between deliverables would be a scope-change handoff.

**On-ruling mechanism.** Decision 1: owner confirmation recorded at R4. If a catch-up is directed instead, an R5 record repair. Decision 2: an R5 record repair of the chosen SOW(s). If ownership moves, the scope-change workflow (SCOPE_CHANGE_HANDOFF).

**Risk if unrepaired.** For DEL-11-01, the SOW contradicts its own artifact. For DEL-07-02 and DEL-07-03, ownership of shipped editing surfaces stays ambiguous (see T3/T11).

**Representative keys.**
- `DEL-11-01:SOW#CLM-010.s02`: `OWNER_DIRECTION_RECORD` in Notes; DEC-107.
- `DEL-11-01:SOW#CLM-018.r05` (`ScopeOfWork.md:249`): the write scope is the deliverable folder only.
- `DEL-07-02:SOW#CLM-034`: PRs #832–#834 against the SCA-009 annex.

**Exceptions kept visible.** No resolution rows. All four DEL-11-01 rows rest on an A3a owner record, which goes to the owner for confirmation, not decision.

## T5B-C10 — Contract version advanced: analysis-record 0.2 and the schema dispatcher

**Description.** Since `6bb26b118` (2026-09-14), under D-67, `schemas/analysis_run.schema.json` has been an exact-version dispatcher over the Python 0.1 record, the legacy desktop 0.1 record and the strict 0.2 record.
- **DEL-14-02, 6 rows, FG-DEL-14-02-01.** The SOW describes only the Python 0.1 `SORTED_COMPACT_JSON` record (`PKG-14_…/DEL-14-02_…/ScopeOfWork.md:66`, `:86`, `:100`, `:204`). It never mentions the 0.2 record that the product now builds with the `openpipestress_jcs_ijson_v1` checked profile.
- **DEL-00-06, 2 rows, FG-DEL-00-06-03.** The ArchitectureBasis anchors the warning-class enum in the dispatcher file (`PKG-00_…/DEL-00-06_…/ArchitectureBasis.md:25`), but the enum now lives in `analysis_run.v0.1.schema.json`, with results 0.2 holding the result-boundary enum. The substance holds for 0.1; only the anchor moved with the version.

**Signature.** CONTRACT_VERSION_ADVANCED · LOCAL_DESIGN · STALE_REVIEW_OR_EVIDENCE (7) or STALE_SETUP_SPECIFICATION (1); `RECORD`; `NONE`; AuthorityNeeded `NO`.

**Population.** 8 rows. Packages PKG-00, PKG-14. Deliverables DEL-00-06, DEL-14-02.

**Owning authority.** NONE. **Route.** R5_RECORD_REPAIR.

**On-ruling mechanism.** An R5 record repair. Update the DEL-14-02 SOW hash, schema, construction and acceptance text to describe the D-67 0.2 contract beside the retained 0.1 profile. Repoint the DEL-00-06 enum anchor to the versioned schemas. Both follow D-67's adopted plan. The label question is separate: the W3 owner item that "JCS-compatible" labels are not RFC 8785, and the PKG-17 frozen-contract rename of `openpipestress_jcs_ijson_v1`. The repair should describe the label as it is, and T8's "Tier of in-scope REQs" cluster includes DEL-14-02. Neither changes this class's routing.

**Risk if unrepaired.** The accepted DEL-14-02 contract describes a record version the product no longer primarily builds. Readers relying on the SOW would mis-state the hash basis of current records.

**Representative keys.**
- `DEL-14-02:SOW`: the SURFACE row carrying the common defect (C1).
- `DEL-14-02:SOW#CLM-006` (`ScopeOfWork.md:90-105`): step 5 prescribes only `SORTED_COMPACT_JSON` (`:100`).
- `DEL-00-06:AB#normative-requirements.s01`: the anchor moved to `analysis_run.v0.1.schema.json`.

**Exceptions kept visible.** No resolution rows.

## Coverage

- **Population.** `R3/CORPUS_CLAIMS.csv` rows with `Divergent = YES` and effective CauseTag in {EVIDENCE_OVERTAKEN, SCOPE_REDIRECTED_BY_RULING, SCOPE_GREW_BY_DIRECTION, CONTRACT_VERSION_ADVANCED}: **583** (202 + 359 + 14 + 8). This matches the launch message and `R3_PLAN.md`.
- **Output.** `T5B_CLASSES.csv` has 583 body rows and the sentinel `#END,,583`. Class counts: C01 175, C02 12, C03 15, C04 40, C05 71, C06 226, C07 22, C08 9, C09 5, C10 8, total 583.
- **Check run.** A read-only Python script re-read `CORPUS_CLAIMS.csv` and `T5B_CLASSES.csv` and checked the following. The set of (ClaimKey, DeliverableID) pairs is identical in both, so no key is missing and none is extra. Each pair appears exactly once in the output. Every ClassID is one of the ten above. The per-class counts sum to 583. The sentinel count equals the body count. The file uses CRLF line endings.
- **Resolution-class visibility.** Every row in the population with a CONTESTED (7), OBSERVED (9), FIELD (27), FIRM (4), WEAK (2) or OWNER_CONFIRMED (39) resolution is named in its class's exceptions line. The 39 OWNER_CONFIRMED rows are all in C04 and are listed there as its population.
- **Inputs.** The brief (SHA-256 `4463e540…f737f6a309`, verified); `R3_PLAN.md`; `CORPUS_CLAIMS.csv`; the sealed forward ledgers outside `superseded_*`, for Notes, DecisionBasis, evidence and RemainingWork; `CONVENTIONS.md`; `CANONICAL_SITUATIONS.md` (CP-09, CS-05); `EVIDENCE_MAP.csv`; `CLAIM_KEYS_V2.csv` (line ranges); `WAVES/W3/W3_ASSESSMENT.md`; and the freeze at `00115c719` for the `path:line` anchors. No draft resolutions file (`RESOLUTIONS_DRAFT*.csv`, `RESOLUTIONS_MERGED_DRAFT.csv`) and no other task's file was read. Nothing was built or run beyond read-only scripts.

## R3 observations

These are observations, not corrections. No effective value is changed.

1. **DEL-07-09 cause split within one finding.** Three DEL-07-09 rows carry FIELD resolutions correcting CauseTag to RECORD_DRIFT: `…ROUTING#all-row-routing-and-bounded-residuals.r02`, `…ORGANIZATION_CONTRACT#evidence-disposition` and `…ORGANIZATION_CONTRACT#…pend.s04`. Under the effective-values rule a FIELD row that sets no Disposition is not applied, so they stay EVIDENCE_OVERTAKEN in this partition. Their six siblings in the same finding (the N7 rereview is pending but done) were not corrected. If Agent 0 adopts RECORD_DRIFT for the finding, all nine C03 DEL-07-09 rows would move to T4B's partition together, not three alone. The route (R5_RECORD_REPAIR) is the same either way.
2. **Tier convention gap for "Still TBD item since ruled".** A CONTESTED resolution (`DEL-15-02:CONTEXT#architecture-basis-injection.s02`) and four OBSERVED FG-06 rows ask whether such rows are LOCAL_DESIGN or PROJECT_BASELINE. The same pattern is sealed LOCAL_DESIGN in all 58 C05 Still-TBD rows and in many C06 rows. If R4 rules PROJECT_BASELINE, C05 and parts of C06 would move from R5_RECORD_REPAIR to an owner confirmation. This needs one corpus-wide reading, not per-row handling.
3. **Sequencing of parity after repairs.** C01 has 175 rows re-reviewed against current SOW bytes. The C03, C05–C08 and C10 SOW repairs change those bytes, so the re-review should run after them.
4. **C04 membership outside the OWNER_CONFIRMED set.** `DEL-11-05:SOW#CLM-031` carries the same SEMANTIC_READY premise in a SOW row. It has no resolution row, so the Direction 8 confirmation does not formally cover it. It is placed in C04 because its repair depends on the same owner choice.
5. **PROJECT_BASELINE with AuthorityNeeded NO.** The two DEL-08-05 rows are PROJECT_BASELINE and RULED_CRITERION, yet their AuthorityNeeded is NO. The DEC-058 restatement looks like a no-decision catch-up. Under A2 a PROJECT_BASELINE divergence goes to R4, so they are routed to owner confirmation in C07. Agent 0 may instead treat them as R5 record repair if R4 accepts that DEC-058 is unambiguous.
6. **DEL-05-01 VER-001 FindingGroup.** The FIRM resolution asks for a shared FindingGroup with `CLM-016/REQ-05-01-003`. That is an "other correction", so it is not applied. Both rows are in C02, which carries out the grouping in effect.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
