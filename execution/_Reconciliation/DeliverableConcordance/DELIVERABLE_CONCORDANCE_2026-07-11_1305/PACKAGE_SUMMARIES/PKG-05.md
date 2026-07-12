# Package Concordance Summary — PKG-05 (Loads, Load Cases, and Stress Recovery)

> **Epistemic status: agent-authored, non-operative evidence** (R2 wave W2,
> run DELIVERABLE_CONCORDANCE_2026-07-11_1305). Computed from the five W2
> claim ledgers after the fan-in verification pass and one owning-pilot
> evidence-pointer amendment. No disposition here is an owner or
> engineering ruling; nothing edits any deliverable. Frozen source state:
> `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.

## Census

5 deliverables (DEL-05-01 primitive load case engine, DEL-05-02 load-case
algebra engine, DEL-05-03 fundamental stress recovery module, DEL-05-04
analysis status semantics, DEL-05-05 concentrated and distributed user
load application), all `IN_PROGRESS`; 109 claim rows; ledgers
`WAVES/W2/CLAIM_CONCORDANCE_DEL-05-0*.csv`. All pilots fable per the
Receipt-17 steer. Verification: `WAVES/W2/W2_VERIFICATION_PKG-05.md` —
**5/5 SOUND** (45 PASS / 12 QUALIFIED / 0 FAIL); no re-runs; one
owning-pilot amendment post-fan-in (DEL-05-04-REQ-014 VerificationEvidence
extended with two declaration-level const-flag asserts, explicitly
qualified as not the missing negative test; disposition unchanged).

## ClaimType × Disposition (computed from ledgers)

| ClaimType | total | ALIGNED | PARTIALLY_IMPLEMENTED | STALE_SETUP_SPECIFICATION | ACCEPTED_DIVERGENCE |
|---|---|---|---|---|---|
| REQUIREMENT | 54 | 53 | 1 | 0 | 0 |
| ACCEPTANCE | 1 | 0 | 0 | 0 | 1 |
| EXCLUSION | 17 | 17 | 0 | 0 | 0 |
| DECLARED_STATE | 34 | 18 | 0 | 16 | 0 |
| REMAINING_WORK | 3 | 3 | 0 | 0 | 0 |

`SelectableUnderCurrentLoop=YES`: 2 rows (DEL-05-01 DECL-005/REM-001) —
mechanical derivation only. No IMPLEMENTED_UNMAPPED, UNKNOWN, or
AUTHORITY_CONFLICT rows.

## Package reading

PKG-05's requirement substance is nearly uniformly ALIGNED (53/54) on
re-executed or re-counted crate/pytest evidence at the frozen SHA (load
case engines, stress recovery 26/26, user loads 28/28 on verified
byte-identical scratch copies, status-semantics suites 6 passed + a
37-assert script suite closing R1 PY-15's no-recorded-execution gap).
Solver-mechanics strictness held: validation legs that rest on
agent-generated witnesses stay UNVERIFIED on the SourceReliability ladder;
no unit-test promotion to engineering validation anywhere in the package.
Staleness again concentrates on declared-state surfaces: the corpus-wide
rev-0.7→0.8 pointer drift (8 DECL rows, each pointer verified in the
frozen kits, one owner-calibration caveat per ledger) plus crate-README
enumeration gaps (DEL-05-03/05-05 DECL-007 — READMEs omitting implemented
station-recovery and straight-pipe equivalent recovery families; verified
hard against the tree and mutually consistent) and DEL-05-01's undated
MEMORY "Boundaries Preserved" header drift (STALE upheld: nothing corrects
in-file; addendum 1 protects only dated entries).

Substantive non-staleness findings:

- `DEL-05-04-REQ-014` PARTIALLY_IMPLEMENTED: the kit's own verification
  approach names a stale-hash acceptance-reuse negative test that does not
  exist anywhere in the frozen `tests/` sweep; only declaration-level
  const-flag asserts exist (now cited in-row with the qualifier).
- `DEL-05-03-ACC-001` ACCEPTED_DIVERGENCE: residual tolerance/benchmark
  TBD scope deferred under the 2026-06-05 human blocker-closure ruling —
  addendum 11 satisfied (the package's thinnest permission record);
  ENGINEERING routing upheld over addendum-13 OWNER because the
  disposition rests on the ruling itself.

## Decision findings (routing, not rulings)

AuthorityNeeded: NO 101, OWNER 3, ENGINEERING 2, REVIEW 1, D-38 2
(gate-named tokens; W1 D-07b precedent — dedupe at R3). The corpus
tolerance/threshold TBD is routed exactly once (DEL-05-03-ACC-001,
ENGINEERING); DEL-05-05-REQ-005's AuthorityNeeded=NO on recorded
release-threshold TBDs is non-counting (no numeric claim to promote) —
both upheld as fact-based by the verifier.

## Verification and repair record

Fan-in (fable, high effort): all five ledgers SOUND; 0 FAIL spot-checks;
the verifier independently re-ran stress_recovery (26 passed) and the
DEL-05-04 pytest set (6 passed), re-ran two addendum-10 diffs (both
empty), and programmatically verified all five bootstrap DECL-005
RecordedRemaining cells byte-equal to the frozen `_STATUS.md`.
Adjudications of record: DEL-05-01 MEMORY STALE correct on its facts and
consistent with DEL-04-01's ALIGNED (which rested on an in-file
correction); threshold-TBD routing variance both-defensible with an R3
dedupe instruction; README enumerated-declaration STALE grain
convention-correct and consistent across 05-01/03/05. No defects; one
evidence-pointer amendment applied by the owning pilot, revalidated,
histograms unaffected.

## Cross-ledger risks carried forward (W3–W5 calibration / R3)

1. **SourceReliability leg-keying variance (top risk)**: DEL-05-01 keys
   REVIEWED to human Gate-5-approved crate evidence on rows whose
   validation witnesses are still pending, while DEL-05-05 keys UNVERIFIED
   to the weakest load-bearing (validation) leg. R3 must not read REVIEWED
   on MECHANICS rows as human-validated mechanics; recommend a "weakest
   load-bearing leg" rule for W3+.
2. AuthorityNeeded declared-TBD semantics, incl. gate-named tokens (D-38
   here, D-07b in W1) — normalize at R3.
3. Undated-MEMORY-head treatment: codify the "corrected-in-file"
   distinguisher before W3 (joint with PKG-04 item 2).
4. README enumeration-grain rule — candidate convention addendum (joint
   with PKG-04 item 1).
5. DEL-05-02 DECL-003 overtaken dependency-maturity prose analog —
   aggregation-safe; dedupe with the Gate-C-prose family at R3.

## Fences

Discovery read-only; frozen tree porcelain empty before and after all
pilot, verifier, and amendment operations; no lifecycle/DAG/scope change
proposed as operative; no F-PIP-1..5 claim language outside attributed
quotes; all dispositions are agent judgments routed via AuthorityNeeded.
