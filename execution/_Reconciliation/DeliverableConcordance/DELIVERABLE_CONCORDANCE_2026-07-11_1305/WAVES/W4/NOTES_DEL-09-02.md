# NOTES — DEL-09-02 Stress recovery benchmark suite (R2 wave W4)

Deliverable: **DEL-09-02** (PKG-09 Verification, Validation, and Quality Oracles),
status IN_PROGRESS. Frozen source tree pinned SHA
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Ledger:
`CLAIM_CONCORDANCE_DEL-09-02.csv` (18 rows, 20 columns, RFC-4180 CRLF).

Discovery role/model attribution: **GPT-5 discovery pilot**, as assigned at dispatch. No
model substitution occurred.

Run-level `NormativeSource` aliases (addendum 12): kit filenames are relative to
`execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-02_Stress recovery benchmark suite/`;
`SOFTWARE_DECOMP.md` means `execution/_Decomposition/SOFTWARE_DECOMP.md`; implementation
paths are relative to `projects/chirality-piping/` in the frozen tree.

## 1. Histograms (recount reproduces from the CSV)

**ClaimType (n=18):**

| ClaimType | Count |
|---|---:|
| REQUIREMENT | 8 |
| ACCEPTANCE | 0 |
| EXCLUSION | 2 |
| DECLARED_STATE | 7 |
| REMAINING_WORK | 1 |

**Disposition (n=18):**

| Disposition | Count |
|---|---:|
| ALIGNED | 10 |
| STALE_SETUP_SPECIFICATION | 5 |
| IMPLEMENTED_DIFFERENTLY | 1 |
| PARTIALLY_IMPLEMENTED | 1 |
| REMAINING_STATE_MISMATCH | 1 |

All eight requirement rows carry substance dispositions. `STALE_SETUP_SPECIFICATION` is
used only on census `DECLARED_STATE` rows. `SelectableUnderCurrentLoop=NO` on every row:
the only recorded `_STATUS.md` item is the D-41 bootstrap, which addendum 2 excludes, and
the omitted residual row has no recorded selectable work item.

## 2. Census reasoning

- **REQUIREMENT: 8.** These correspond one-for-one to Specification native IDs
  `DEL-09-02-RQ-001..008`; fixed run ClaimIDs use `REQ-001..008`.
- **ACCEPTANCE: 0.** The Specification and Procedure `Verification` tables restate the
  eight requirement/setup expectations. They are not a distinct acceptance section, so
  no mirrored acceptance rows were minted (addendum 12; PKG-06/08 carried calibration).
- **EXCLUSION: 2.** EXC-001 preserves the time-bounded setup-session no-implementation
  exclusion at its historical run grain. EXC-002 groups the durable protected-content,
  code-rule, fatigue, compliance and professional-authority boundaries. Later authorized
  implementation is declaration drift, not a retroactive defect in EXC-001.
- **DECLARED_STATE: 7.** Four kit documents + `_STATUS.md` + `MEMORY.md` + the owned
  `validation/benchmarks/stress/README.md`. Specification, Datasheet, Guidance and
  Procedure are stale because the frozen tree contains the implemented suite. The README
  is also stale only on its blanket tolerance-policy declaration: DEC-026 was accepted
  five days after the dated readiness tranche and distinguishes an accepted analytic-class
  seed/tier structure from still-unmeasured per-kind values. `_STATUS.md` is current.
  Every MEMORY section is dated, so its truthful historical snapshots remain ALIGNED
  (addendum 1).
- **REMAINING_WORK: 1.** The owned README and dated readiness record explicitly preserve a
  combined set of technical/readiness TBDs, while `_STATUS.md` has no non-bootstrap item
  and names no alternate home. REM-001 records that mismatch without making any item
  operative or proposing a lifecycle transition.
- **IMPLEMENTED_UNMAPPED: 0.** The benchmark crate, hand calculations and formal witness
  are explicitly mapped to DEL-09-02 by the deliverable MEMORY/run records and README.
  No material orphan surface was found at the bounded named-slice grain.

## 3. Self-flagged rows for package fan-in

- **REQ-002 / REQ-003 (SECURITY):** both are ALIGNED only at the recorded
  project-original provenance/boundary grain. Human source eligibility and protected-content
  sufficiency are not evidenced by a named disposition. The convention-6 marker is therefore
  applied only to `ValidationEvidence`, with MEDIUM confidence and OWNER routing; it is not
  blanket-applied to non-sufficiency SECURITY facts.
- **REQ-005 (units):** ALIGNED rests on explicit fixture-local units, dimensioned original
  hand calculations and the independent formal witness. The PKG-02 finding remains
  `TECHNICALLY_ADDRESSED_PENDING_HUMAN`; addendum 13 therefore caps confidence at MEDIUM
  and routes OWNER despite the independent technical grounds.
- **REQ-007 (tolerances):** chose IMPLEMENTED_DIFFERENTLY rather than ALIGNED. The frozen
  implementation still says blanket `final_tolerance_policy=TBD`, but DEC-026 accepted the
  class-tiered policy and analytic 1.0e-9 relative seed while allowing unmeasured per-kind
  relative+absolute pairs to remain TBD. This is not an authority conflict; it is a bounded
  implementation/declaration lag against a resolved decision.
- **REQ-008 (result envelope):** chose PARTIALLY_IMPLEMENTED. The suite demonstrably calls
  the governed stress-recovery API and preserves `HumanReviewRequired`, but its own readiness
  boundary says result-envelope/export integration is TBD. The row does not treat direct
  crate tests as proof of the missing integration.
- **DECL-007 (owned README):** widened addendum 4 covers post-alignment drift, not only
  setup prose. Only the blanket tolerance declaration is stale; inventory/unit/provenance
  statements are current.
- **REM-001:** the combined readiness set may be split/homed elsewhere by a later owner act.
  The ledger records the mismatch and smallest next action only; it does not perform R4/R5.

## 4. Evidence-execution and addendum-9 log

Before discovery, frozen HEAD was exactly `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
`git status --porcelain=v1 --ignored=matching` showed exactly the six disclosed sets:

1. `projects/chirality-piping/.pytest_cache/`
2. `projects/chirality-piping/core/reporting/report_generator/Cargo.lock`
3. `projects/chirality-piping/core/reporting/result_export/Cargo.lock`
4. `projects/chirality-piping/core/reporting/state_comparison_handoff_sections/__pycache__/`
5. `projects/chirality-piping/tests/__pycache__/`
6. `projects/chirality-piping/validation/benchmarks/nonlinear/target/`

The stress benchmark crate is lock-bearing (`validation/benchmarks/stress/Cargo.lock` is
tracked), so no copy-out was required. Re-executed with all build output external:

- `CARGO_TARGET_DIR=/tmp/chirality-d41-del-09-02-target CARGO_NET_OFFLINE=true cargo test
  --locked --manifest-path projects/chirality-piping/validation/benchmarks/stress/Cargo.toml`
  → **22/22 unit tests PASS**, 0 doctests.
- `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest -q -p no:cacheprovider
  projects/chirality-piping/tests/test_calculation_witness.py` → **9/9 PASS**.

No `py_compile` was run. The after-check with `--ignored=matching` reproduced exactly the
same six allow-listed sets and no seventh path; tracked porcelain remained empty. The
frozen tree was neither cleaned nor modified. The 2026-06-06 readiness run's recorded
19-test result remains truthful for its source state; three later physical-model P3 tests
bring the frozen suite to 22.

## 5. Convention/calibration carry-forward applied

- Rev-0.7/current-basis drift is encoded STALE-side only where it occurs on a census
  surface (Datasheet DECL-002); `_CONTEXT`, `_REFERENCES` and the dated MEMORY authority
  entry are notes, not additional DECL rows or stale dispositions.
- Setup/future prose is STALE only on declaration rows; REQ/EXC substance remains separate.
- SourceReliability uses weakest load-bearing leg: all technical rows are UNVERIFIED
  (project-original agent-generated code/evidence with agent review); declaration prose is
  NOT_APPLICABLE. No technical row is inflated to REVIEWED by the readiness SELF_CHECK.
- The bootstrap text appears only on `_STATUS` DECL-005 and is excluded from gate,
  residual and selectability analysis. No REM row is created for it.
- Verification and validation remain distinct. The 22 Rust tests prove executable behavior;
  mechanics rows additionally cite original hand calculations/formal witness or explicitly
  state the remaining validation-policy gap.
- AuthorityNeeded routes adjudication, not who performs future work. OWNER is used for
  SECURITY sufficiency/pending-human disposition/stale governed declarations and the omitted
  residual-home decision; ENGINEERING is used for implementing the already-ruled tolerance
  and result-envelope technical deltas.

## 6. Fences and findings

No material authority contradiction was found. The frozen register/decomposition time split
for D-41 is run mechanics, not a conflict. The missing `DEV-001_DISPATCH_DEL-09-02.md` is
marked `ATTESTED: MEMORY.md ..., record not present in tree` on EXC-001 and is not used to
manufacture acceptance. No lifecycle, DAG, dependency, product, document-kit or review
disposition file was changed. No release-readiness, certification, sealing, professional-
approval or code-compliance conclusion is made; such terms appear only as attributed
deliverable exclusions/boundaries. All dispositions are this discovery pilot's judgments,
not human rulings.
