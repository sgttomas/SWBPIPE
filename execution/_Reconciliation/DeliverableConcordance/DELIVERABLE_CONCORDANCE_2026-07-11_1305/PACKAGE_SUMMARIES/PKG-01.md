# Package Concordance Summary — PKG-01 (Governance, IP Boundary, and Professional Responsibility)

> **Epistemic status: agent-authored, non-operative evidence** (R2 wave W1,
> run DELIVERABLE_CONCORDANCE_2026-07-11_1305). Computed from the four W1
> claim ledgers after the fan-in verification pass. No disposition here is an
> owner or engineering ruling; nothing edits any deliverable. Frozen source
> state: `551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Owner-ruled: all PKG-01
> discovery pilots were fable (F-PIP-2 fence adjacency; DEL-01-01 ISSUED).

## Census

4 deliverables; 87 claim rows. DEL-01-01 is the corpus's only **ISSUED**
deliverable (read-only baseline, F-PIP-3): all 22 rows copy
LifecycleState=ISSUED, no bootstrap row exists (its `_STATUS.md` was never
flipped), and every change-shaped finding routes
`AuthorityNeeded=SCOPE_CHANGE` with no repair proposed. Verification:
`WAVES/W1/W1_VERIFICATION_PKG-01.md` — **all four ledgers SOUND**, zero
re-runs, spot-checks 44 PASS / 9 QUALIFIED / 0 FAIL.

## ClaimType × Disposition (computed from ledgers)

| ClaimType | total | ALIGNED | ACCEPTED_DIVERGENCE | STALE_SETUP_SPECIFICATION |
|---|---|---|---|---|
| REQUIREMENT | 37 | 37 | 0 | 0 |
| ACCEPTANCE | 15 | 14 | 1 | 0 |
| EXCLUSION | 6 | 6 | 0 | 0 |
| DECLARED_STATE | 25 | 16 | 0 | 9 |
| REMAINING_WORK | 4 | 2 | 2 | 0 |
`SelectableUnderCurrentLoop=YES`: 2 rows (DEL-01-03 DECL-005 surface row via
its ungated listed residual, and REM-003), mechanical derivations only.

## Package reading

Requirement substance is fully ALIGNED across the governance/IP/claims
policies — every requirement row verified with bind-current evidence, and
the F-PIP-2 fence held everywhere (verifier ran a full cell scan of
DEL-01-04: every forbidden-claims phrase is an attributed quote, negation,
or description of the policy's own prohibition text). The four
ACCEPTED_DIVERGENCE rows all rest on named human records meeting the
addendum-11 bar (DEC-027 closed-contributions deferral; the 2026-06-05
explicit_human_approval deferral; the 2026-06-04 wording-approval deferral
with REV-W001 DEFERRED_BY_HUMAN_RULING).

Staleness findings (9 DECL rows):

- **DEL-01-01 (ISSUED)**: Datasheet Status cell still reads "IN_PROGRESS
  governance baseline refresh" against the ISSUED `_STATUS.md`; kit cites
  revision 0.7 / DAG-006 as current. Routed SCOPE_CHANGE (touching the
  ISSUED baseline is a change-path act); no repair proposed.
- **DEL-01-03**: Datasheet still carries "License decision: TBD" against the
  frozen selected PolyForm-Noncommercial-1.0.0 state — uncured drift the
  REV-001 fix missed; plus rev-pointer drift rows.
- **DEL-01-02/DEL-01-04**: rev-0.7/DAG-006 pointer drift on
  Datasheet/Procedure surfaces.

REMAINING_WORK: DEL-01-02 REM-001 (certification-mechanism deferral;
ruling-permitted, homing verified) and DEL-01-03 REM-001 (D-07b-gated
public-intake mechanism) are the two ACCEPTED_DIVERGENCE residuals; two
ALIGNED residual rows record accurate decision-less residuals.

## Decision findings (routing, not rulings)

- SCOPE_CHANGE: DEL-01-01 DECL-001/002/004 (any refresh of the ISSUED kit).
- OWNER: DEL-01-02 DECL-002 + REM-001 context; DEL-01-03 REM-001 companion
  row; DEL-01-04 ACC-001 (wording-approval act still open).
- Named gate: D-07b (AWAITING_RULING at the frozen register) on DEL-01-03's
  intake residual.
- No ENGINEERING routing in this package.

## Unmapped implementation

None ledgered. Notes-level observations for R3: `governance/MAINTAINERS.md`
and `docs/README.md` carry mixed 0.7/DAG-007 pointers with no convention
home; R1 IMPLEMENTATION_SURFACES.csv lacks rows for two mapped policy docs
(IP_AND_DATA_BOUNDARY.md, PROFESSIONAL_BOUNDARY.md).

## Conflicts and unknowns

None (zero AUTHORITY_CONFLICT / UNKNOWN rows).

## Verification and repair record

All four ledgers SOUND at fan-in; no re-runs. QUALIFIED items (disclosed,
aggregation-safe): acceptance-row grain, exclusion split judgments, MEMORY
census judgments, bootstrap-metadata cell annotation variant (DEL-01-02),
AuthorityNeeded variance on identical pointer drift.

## Cross-ledger risks carried forward (W2–W5 / R3)

1. Bootstrap-item gate/source cell annotation variant (annotated suffix vs
   NONE_RECORDED) — mechanical gate-suffix sweeps must strip the annotated
   D-41 value; candidate one-line run-level clarification before W2.
2. AuthorityNeeded variance (SCOPE_CHANGE/OWNER/NO) on same-species pointer
   drift — dedupe at R3; the SCOPE_CHANGE routing is ISSUED-specific.
3. Addendum-4-vs-ISSUED "perpetual staleness" tension flagged by the
   DEL-01-01 pilot — candidate owner-level clarification (an ISSUED kit can
   only be refreshed through the change path, so its drift rows persist by
   design).
4. ACCEPTANCE row grain varies by pilot judgment (see PKG-00 risk 3).

## Fences

Discovery read-only; frozen tree clean throughout; DEL-01-01 baseline
untouched; no F-PIP-1..5 claim language outside attributed quotes; all
dispositions are agent judgments.
