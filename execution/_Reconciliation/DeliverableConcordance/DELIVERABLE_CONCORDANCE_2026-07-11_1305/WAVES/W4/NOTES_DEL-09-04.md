# NOTES — DEL-09-04 Validation manual skeleton (R2 wave W4)

Deliverable **DEL-09-04**, PKG-09, lifecycle IN_PROGRESS. Frozen source:
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Role/model: **GPT-5 discovery
pilot**; no model substitution. Ledger has 26 rows, 20 columns, RFC-4180 CRLF.
Kit-path aliases are relative to
`execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/`;
other paths are repository-root relative.

## Histograms

ClaimType: REQUIREMENT 10; ACCEPTANCE 5; EXCLUSION 2; DECLARED_STATE 6;
REMAINING_WORK 3. Disposition: ALIGNED 21; STALE_SETUP_SPECIFICATION 3;
ACCEPTED_DIVERGENCE 2. AuthorityNeeded: NO 22; OWNER 4. Selectability:
YES 2 / NO 24 (DECL-005 and ungated REM-001).

## Census and adjudication

- Ten native `VAL-REQ-*` requirements become contiguous REQ-001..010. The
  frozen `docs/validation_manual/index.md`, 63 generated case pages,
  reproduction page, and `docs/VALIDATION_STRATEGY.md` substantively address
  all ten at documentation-contract grain.
- Five ACC rows mirror the five distinct Specification verification checks.
  ACC-001 is direct-presence evidence because the named
  `check_four_documents.sh` is absent; ACC-005 cites the recorded dependency
  validator pass and clearly says it was not re-executed.
- Two durable exclusions capture the professional-authority boundary and the
  public/private/protected-source boundary. Overtaken setup-session write
  prohibitions are declaration drift, not durable exclusions.
- Addendum-1 DECL census is exactly four kit documents + `_STATUS.md` +
  `MEMORY.md`; there is no deliverable-owned README. Specification, Datasheet,
  and Procedure are STALE because they retain setup/local-only facts after the
  repository manual, case generator/pages, and strategy updates landed.
  Guidance remains factually current. `_STATUS.md` and MEMORY are ALIGNED.
- Three REM rows correspond one-for-one to the non-bootstrap status items.
  REM-001 and REM-002 use ACCEPTED_DIVERGENCE because DEC-054 explicitly
  carries the incomplete §16.5 evidence system forward and DEC-046 explicitly
  preserves unmeasured tolerance entries as TBD. REM-003 is ALIGNED as a
  correctly recorded, stage-gated external-reproduction residual.
- The D-41 bootstrap is byte-exact only in DECL-005 `RecordedRemaining`; it is
  excluded from residual, gate, and selectability analysis.
- R1 surfaces show the runner, CLI, three benchmark crates, and input generator
  mapped to DEL-09-04; no material IMPLEMENTED_UNMAPPED row was found.

## Evidence posture and calibration

No command, test, cargo operation, pytest, generator, or Python compilation was
re-executed. The ledger attributes recorded runs to
`WORKING_ITEMS_RUN_2026-07-10_TP-E2-VALMANUAL-001.md` (mechanics 30, stress 22,
nonlinear 19; generator determinism) and marks them not re-executed by W4.
Direct read-only inspection confirmed 63 committed case-page Markdown files
and the generator. Commit `03344e6...` does not resolve in the frozen object
store, so it is not used as a standalone verifiable basis; MEMORY/run records
remain attestation-level history.

W1–W3 calibration applied: weakest-load-bearing technical evidence is
UNVERIFIED; all DECL prose is NOT_APPLICABLE; current manual rev-0.8/DAG-007
correction prevents dated MEMORY rev-0.7/DAG-006 entries from becoming stale
current declarations; setup false facts are STALE only on census DECL rows;
and AuthorityNeeded routes adjudication rather than ordinary work performance.

Self-flags for fan-in: ACC-002 source-boundary review is evidence-backed but no
legal/sufficiency conclusion is claimed; REQ-010 is SECURITY-class with
NOT_APPLICABLE validation because it states a deterministic data-boundary
contract rather than deferring a security-sufficiency review; DECL-002 judges
the Datasheet's “Skeleton required” table overtaken by landed draft evidence;
REM-002's DEC-046/owner-threshold relationship warrants close review; and the
five-row acceptance census may be compared against W3's narrowed distinct-
section grain.

## Addendum-9 and fences

Before and after, frozen `git status --short --ignored=matching` showed exactly
the six allow-listed pre-existing paths: `.pytest_cache/`, the report_generator
and result_export `Cargo.lock` files, the state-comparison and tests
`__pycache__/` directories, and `validation/benchmarks/nonlinear/target/`.
No new ignored path appeared and the frozen tree was not modified or cleaned.
Because nothing was executed, copy-out cargo, pytest cache suppression, and
bytecode controls were not invoked. Writes were limited to this CSV and notes
file. No lifecycle, DAG, register, decision, deliverable, product, R4, or R5
surface was changed. Dispositions are agent judgments, never human rulings.

