# Package Concordance Summary — PKG-04 (Solver Core and Numerical Methods)

> **Epistemic status: agent-authored, non-operative evidence** (R2 wave W2,
> run DELIVERABLE_CONCORDANCE_2026-07-11_1305). Computed from the six W2
> claim ledgers after the fan-in verification pass and owning-pilot string
> corrections. No disposition here is an owner or engineering ruling;
> nothing edits any deliverable. Frozen source state:
> `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.

## Census

6 deliverables (DEL-04-01 3D frame stiffness kernel, DEL-04-02 straight
pipe element, DEL-04-03 linear support and restraint models, DEL-04-04
nonlinear support active-set solver, DEL-04-05 sparse solver performance
harness, DEL-04-06 solver diagnostics and singularity detection), all
`IN_PROGRESS`; 133 claim rows; ledgers `WAVES/W2/CLAIM_CONCORDANCE_DEL-04-0*.csv`.
All pilots fable per the Receipt-17 steer (solver mechanics —
engineering-adjudication risk). Verification:
`WAVES/W2/W2_VERIFICATION_PKG-04.md` — **6/6 SOUND** (55 PASS /
13 QUALIFIED / 0 FAIL); no re-runs; two owning-pilot string corrections
applied post-fan-in (convention-7 ATTESTED markers on DEL-04-03-REQ-004
and DEL-04-05-REQ-006 for a MEMORY-transcribed record path absent from the
frozen tree). All histograms reproduce; static test counts independently
recounted by the verifier; addendum-10 diffs independently re-run.

## ClaimType × Disposition (computed from ledgers)

| ClaimType | total | ALIGNED | PARTIALLY_IMPLEMENTED | STALE_SETUP_SPECIFICATION | ACCEPTED_DIVERGENCE |
|---|---|---|---|---|---|
| REQUIREMENT | 58 | 53 | 4 | 0 | 1 |
| ACCEPTANCE | 1 | 1 | 0 | 0 | 0 |
| EXCLUSION | 24 | 23 | 0 | 0 | 1 |
| DECLARED_STATE | 41 | 18 | 0 | 23 | 0 |
| REMAINING_WORK | 9 | 9 | 0 | 0 | 0 |

`SelectableUnderCurrentLoop=YES`: 7 rows (DEL-04-01 DECL-005/REM-001,
DEL-04-03 DECL-005/REM-001, DEL-04-04 DECL-005/REM-002/REM-003) —
mechanical derivation only; the owner suspension remains a run-level
caveat. No IMPLEMENTED_UNMAPPED, UNKNOWN, or AUTHORITY_CONFLICT rows.

## Package reading

PKG-04 is a mature solver-mechanics package: requirement substance is
overwhelmingly ALIGNED (53/58) with crate test suites re-executed or
re-counted at the frozen SHA (frame kernel, straight pipe, restraints,
active-set solver, performance harness, diagnostics). The staleness burden
again sits on declared-state surfaces: the corpus-wide SOFTWARE_DECOMP
rev-0.7→0.8 pointer drift (verified in all six frozen Datasheets) encoded
STALE-side per the W1 adjudication with the owner-calibration caveat once
per notes, plus kit-local overtaken setup prose (e.g. DEL-04-05's Procedure
declaring a false "current setup-only boundary" with no refresh — STALE
upheld by the verifier against DEL-04-04's refreshed-Procedure ALIGNED).

Substantive non-staleness findings:

- 4 PARTIALLY_IMPLEMENTED REQUIREMENT rows (partial coverage or missing
  named checks at requirement grain; none disputed by the verifier).
- 2 ACCEPTED_DIVERGENCE rows resting on named permitting records
  (addendum-11 verified).
- Verification ≠ validation held package-wide: no unit-test promotion to
  engineering validation; threshold/tolerance matters route ENGINEERING
  (4 rows).

## Decision findings (routing, not rulings)

AuthorityNeeded: NO 104, OWNER 17, REVIEW 8, ENGINEERING 4. OWNER rows are
dominated by rev-drift DECL surfaces co-carrying overtaken TBD registers
and bootstrap _STATUS surfaces; ENGINEERING rows are threshold/tolerance
adjacent per the solver-mechanics strictness.

## Verification and repair record

Fan-in (fable, high effort): all six ledgers SOUND; 0 FAIL spot-checks.
Adjudications of record: (1) the DEL-04-04 (ALIGNED) vs DEL-04-05 (STALE)
Procedure-surface variance is fact-based, both correct; (2) DEL-04-01's
MEMORY ALIGNED-with-note is convention-permitted (in-file correction); the
package treats undated MEMORY heads three ways — rule needed before W3;
(3) DEL-04-03 bootstrap cell scoping is aggregation-safe QUALIFIED —
recommend R3 standardize on the exclusion variant (uniform in W2);
(4) byte-identical out-of-tree copy re-execution (DEL-04-01/02/03) —
addendum-9 sound in all three uses; proposed R3 rule: independent
byte-identity verification + external target dir + porcelain check +
in-row disclosure. String corrections (non-defect): DEL-04-03-REQ-004 and
DEL-04-05-REQ-006 ATTESTED markers applied by owning pilots, revalidated,
histograms unaffected.

## Cross-ledger risks carried forward (W3–W5 calibration / R3)

1. Crate-README census grain: DEL-04-02 excluded its self-identifying
   README (6 vs 7 DECL rows); W1 precedent exists both ways — R3 rule.
2. Undated-MEMORY-head treatment split (04-01/02 ALIGNED-with-note vs
   04-06 STALE) — needs a run-level "corrected-in-file" distinguisher
   before W3 (see also PKG-05 item).
3. SourceReliability ladder variance (04-01 REVIEWED on REQ rows vs
   siblings UNVERIFIED — both defensible under addendum 6): histograms not
   comparable at face value.
4. ACCEPTED_DIVERGENCE species for overtaken setup-pass exclusions
   (04-05 EXC-001 vs 04-01 EXC-001 ALIGNED) — dedupe at R3.
5. Addendum-13 cap/routing three-way variance (OWNER/REVIEW, MEDIUM/HIGH).
6. `AuthorityNeeded` semantics on ALIGNED REM rows (ENGINEERING read as
   "perform the work", not "adjudicate a claim") — R3 must not count these
   as open adjudications.
7. Byte-identical copy re-execution pattern — candidate convention
   addendum (see verification record).

## Fences

Discovery read-only; frozen tree porcelain empty before and after all
pilot, verifier, and correction operations; no lifecycle/DAG/scope change
proposed as operative; no F-PIP-1..5 claim language outside attributed
quotes; all dispositions are agent judgments routed via AuthorityNeeded.
