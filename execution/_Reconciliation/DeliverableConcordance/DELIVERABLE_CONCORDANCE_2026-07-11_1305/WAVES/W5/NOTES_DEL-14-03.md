# NOTES — DEL-14-03 Model-state comparison engine (R2 wave W5)

Frozen source tree: `main` @ `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
Deliverable: PKG-14 / DEL-14-03, status `IN_PROGRESS`.
Pilot attribution: delegated GPT-5 bounded owning deliverable pilot; evidence-led concordance judgment at full available task capability.

## Scope and census

The ledger contains **30 rows**: REQUIREMENT 10, ACCEPTANCE 7, EXCLUSION 5,
DECLARED_STATE 6, and REMAINING_WORK 2. The declared-state census includes
exactly the four-document kit plus `_STATUS.md` and `MEMORY.md`; the owned
implementation directory has no README, so no additional README declaration
row is due. Unqualified deliverable-document names in the ledger resolve to
`execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-03_Model-state comparison engine/`
at the frozen SHA.

Disposition histogram: ALIGNED 22, PARTIALLY_IMPLEMENTED 2,
STALE_SETUP_SPECIFICATION 4, REMAINING_STATE_MISMATCH 2. Confidence: HIGH 26,
MEDIUM 4, LOW 0. Authority routing: NO 17, OWNER 11, ENGINEERING 2. Every row is
mechanically `SelectableUnderCurrentLoop=NO`; `_STATUS.md` has no
non-bootstrap residual.

## Substantive reading and self-flags

- `REQ-007` and `ACC-005` are PARTIALLY_IMPLEMENTED at exact unit-behavior
  grain. The engine blocks missing metadata, incompatible dimensions, and
  differing units only for caller-declared `unit_bearing_fields`; it does not
  discover unit-bearing fields or apply the governed DEL-02-02/DEL-14-05
  normalization contract. The focused test directly exercises missing
  metadata, while the other two branches are source-inspected but not directly
  exercised in `test_model_state_comparison.py`. These rows remain
  ENGINEERING-routed and do not invent a tolerance or normalization policy.
- `DECL-001`, `DECL-002`, `DECL-003`, and `DECL-004` are stale at
  document-surface grain.
  Specification and Datasheet explicitly present revision 0.7 as current
  authority while the frozen decomposition is revision 0.8, and Specification,
  Datasheet, and Procedure retain setup-era TBD artifact pointers that the
  frozen engine, focused tests, MEMORY, and run records have overtaken.
  Guidance's substantive principles remain sound, but its conflict table still
  presents pre-v3.1 enum preservation as an unresolved human-ruling question
  after the local register was normalized to canonical v3.1 with legacy meaning
  preserved in Notes; staleness is limited to that declaration.
- `REM-001` and `REM-002` preserve the two OPEN agent findings from
  `Review_Findings.csv`. Both have `HumanDisposition=TBD` and are absent from
  the sole current work-discovery surface, so they are evidence-backed
  REMAINING_STATE_MISMATCH rows with MEDIUM+OWNER under addendum 13. They are
  findings for later homing/adjudication, not rulings or authorization to edit
  dependencies or documents.
- Fan-in should also inspect `REQ-003`/`ACC-004` (bounded explicit-mapping
  contract versus DEL-14-05 policy ownership), `REQ-005`/`ACC-006`
  (determinism at invented-fixture grain), and the negative-boundary rows
  `REQ-009`, `REQ-010`, `ACC-007`, `EXC-004`, and `EXC-005`. Their ALIGNED
  judgments do not imply external validation, professional approval, legal
  clearance, security assurance, or release readiness.

## Evidence and reliability

Under addendum-9 controls at the frozen SHA:

- `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest -q -p no:cacheprovider projects/chirality-piping/tests/test_model_state_schema.py projects/chirality-piping/tests/test_comparison_contracts.py projects/chirality-piping/tests/test_model_state_comparison.py` returned **13 passed**.
- `PYTHONDONTWRITEBYTECODE=1 python3 projects/chirality-piping/tools/validation/validate_dependencies_schema.py <DEL-14-03>/Dependencies.csv` returned **VALID**, 29 columns and 15 data rows.

All load-bearing technical evidence is project-original and agent-generated;
the open review findings retain TBD human dispositions. Accordingly every
technical row is `UNVERIFIED`, while the six declaration-prose rows are
`NOT_APPLICABLE`. No technical row is promoted to REVIEWED by the historical
review recommendation or lifecycle entry.

## Addendum-9 containment and fences

Ignored-aware porcelain before and after verification contained exactly the
six allow-listed incident paths and no seventh path: project `.pytest_cache/`,
the two reporting `Cargo.lock` files, the two reporting/tests `__pycache__/`
trees, and `validation/benchmarks/nonlinear/target/`. Pytest used
`-p no:cacheprovider` with bytecode disabled; no cargo or in-tree `py_compile`
ran. The seeded D-41 item is copied byte-exactly only on `DECL-005` and is
excluded from residual, gate, and selectability analysis.

No frozen source, deliverable, product, lifecycle, DAG, register, dependency,
R4, or R5 surface was changed. All dispositions and routes are agent judgments,
never owner, engineering, professional, compliance, or release rulings.

## Cross-ledger risks for PKG-14 fan-in and R3

1. DEL-14-03 consumes DEL-14-01 and DEL-14-05 surfaces without assuming their
   schema/policy ownership; shared schemas and helpers must not be mistaken for
   duplicate authoritative ownership.
2. Object/record metadata preservation does not establish field-level
   traceability, unit-field discovery, tolerance policy, or normalized numeric
   comparison. Join this carefully with PKG-13 traceability findings and later
   handoff/export ledgers.
3. The OPEN RF-001 dependency finding is a dependency-closure/status-home issue,
   not evidence that the bounded comparison engine is absent or failed.
4. Exact mapping consumption is implemented, but heuristic equivalence and
   mapping-policy choices remain outside this deliverable; avoid collapsing
   this distinction into a generic “comparison complete” species.
5. Setup-era artifact TBDs and revision-pointer drift belong to declaration
   staleness; they do not automatically stale independently verified
   requirement substance.
