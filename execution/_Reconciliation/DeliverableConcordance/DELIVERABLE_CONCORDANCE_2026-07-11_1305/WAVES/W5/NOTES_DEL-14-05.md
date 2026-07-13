# NOTES — DEL-14-05 Comparison mapping, tolerance, and export contracts (R2 wave W5)

Frozen source tree: `main` @ `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
Deliverable: PKG-14 / DEL-14-05, status `IN_PROGRESS`.
Pilot attribution: delegated GPT-5 bounded owning deliverable pilot; evidence-led concordance judgment at full available task capability.

## Scope and census

The ledger contains **33 rows**: REQUIREMENT 11, ACCEPTANCE 11, EXCLUSION 5,
and DECLARED_STATE 6. There are no REMAINING_WORK or IMPLEMENTED_UNMAPPED
rows. The declared-state census is exactly the four-document kit plus
`_STATUS.md` and `MEMORY.md`; neither mapped schema surface has a
deliverable-owned README.

Unqualified deliverable-document names resolve to
`execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-05_Comparison mapping, tolerance, and export contracts/`
at the frozen SHA. Dispositions: ALIGNED 25, PARTIALLY_IMPLEMENTED 4,
STALE_SETUP_SPECIFICATION 4. Confidence: HIGH 29, MEDIUM 4, LOW 0. Authority:
NO 17, OWNER 14, ENGINEERING 2. All 33 rows are mechanically
`SelectableUnderCurrentLoop=NO` because `_STATUS.md` is bootstrap-only.

## Substantive reading and self-flags

- `REQ-008` is PARTIALLY_IMPLEMENTED at report-section contract grain. The
  schema preserves global diagnostics/assumptions/hashes/provenance/boundary
  context and reserves typed report-section roles, but does not explicitly
  represent analysis statuses or limitations on the section reference and
  fixes rendering to `reserved_reference_only_not_implemented`.
- `ACC-001` is partial because schema stability fields and ordering exist but
  equivalent-input state/run determinism belongs to DEL-14-03/04 engines and
  has no DEL-14-05-owned repeatability execution.
- `ACC-007` is partial because the CSV contract defines format, controlled
  columns, stable ordering, and redaction, but no materialized public-safe CSV
  fixture or parse/round-trip test exists.
- `ACC-008` is partial because report preservation flags exist without a
  rendered section fixture, explicit limitation/status fields, or downstream
  integration.
- `DECL-001..004` are stale at document-surface grain. The setup-era kit still
  describes materialized mapping/unmatched enums, schema bodies, JSON/CSV
  fields, and focused verification as TBD, and presents revision-0.7/DAG-006
  authority as current. Genuine governed tolerance values, profile approval,
  source-safe examples, runtime engines, and final rendering remain valid
  deferrals and are not reclassified as implemented.
- Fan-in should scrutinize `REQ-004`/`ACC-004` (no-default tolerance safety is
  not engineering suitability), `REQ-007`/`ACC-010` and `EXC-005` (privacy/IP
  boundary without assurance or legal clearance), `REQ-009`/`ACC-009` and
  `EXC-004` (negative professional-boundary controls), and `REQ-011`/`ACC-011`
  (contract compatibility and mirror validity do not establish dependency
  closure).

## Evidence and reliability

Under addendum-9 controls at the frozen SHA:

- `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest -q -p no:cacheprovider projects/chirality-piping/tests/test_comparison_contracts.py` returned **3 passed**.
- Both schemas passed `python3 -m json.tool`.
- `PYTHONDONTWRITEBYTECODE=1 python3 projects/chirality-piping/tools/validation/validate_dependencies_schema.py <DEL-14-05>/Dependencies.csv` returned **VALID**, 29 columns and 13 data rows.

The focused test statically verifies schema structure, controlled values,
no-default guards, and forbidden affirmative scope text. It does not execute a
comparison engine, serialize a runtime export, validate engineering tolerance
values, or render a report. All technical evidence is project-original and
agent-generated, so the 27 substantive rows are `UNVERIFIED`; the six
declaration rows are `NOT_APPLICABLE`. The agent-only PKG-02 compatibility
review does not elevate SourceReliability to REVIEWED.

## Remaining-work and containment discipline

`Review_Findings.csv` contains only its header. The kit's genuine TBDs are
explicit bounded contract exclusions/deferrals or values requiring external
governance; none has an evidence-backed current work-home finding warranting a
separate REM row. The seeded D-41 item is copied byte-exactly only on
`DECL-005` and excluded from residual, gate, and selectability analysis.

Ignored-aware porcelain before and after verification contained exactly the
six allow-listed incident paths and no seventh path: project `.pytest_cache/`,
the two reporting `Cargo.lock` files, the reporting/tests `__pycache__/` paths,
and `validation/benchmarks/nonlinear/target/`. Pytest used
`-p no:cacheprovider` with bytecode disabled; no cargo or in-tree `py_compile`
ran.

No DEL-14-03 artifact, frozen source, deliverable, product, lifecycle, DAG,
register, dependency, R4, or R5 surface was changed. All dispositions/routes
are agent judgments, never owner, engineering, professional, compliance,
security, validation, or release rulings.

## Cross-ledger risks for PKG-14 fan-in and R3

1. DEL-14-05 owns mapping/tolerance/export contract vocabulary, while
   DEL-14-03/04 own engine behavior and DEL-08-04/reporting surfaces own runtime
   export/rendering. Shared schema consumption is not duplicate authority.
2. A no-default numeric guard proves explicit governance routing, not the
   suitability of any supplied tolerance value or profile.
3. Schema-required units and normalization-basis vocabulary do not establish
   runtime conversion correctness; join with DEL-14-03/04 unit findings at
   exact behavior grain.
4. CSV/JSON contract shape must not be aggregated as a completed runtime
   exporter; report-section references explicitly remain unrendered.
5. Four dependency satisfaction cells remain TBD even though the mirror is
   schema-valid and ACTIVE. Do not flatten register validity into closure.
6. Setup-era doc staleness should be deduplicated from genuine governed-value
   and rendering deferrals rather than treating every `TBD` token alike.
