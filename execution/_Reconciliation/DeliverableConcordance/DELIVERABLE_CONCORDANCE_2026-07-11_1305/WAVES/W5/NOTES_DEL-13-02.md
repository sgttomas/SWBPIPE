# NOTES — DEL-13-02 Constraint entity and provenance model (R2 W5)

DEL-13-02 / PKG-13, IN_PROGRESS; frozen SHA
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Role/model: **GPT-5
deliverable-grained discovery pilot**. Ledger: 30 rows, 20 columns, RFC-4180
CRLF after normalization.

## Census and histograms

ClaimType: REQUIREMENT 10; ACCEPTANCE 8; EXCLUSION 5; DECLARED_STATE 6;
REMAINING_WORK 1. Disposition: ALIGNED 28; STALE_SETUP_SPECIFICATION 1;
REMAINING_STATE_MISMATCH 1. AuthorityNeeded: NO 26 / OWNER 4.
Selectability: YES 0 / NO 30.

The eight acceptance rows preserve the eight distinct entries in
Specification `## Verification`, including grouped requirement IDs. The
declaration census is exactly the four-document kit plus `_STATUS.md` and
`MEMORY.md`; no deliverable-owned README exists. The D-41 bootstrap is quoted
only in DECL-005 `RecordedRemaining` and is excluded from residual and
selectability analysis.

## Key judgments and calibration

- The ten requirement rows are ALIGNED at data-model-contract grain. This does
  not imply runtime validation, GUI reachability/blocking, physical-to-
  analytical consumption, public-example publication, professional reliance,
  or code compliance.
- REQ-005 and ACC-004 use the focused unit-schema evidence but remain MEDIUM
  and OWNER-routed because finding PKG13-DEL-13-02-PKG02-001 is
  `TECHNICALLY_ADDRESSED_PENDING_HUMAN`. The substance disposition rests on
  the current exact-vocabulary test; it does not perform the human disposition.
- REM-001 records that pending human disposition because it is absent from the
  status remainder and no alternate named home was found. It is a remaining-
  state mismatch, not a technical failure.
- Procedure DECL-004 alone is STALE: it calls decomposition revision 0.7 the
  current accepted basis, while the run's R1 inventory and W1–W4 calibration
  establish the corpus-wide 0.7-to-0.8 pointer drift. Technical procedure steps
  survive. MEMORY's dated June 4 revision-0.7 record remains historical and is
  not treated as stale current prose.
- SECURITY rows do not use the convention-6 marker. They are bounded schema-
  content/exclusion checks and do not contain an accepted sufficiency deferral.
  Their validation cells explicitly disclaim legal/security sufficiency.
- No ACCEPTED_DIVERGENCE is encoded: no named human ruling permits a deferred
  completed-state divergence. No implementation-unmapped surface was found.

## Verification and containment

Re-executed in the frozen evidence tree with
`PYTHONDONTWRITEBYTECODE=1`:

- `python3 -m json.tool projects/chirality-piping/schemas/constraint.schema.json`
  — PASS.
- `python3 projects/chirality-piping/tests/test_constraint_schema.py` — PASS.

No pytest, cargo, generator, compilation, or `py_compile` was invoked. The
pytest `-p no:cacheprovider` and copy-out lockless-cargo controls therefore did
not apply.

Frozen `git status --short --ignored=matching` before and after showed exactly
the same six allow-listed pre-existing ignored sets:

1. `projects/chirality-piping/.pytest_cache/`
2. `projects/chirality-piping/core/reporting/report_generator/Cargo.lock`
3. `projects/chirality-piping/core/reporting/result_export/Cargo.lock`
4. `projects/chirality-piping/core/reporting/state_comparison_handoff_sections/__pycache__/`
5. `projects/chirality-piping/tests/__pycache__/`
6. `projects/chirality-piping/validation/benchmarks/nonlinear/target/`

Tracked porcelain was empty before and after. The frozen tree was not cleaned
or modified. Writes were limited to this CSV and notes in the dedicated D-41
worktree. No product, status, lifecycle, DAG, register, decision, dependency,
review finding, R4, or R5 surface changed. No release, certification, sealing,
approval, authentication, professional-reliance, code-compliance, legal-
sufficiency, or security-sufficiency assertion is made. Dispositions are agent
judgments, never human rulings. No material contradiction was found.
