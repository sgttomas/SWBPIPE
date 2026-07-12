# Package Concordance Summary — PKG-00 (Software Architecture Runway)

> **Epistemic status: agent-authored, non-operative evidence** (R2 wave W1,
> run DELIVERABLE_CONCORDANCE_2026-07-11_1305). Computed from the eight W1
> claim ledgers after the fan-in verification pass and fable re-runs. No
> disposition here is an owner or engineering ruling; nothing edits any
> deliverable. Frozen source state: `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.

## Census

8 deliverables (DEL-00-01..08), all `IN_PROGRESS`; 137 claim rows; ledgers
`WAVES/W1/CLAIM_CONCORDANCE_DEL-00-0*.csv`. Verification:
`WAVES/W1/W1_VERIFICATION_PKG-00.md` (5 of 8 ledgers initially DEFECTIVE on
one shared pattern; all repaired by fable re-runs; final state revalidated).

## ClaimType × Disposition (computed from ledgers)

| ClaimType | total | ALIGNED | PARTIALLY_IMPLEMENTED | STALE_SETUP_SPECIFICATION |
|---|---|---|---|---|
| REQUIREMENT | 40 | 37 | 3 | 0 |
| ACCEPTANCE | 26 | 26 | 0 | 0 |
| EXCLUSION | 23 | 23 | 0 | 0 |
| DECLARED_STATE | 48 | 22 | 0 | 26 |

`SelectableUnderCurrentLoop=YES`: 0 rows. No REMAINING_WORK,
IMPLEMENTED_UNMAPPED, UNKNOWN, or AUTHORITY_CONFLICT rows.

## Package reading

PKG-00 is a documentation/architecture-runway package: requirement substance
is overwhelmingly ALIGNED at runway grain (accepted architecture bases
SCA-001/AB-00-0X realized downstream), while the staleness burden sits
entirely on declared-state surfaces. The dominant finding — present in all
eight kits — is the SOFTWARE_DECOMP **rev-0.7 authority-pointer drift**
(Specification/Datasheet/Procedure cite revision 0.7; the frozen decomp is
revision 0.8, `status: current_basis`), encoded STALE_SETUP_SPECIFICATION on
26 DECL rows per the fan-in adjudication under addendum 4, with the decomp
§13 sanction ("stale … until refreshed") carried as immateriality context,
AuthorityNeeded=NO except where a kit also carries overtaken TBD registers
(DEL-00-04/05 → OWNER). **Owner-calibration caveat:** if the owner later
calibrates this corpus-wide pattern to ALIGNED-with-note, the affected rows
flip back mechanically — the drift facts are recorded in-row either way.

Substantive non-staleness findings:

- `DEL-00-02-REQ-005` PARTIALLY_IMPLEMENTED (REVIEW): review-gate mechanisms
  exist but no concrete boundary-drift-detection check is specified.
- `DEL-00-05-REQ-004` PARTIALLY_IMPLEMENTED (OWNER): selection modeled,
  property-inspector architecture absent; no record delegates it to
  DEL-07-02 (verifier-endorsed encoding).
- `DEL-00-07-REQ-005` PARTIALLY_IMPLEMENTED: handoff obligations partially
  defined.
- DEL-00-04/05 Datasheet TBD registers overtaken by accepted rulings
  (SCA-003/AB-00-04 persistence choices; DEC-009 GUI framework/viewport) —
  R5 repair candidates routed OWNER.

## Decision findings (routing, not rulings)

- OWNER: DEL-00-04 DECL-002/003, DEL-00-05 REQ-004/DECL-002/003 (overtaken
  TBD registers; inspector-architecture home).
- REVIEW: DEL-00-01 REQ rows ×5 (human acceptance pending), DEL-00-02
  REQ-005 + EXC-001 context, DEL-00-06 REQ/basis rows ×3.
- No ENGINEERING or SCOPE_CHANGE routing in this package.

## Unmapped implementation

None ledgered. DEL-00-08's 14–17 co-attributed shared test surfaces resolve
to their owning implementing deliverables (recorded in its notes); the
unexecuted-suite gap belongs to the owning deliverables' waves.

## Conflicts and unknowns

None (zero AUTHORITY_CONFLICT / UNKNOWN rows). The D-41 frozen-register
`AWAITING_RULING` state is run mechanics per RUN_BASIS, not a conflict.

## Verification and repair record

Fan-in (fable, high effort): DEL-00-01/03/08 SOUND; DEL-00-02/04/05/06/07
DEFECTIVE on the rev-drift encoding split (3 ledgers STALE vs 5
ALIGNED-or-silent on textually identical drift). Adjudication: STALE side
correct under addendum 4. Fable re-runs re-encoded 13+ rows across the five
ledgers (each independently re-verified the drift before re-encoding; zero
disagreements); addendum-6 ladder harmonized (DEL-00-02 EXC-001, DEL-00-05
ACC/EXC rows REVIEWED→UNVERIFIED after the D-40 lock-review reset). All 137
rows revalidated structurally clean; notes histograms reproduce.

## Cross-ledger risks carried forward (W2–W5 calibration / R3)

1. **Rev-pointer drift is corpus-wide**: every kit citing decomp revision
   0.7 will hit the same encoding; W2–W5 pilots should apply the W1
   adjudication (STALE on the declared-state row, drift facts in-row,
   owner-calibration caveat noted once per ledger).
2. SourceReliability on ACC/EXC rows resting on the D-40-reset PKG00 lock
   review: UNVERIFIED is the harmonized encoding.
3. ACCEPTANCE/EXCLUSION census grain varies across near-identical kits
   (ACC 2–4 / EXC 2–4) — disclosed; histogram ClaimType counts are not
   comparable at face value across packages (PKG-02/03 encoded ACCEPTANCE=0).
4. AuthorityNeeded on pending-human-review REQ rows varies (REVIEW vs NO) —
   dedupe at R3 aggregation.

## Fences

Discovery read-only; frozen tree clean throughout (verifier and every
repair pilot re-checked porcelain); no lifecycle/DAG/scope change proposed
as operative; no F-PIP-1..5 claim language outside attributed quotes; all
dispositions are agent judgments.
