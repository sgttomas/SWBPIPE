# NOTES — DEL-15-03 Downstream modeling export workflow (R2 wave W5)

Frozen source tree: `main` @ `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
Deliverable: PKG-15 / DEL-15-03, status `IN_PROGRESS`.
Pilot attribution: delegated GPT-5 bounded owning deliverable pilot; evidence-led concordance judgment at full available task capability.

## Scope and census

The ledger contains **30 rows**: REQUIREMENT 10, ACCEPTANCE 6, EXCLUSION 5,
DECLARED_STATE 6, and REMAINING_WORK 3. No IMPLEMENTED_UNMAPPED row is due.
The declaration census is exactly the four-document kit plus `_STATUS.md` and
`MEMORY.md`; `core/handoff/exporter/` contains no owned README.

Unqualified deliverable-document names resolve to
`execution/PKG-15_Handoff and External Prover Workflow/1_Working/DEL-15-03_Downstream modeling export workflow/`
at the frozen SHA. Dispositions: ALIGNED 25, PARTIALLY_IMPLEMENTED 1,
STALE_SETUP_SPECIFICATION 1, REMAINING_STATE_MISMATCH 3. Confidence: HIGH 26,
MEDIUM 4, LOW 0. Authority: NO 18, OWNER 11, ENGINEERING 1. All 30 rows are
mechanically `SelectableUnderCurrentLoop=NO`; `_STATUS.md` is bootstrap-only.

## Substantive reading and self-flags

- `REQ-008` is PARTIALLY_IMPLEMENTED. The provider-neutral Python workflow and
  JSON Schema inputs satisfy the current functional slice, but the requirement
  names a Rust core/application-service basis and JCS-compatible hashing. The
  module's `canonical_json` is stable sorted compact JSON, not demonstrated
  RFC-8785/JCS, and the tests preserve supplied checksum labels rather than
  compute/verify them. This is an architecture/canonicalization question, not
  an adverse professional or interoperability ruling.
- `DECL-002` is stale only at its explicit authority-pointer grain. Datasheet
  construction facts are current, but its reference list still presents
  revision 0.7 and DAG-006 while the frozen decomposition is revision 0.8 and
  `_DEPENDENCIES.md` names DAG-007. Specification, post-remediation Guidance,
  Procedure, status, and memory remain aligned on their current facts.
- `REM-001..003` preserve all three recorded findings. Their technical defects
  are addressed and independently rechecked, but each retains
  `HumanDisposition=TBD`; the package-audit row is
  `TECHNICALLY_ADDRESSED_PENDING_HUMAN` and RF-001/RF-002 remain OPEN. Because
  none appears on `_STATUS.md`, addendum 13 yields MEDIUM+OWNER unhomed
  REMAINING_STATE_MISMATCH rows rather than inferred closure.
- Fan-in should also inspect the negative-boundary rows `REQ-006`, `REQ-007`,
  `ACC-005`, `ACC-006`, `EXC-003`, and `EXC-004`; ALIGNED means only that the
  current invented/provider-neutral slice enforces its stated negative
  boundaries. It does not imply professional approval, legal clearance,
  security assurance, external validation, or target-format suitability.

## Evidence and reliability

Under addendum-9 controls at the frozen SHA:

- `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest -q -p no:cacheprovider projects/chirality-piping/tests/test_handoff_export_workflow.py` returned **6 passed**.
- `PYTHONDONTWRITEBYTECODE=1 python3 projects/chirality-piping/tools/validation/validate_dependencies_schema.py <DEL-15-03>/Dependencies.csv` returned **VALID**, 29 columns and 16 data rows.

The focused suite validates both upstream schemas using Draft 2020-12,
determinism, preserved context, explicit unsupported/approximate behavior,
unit/hash mismatches, target-metadata authority screening, and negative output
wording. It does not exercise a commercial target, external solve/prover,
target-specific unit conversion, physical package container, or JCS conformance
vector. All 24 substantive rows are `UNVERIFIED`; the six declaration rows are
`NOT_APPLICABLE`. Human approval of the earlier CHECKING transition does not
dispose the three findings or elevate unrelated technical evidence.

## Addendum-9 containment and fences

Ignored-aware porcelain before and after verification contained exactly the
six allow-listed incident paths and no seventh path: project `.pytest_cache/`,
the two reporting `Cargo.lock` files, the reporting/tests `__pycache__/` paths,
and `validation/benchmarks/nonlinear/target/`. Pytest used
`-p no:cacheprovider` with bytecode disabled; no cargo or in-tree `py_compile`
ran. The seeded D-41 item is copied byte-exactly only on `DECL-005` and is
excluded from residual, gate, and selectability analysis.

No PKG-14 artifact, frozen source, deliverable, product, lifecycle, DAG,
register, dependency, R4, or R5 surface was changed. All dispositions/routes
are agent judgments, never owner, engineering, professional, compliance,
security, validation, or release rulings.

## Cross-ledger risks for PKG-15 fan-in and R3

1. DEL-15-03 consumes DEL-15-01/02 schema authority and references PKG-10,
   PKG-12, PKG-13, and PKG-14 contracts; consumption must not be aggregated as
   duplicate ownership.
2. Provider-neutral envelope preservation does not establish a canonical
   physical package container, target-specific mapping coverage, target-format
   compatibility, or external execution/result ingestion.
3. Stable sorted compact JSON does not prove RFC-8785/JCS canonicalization or
   the correctness of supplied checksum values; deduplicate this with PKG-14's
   canonicalization-fidelity finding.
4. Unit metadata preservation and missing-unit blocking do not validate
   target-specific conversion correctness or engineering suitability.
5. Technically addressed review findings remain human-undispositioned; dedupe
   the underlying finding status/home rather than treating the three REM rows
   as product defects.
6. Seven dependency satisfaction cells remain TBD even though the local mirror
   is ACTIVE and schema-valid. Register validity is not dependency closure.
7. OI-015/DEL-17 target-format deferrals are bounded ownership facts, not
   authorization to implement parsers or claim interoperability breadth.
