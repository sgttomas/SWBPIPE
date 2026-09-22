# R3 synthesis: piping whole-corpus reconciliation

This is R3 (cross-package synthesis) of `RECON_2026-09-21_WHOLE_CORPUS`. It
reads the 102 accepted R2 ledgers together, with the adopted resolutions. It
says what the deliverable corpus and the frozen code (`00115c719`) show when
compared, and it routes each finding to the holder who must act. It changes
nothing. Coverage, reproduction and limits are in `COVERAGE_AND_QA.md`. Read
the limits before relying on any figure here.

## 1. Summary

- **What was compared.** 102 deliverables, 9,889 claim rows (every issued
  key), and 598 code capabilities.
- **Aligned.** 5,052 rows (51%): the deliverable text and the frozen code
  agree. Another 1,538 rows are containers or non-normative text.
- **Divergent.** 3,299 rows (33%). Most of this is record drift, not product
  failure:
  - 2,099 rows are record repairs: stale pins and pointers, residue from the
    four-document era, and setup-era TBDs that the code has since answered.
  - 444 rows need an owner decision.
  - 375 rows are code-fix candidates.
  - 295 need review.
  - 17 go to scope change and 11 to engineering authority.
  - 58 need no action.
- **Ownership gaps.** Four product areas have no owning deliverable:
  - the product solve path (`core/product_physics` and the DEC-044 nonlinear
    loop);
  - the desktop workspace shell;
  - the runtime model-operation applier;
  - the load-case and self-weight editors, which DEC-094 lands where the
    receiving deliverable's envelope excludes editors.
- **Unmapped capabilities.** 130 capabilities have no owner. Of these, 57 are
  product capabilities, 38 are routing gaps, 34 are tooling or test
  infrastructure, and 1 is a complementary shared split. A further 68 are
  only partly owned, and 6 more routing gaps sit among the partly owned.
- **Engines no product path calls.** 666 rows rest on code that no product
  path calls. The main cases are the handoff and export Python engines, the
  comparison and model-state engines, and the whole PKG-13 constraint stack.
  Whether these enter the product is an owner question.
- **The owner's gate.** R4 turns these into 29 decision packets. The largest
  are:
  - DEC-009 (Python engines where a Rust core was adopted);
  - the JSON hash basis;
  - 13 hold topics the code settled without a ruling;
  - one rename ruling;
  - the SEMANTIC_READY lifecycle advance (Direction 8);
  - the ISSUED DEL-01-01 change path;
  - which handoff path is canonical;
  - the status of PKG-13.

## 2. Corpus picture

**Effective dispositions over all 9,889 rows** (`PACKAGE_SUMMARY.csv`):

| Disposition | Rows |
|---|---:|
| ALIGNED | 5,052 |
| STALE_REVIEW_OR_EVIDENCE | 1,522 |
| STALE_SETUP_SPECIFICATION | 952 |
| NOT_ASSESSED | 902 |
| COVERED_BY_CHILDREN | 636 |
| PARTIALLY_IMPLEMENTED | 514 |
| IMPLEMENTED_DIFFERENTLY | 130 |
| UNKNOWN | 51 |
| DOCUMENTED_UNIMPLEMENTED | 49 |
| REMAINING_STATE_MISMATCH | 29 |
| LIFECYCLE_REASSESSMENT_REQUIRED | 16 |
| ACCEPTED_DIVERGENCE | 14 |
| VERIFIED_NOT_VALIDATED | 14 |
| IMPLEMENTED_UNDOCUMENTED | 5 |
| ENGINEERING_AUTHORITY_REQUIRED | 2 |
| AUTHORITY_CONFLICT | 1 |

**Divergence by package.** The share of rows that diverge ranges from 25%
(PKG-03, PKG-13) to 41% (PKG-14). PKG-09 and PKG-10 are at 40% and PKG-04 at
38%. No package stands out as broken. The variation follows how much each
package's text predates the July–September code growth.

## 3. Classes (every divergent row, exactly once)

These are the 55 classes from tasks T4A–T7, grouped by route. The table is
generated from `CLASS_INDEX.csv`. Each row's class is in
`CLASS_ASSIGNMENTS.csv`. Class definitions, representative evidence and
on-ruling mechanisms are in `R3/TASKS/<task>_CLASSES.md`.

- **Authority versus route.** Some classes carry an owning authority that
  differs from their route. For example, T5A's record repairs need review
  first, and T4A-C02's repair waits on an owner treatment choice. The
  handoffs carry this as `BlockedOnPacket`.

**OWNER_DECISION** — 16 classes, 444 rows

| Class | Name | Rows | Authority |
|---|---|---:|---|
| T6-C04 | Gap blocked on a held owner selection | 87 | OWNER |
| T4B-C01 | Rename and identity residue (single R4 ruling) | 86 | OWNER |
| T7-C01 | CP-10 holds settled in code without a ruling | 50 | OWNER |
| T4B-C03 | PKG-00 SEMANTIC_READY injection sub-claims (SR-1) | 42 | OWNER |
| T5B-C04 | SR-1: "PKG-00 at SEMANTIC_READY supplies the architecture basis" | 40 | OWNER |
| T7-C05 | Behaviour contradicts SOW; owner rules intent before fix | 27 | OWNER |
| T5B-C07 | PROJECT_BASELINE claims overtaken by rulings (ISSUED DEL-01-01 group and four holds) | 22 | OWNER |
| T4B-C04 | Declarations citing evidence absent from the freeze (CP-08) | 17 | OWNER |
| T5A-C05 | Code departs from text on owner-reserved points | 17 | OWNER |
| T4A-C06 | Pointer rows tied to open owner clusters (DEC-009, export plan, release-label floor) | 14 | OWNER |
| T7-C02 | JSON hash-basis departure from AB-00-04 (JCS) | 11 | OWNER |
| T4A-C08 | Pins on the ISSUED DEL-01-01 | 9 | OWNER |
| T7-C03 | DEC-009 Rust-core baseline vs Python engines | 7 | OWNER |
| T4B-C02 | ISSUED and baseline-bearing record drift | 5 | OWNER |
| T5B-C09 | Scope grown on owner records or awaiting an owner boundary | 5 | OWNER |
| T7-C04 | Authority silent or in conflict (non-CP-10) | 5 | OWNER |

**CODE_FIX_CANDIDATE** — 5 classes, 375 rows

| Class | Name | Rows | Authority |
|---|---|---:|---|
| T6-C01 | Deliverable-local implementation remainder | 180 | NONE |
| T6-C02 | Baseline-bound implementation remainder | 116 | NONE |
| T7-C06 | Code/test fix candidates (no ruling needed) | 42 | REVIEW |
| T6-C03 | Protected-subject implementation remainder | 34 | REVIEW |
| T4A-C07 | Product documents carrying the dead or stale pointer | 3 | NONE |

**ENGINEERING_AUTHORITY** — 2 classes, 11 rows

| Class | Name | Rows | Authority |
|---|---|---:|---|
| T7-C07 | Engineering validation and reference-model authority | 7 | ENGINEERING |
| T6-C06 | Engineering convention or qualification pending | 4 | ENGINEERING |

**REVIEW** — 7 classes, 295 rows

| Class | Name | Rows | Authority |
|---|---|---:|---|
| T5B-C01 | SOW parity and verification-method records no longer bind the frozen SOW (CP-09) | 175 | REVIEW |
| T6-C05 | Human review disposition pending | 35 | REVIEW |
| T7-C09 | Protected-content / IP-data review record not located | 31 | REVIEW |
| T5A-C08 | Cause or vehicle contested in adopted resolutions | 18 | REVIEW |
| T7-C10 | Other evidence not located (smallest checks) | 14 | REVIEW |
| T5B-C02 | Protected-content and boundary review records predate later changes | 12 | REVIEW |
| T5A-C06 | Local semantic departures (adopt or restore) | 10 | REVIEW |

**SCOPE_CHANGE_HANDOFF** — 1 classes, 17 rows

| Class | Name | Rows | Authority |
|---|---|---:|---|
| T6-C07 | Behaviour landed elsewhere, placement unresolved | 17 | SCOPE_CHANGE |

**R5_RECORD_REPAIR** — 20 classes, 2099 rows

| Class | Name | Rows | Authority |
|---|---|---:|---|
| T4B-C05 | Four-document residue (CP-01) | 336 | NONE |
| T5B-C06 | Deliverable TBDs, holds and open questions settled by later rulings | 226 | NONE |
| T4A-C02 | D-41-era "current declaration" blocks pinning rev 0.8 and DAG-007 | 213 | OWNER (treatment choice) |
| T5A-C02 | Setup-era TBDs and future-tense product text answered by implementation | 207 | REVIEW |
| T5A-C01 | Setup-session framing overtaken | 205 | REVIEW |
| T4A-C01 | Keyed `_CONTEXT.md` setup-block drift (CS-01, CS-04, CS-06) | 198 | NONE |
| T4A-C03 | Other revision, graph and amendment pins (SOW, AB, CONTEXT `.sNN`) | 190 | NONE |
| T5A-C03 | Post-migration declarations overtaken | 94 | REVIEW |
| T4A-C05 | Removed-file, renumbered-section and relocated-anchor pointers | 86 | NONE |
| T5B-C05 | Injected `_CONTEXT.md` blocks overtaken by rulings (Still-TBD lists, architecture gate rule) | 71 | NONE |
| T4A-C04 | `_CONTEXT.md` SURFACE rows restating the keyed pins (contested split) | 68 | REVIEW |
| T4B-C06 | Stale status and memory surfaces (CP-05 and analogues) | 59 | NONE |
| T4B-C08 | Other declarations overtaken by later records or rulings | 59 | NONE |
| T4B-C07 | Remaining items overtaken by rulings or events | 21 | NONE |
| T5A-C04 | Model-operation records omit the ruled runtime seam | 19 | REVIEW |
| T5B-C03 | Review-pending declarations and test-count snapshots overtaken | 15 | NONE |
| T5B-C08 | Write scope and boundary text outgrown under recorded direction | 9 | NONE |
| T7-C11 | Record text inaccurate at authoring (OTHER) | 9 | NONE |
| T5B-C10 | Contract version advanced: analysis-record 0.2 and the schema dispatcher | 8 | NONE |
| T5A-C07 | Stale Remaining items | 6 | REVIEW |

**NO_ACTION** — 4 classes, 58 rows

| Class | Name | Rows | Authority |
|---|---|---:|---|
| T6-C08 | Gap held by a named ruling | 36 | OWNER |
| T7-C08 | Owner-held independent validation, accurately recorded | 10 | OWNER |
| T6-C09 | Sanctioned SCA-009 re-point (exception: CONTESTED) | 9 | OWNER |
| T6-C10 | Conditional example rules met by absence (exception) | 3 | NONE |

## 4. Ownership and the final unmapped set

**Capability disposition.** Every capability that is not single-owner (296
of 598) has a disposition in `CAPABILITY_DISPOSITIONS.csv`:

| Disposition | Count |
|---|---:|
| PARTIAL_UNOWNED_REMAINDER | 68 |
| UNKEYED_SCOPE_GAP | 57 |
| PRODUCT_UNOWNED | 57 |
| ROUTING_GAP | 44 |
| NON_DELIVERABLE | 35 |
| SHARED_OK | 34 |
| DUPLICATE | 1 |

The 302 single-owner capabilities were affirmed by their owners' verified
reverse passes.

**The final unmapped set** has three parts:
- 57 PRODUCT_UNOWNED capabilities. No deliverable owns them.
- 44 ROUTING_GAP capabilities. A deliverable plausibly owns each one, but
  was not routed it or declined it. In 30 of the 44, the primary owner's
  package was never routed the capability in R1 (`ROUTING_GAPS.csv`).
- The unowned parts of 68 capabilities that are only partly owned.

These unmapped capabilities cluster in the four areas named in the summary:
- the product solve path (T1, T3 G2/G3);
- the desktop workspace shell (T2 G1, T3 G4);
- the runtime model-operation applier (T3 G1);
- the editors and panels (T1, T3 G5).

**Other ownership findings.**
- **One duplicate.** CAP-VIEW-017 is claimed by two deliverables. DEL-07-05
  is better supported.
- **Circular remainders.** Fifteen remainders have each deliverable pointing
  at the other. The reverse-pass rules have no tie-break for this (packet C6).
- **Unkeyed scope.** Fifty-seven capabilities are built but covered by no
  issued key. Most need record repair or new keys (H1 KEY_ISSUE).

The routes are packets B1–B6 (owner choices) and the scope-change handoff
(`SCOPE_CHANGE_HANDOFF/`).

## 5. Contested clusters (T8)

R2 left seven corpus-wide questions contested. T8 found enough evidence to
propose readings for five of them. Packet C7 puts these readings to the owner
for confirmation; they change no row value until then:
- **SR-1:** the cause is RECORD_DRIFT.
- **Unit vocabulary:** the tier follows the gap.
- **F1 on CONTEXT rows:** F1 reaches them.
- **In-scope REQ tier:** decided by the scope-item test.
- **Acceptance workflow:** the cause is DEFERRED_BY_RULING.

Two need an owner ruling:
- DEC-009 (A1);
- the deleted export plan (A7).

In 113 rows, T8's proposed route differs from the class route. Both views
are kept (`T8_ROUTE_DISAGREEMENTS.csv`).

## 6. Lifecycle, Remaining and stale evidence (T9)

- **Remaining census.** It is complete: 151 Remaining items across 65
  deliverables, and 37 deliverables record NONE. Of these, 29 rows no longer
  match their true state; most are gates that have since been met or ruled.
  Seventeen W1 Remaining rows marked ALIGNED predate the OPEN_ACTION marker
  and should be read as open work in the R6 census.
- **SEMANTIC_READY.** 83 claims in 81 deliverables still say PKG-00 is at
  SEMANTIC_READY. All eight PKG-00 status files read IN_PROGRESS. The owner
  has confirmed the status is outdated (Direction 8), so the advance is
  packet A5.
- **DEL-01-01.** Its ISSUED text was edited after issue, under DEC-081 Wave 2.
  One change-path decision covers it (A6).
- **Stale evidence.** 1,522 rows fall into seven subclasses. Basis pins
  account for 737. Parity records that no longer bind the frozen SOW account
  for 169; they should be re-reviewed after the SOW repairs, not before.
- **Verified, not validated.** 14 rows: 9 are owner-held under D-68, 4 need
  engineering authority, and 1 waits on the external prover (C3).

## 7. Cross-package findings (T11) and unreached engines (T12)

**Dependencies.**
- 30 local dependency rows in 15 deliverables are ACTIVE but RETIRED in
  DAG-010.
- 651 of the 1,402 active DAG-010 edges cite evidence missing at the freeze.
- DEL-07-09 has no edge to PKG-00, unlike the other 93 deliverables that
  carry a dependency register.

These go to the scope-change and DAG follow-on, which is outside this run.

**Terminology.**
- "JCS-compatible" hashing is read six ways across 13 deliverables (A2).
- What counts as a protected-content review record is unsettled (C2).

**Evidence reused with opposite meanings.** The clearest case: diagnostic
fields are lost where diagnostics cross a boundary. DEL-00-06 records this
as a possible defect, while three other deliverables are ALIGNED on the same
surfaces (H3, engineering).

**Unreached engines (T12).** 666 rows sit on code that no product path
calls, in eight clusters. 436 need an owner decision on whether the code
enters the product (B7–B10). 118 need no action (test harness and fixtures).
98 further ALIGNED rows cite these engines without the marker (unsampled or
absence claims), so the rows relying on them may exceed 666.

## 8. July cross-check (T10)

The July concordance is context only. It predates the July R5 repairs, and
its claim grain is about four times coarser.
- **Persistence.** 97 of 101 deliverables were divergent in both July and
  now.
- **Requirement grain.** 424 July claims could be matched to ours by
  requirement ID. Of these:
  - 22 are resolved;
  - 21 persist, mostly partial slices;
  - 86 that July called aligned now diverge. Eleven of those are possible
    defects, proposed for review.
- **Unmapped surfaces.** Five of July's eight unmapped surfaces are still
  unowned.
- **D-42.** It was ruled on 2026-07-15 (DEC-076). SURF-021 is shared export
  infrastructure, not an ownership gap.

## 9. No-action register

408 rows or items carry no repair, each with its stated reason
(`NO_ACTION_ROWS.csv`, 58 from classes, 18 from T8, 80 from T9, 134 from T11
and 118 from T12). Recording them keeps the R5 and R6 accounting exact.
Fifteen of them are class-routed elsewhere (`ClassRoute`); for those, the
class route governs unless the owner accepts T8's reading. The 35
NON_DELIVERABLE and 34 SHARED_OK capabilities are recorded the same way in
`CAPABILITY_DISPOSITIONS.csv`.

## 10. Outputs and what comes next

**Deterministic tables** in `R3/`:
- `CORPUS_CLAIMS.csv`, `PACKAGE_SUMMARY.csv`, `CLUSTER_MATRIX.csv`;
- `CAPABILITY_COVERAGE.csv`, `REMAINING_CENSUS.csv`;
- `CLASS_INDEX.csv`, `CLASS_ASSIGNMENTS.csv`, `ROUTING_GAPS.csv`;
- `NO_ACTION_ROWS.csv`, `CAPABILITY_DISPOSITIONS.csv`,
  `T8_ROUTE_DISAGREEMENTS.csv`.

**Task analyses:** `R3/TASKS/`.

**Handoffs.** These are proposals and nothing in them has been executed:
- `R3/SCOPE_CHANGE_HANDOFF/`;
- `R3/CODE_FIX_BRIEF_CANDIDATES/`;
- `R3/ENGINEERING_AUTHORITY/`.

**R4:**
- `R4/DECISION_PACKETS/`: 29 packets in P1–P3, each with its holder.
- `R4/R5_TRANCHE_PROPOSAL/`: record-repair tranches, blocked where a packet
  decides first.

**Next.** The independent R3/R4 review comes first, then the PR, then the
owner's R4 gate. Under D-73 the run stops at R4. R5 repairs and R6
backcheck and closure each need a separate owner authorization.
Scope-change and DAG rebuild are follow-ons outside this run.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
