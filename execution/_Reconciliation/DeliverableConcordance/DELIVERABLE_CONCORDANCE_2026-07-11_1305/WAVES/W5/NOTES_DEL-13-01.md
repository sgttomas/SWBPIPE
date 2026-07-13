# NOTES — DEL-13-01 Design knowledge schema and provenance model (R2 wave W5)

Frozen source tree: `main` @ `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
Deliverable: PKG-13 / DEL-13-01, status IN_PROGRESS (DATA_MODEL_CHANGE).
Discovery pilot: deliverable-grained GPT-5 pilot.

All implementation and verification paths are relative to `WORKING_ROOT` =
`projects/chirality-piping`. Native requirement IDs `REQ-13-01-001` through
`REQ-13-01-011` are preserved in `NormativeSource`; fixed run ClaimIDs use the
`DEL-13-01-REQ-NNN` form.

## 1. Histograms (recount from the CSV; must reproduce exactly)

Disposition histogram (20 rows):

| Disposition | Count |
|---|---:|
| ALIGNED | 18 |
| PARTIALLY_IMPLEMENTED | 1 |
| STALE_SETUP_SPECIFICATION | 1 |
| **Total** | **20** |

ClaimType histogram (20 rows):

| ClaimType | Count |
|---|---:|
| REQUIREMENT | 11 |
| EXCLUSION | 3 |
| DECLARED_STATE | 6 |
| **Total** | **20** |

ACCEPTANCE = 0, REMAINING_WORK = 0, IMPLEMENTED_UNMAPPED = 0.

Confidence: HIGH = 18, MEDIUM = 2, LOW = 0. Authority: NO = 16,
OWNER = 3, ENGINEERING = 1. SelectableUnderCurrentLoop: NO = 20, YES = 0.

## 2. Self-flagged rows

- **DEL-13-01-REQ-005 and REQ-006 — ALIGNED, MEDIUM, OWNER.** The current
  schema and focused test independently establish the provenance and unit
  contracts, but the two load-bearing PKG-02 review findings remain
  `TECHNICALLY_ADDRESSED_PENDING_HUMAN` with `HumanDisposition=TBD`.
  Addendum 13 therefore caps confidence and routes owner authority without
  changing the independently supported substance disposition.
- **DEL-13-01-REQ-011 — PARTIALLY_IMPLEMENTED.** The governed schema contains
  the required no-bypass control vocabulary, but the frozen product-preview
  `load_design_knowledge` paths load a separate simplified invented fixture
  shape by JSON read and do not validate it against
  `schemas/design_knowledge.schema.json`. The row does not infer that every
  downstream consumer is defective; it records the directly observed gap
  against this requirement's schema/adapter/service breadth.
- **DEL-13-01-DECL-004 — STALE_SETUP_SPECIFICATION.** Procedure declares
  SOFTWARE_DECOMP revision 0.7 as its decomposition basis while the frozen
  authority document's front matter is `current_basis`, revision 0.8. Per the
  W1–W4 rev-drift calibration, the stale disposition is confined to the
  declared-state surface that carries the pointer.
- **DEL-13-01-DECL-003 — ALIGNED.** Guidance says this deliverable has no
  authoritative public example payload. Later product-preview fixtures are
  cross-deliverable downstream evidence and do not falsify that
  deliverable-scoped statement.

## 3. Evidence-execution log

Re-executed from the frozen evidence worktree with
`PYTHONDONTWRITEBYTECODE=1` and external
`PYTHONPYCACHEPREFIX=/tmp/d41-del1301-pycache`:

- `python3 tests/test_design_knowledge_schema.py` → **PASS** (exit 0). This is
  a stdlib assertion script, so pytest discovery correctly reports zero tests;
  the authoritative focused command is direct execution as specified by the
  deliverable Procedure and MEMORY.
- `python3 tools/validation/validate_dependencies_schema.py <DEL-13-01>/Dependencies.csv`
  → **VALID**, 29 required columns, 14 data rows.
- A diagnostic invocation of
  `python3 -m pytest -q -p no:cacheprovider tests/test_design_knowledge_schema.py`
  returned `no tests ran` because the file exposes a `main()` assertion
  harness rather than pytest test functions. It created no cache or bytecode
  and is not counted as validation evidence.

No Cargo command was needed. No in-tree `py_compile` was used.

Ignored-aware porcelain was checked before and after. It contained exactly the
six addendum-9 allow-listed ignored artifact sets and no additional path:
`.pytest_cache/`, `core/reporting/report_generator/Cargo.lock`,
`core/reporting/result_export/Cargo.lock`,
`core/reporting/state_comparison_handoff_sections/__pycache__/`,
`tests/__pycache__/`, and `validation/benchmarks/nonlinear/target/`.

## 4. Convention and calibration notes

- **Declaration census.** The four-document kit, `_STATUS.md`, and `MEMORY.md`
  yield six declarations. No deliverable-owned README exists. The dated MEMORY
  authority-refresh statement remains historical; the active Procedure pointer
  receives the rev-drift disposition.
- **No remaining-work row.** `_STATUS.md ## Remaining` contains only the
  byte-exact seeded D-41 bootstrap item, excluded under addendum 2. The MEMORY
  TBDs are explicitly downstream/homed scope boundaries rather than omitted
  DEL-13-01 status residuals.
- **No acceptance rows.** Specification Verification restates checks already
  carried by the eleven requirement rows and creates no distinct acceptance
  contract at addendum-12 grain.
- **No implemented-unmapped row.** The schema is explicitly owned by
  DEL-13-01. Desktop computed-unit context is recorded as a supporting
  DEL-13-01 evidence slice, while constraint, transform, and preview consumers
  retain their downstream ownership; none is absorbed into this deliverable.
- **SECURITY marker use: zero rows.** Protected-content exclusion is a bounded
  artifact-content claim and does not carry an accepted owner-gated security
  sufficiency review. The exact convention-6 marker is therefore not used.
- **Source reliability.** Technical rows use `UNVERIFIED`; current-state prose
  rows use `NOT_APPLICABLE`. Pending-human finding evidence does not become
  `REVIEWED` and does not provide the sole basis for REQ-005/006.
- **Runtime versus contract grain.** Schema structural requirements are
  ALIGNED when directly present. REQ-011 is partial because its own text
  reaches consuming schema/adapter/service paths and a concrete frozen loader
  bypasses the governed schema shape. This does not assert runtime assurance or
  release readiness.

All dispositions are agent judgments.

## 5. Boundary-compliance statement

All fences held. Discovery was read-only outside the two assigned W5 outputs.
No product file, lifecycle/status file, register, DAG, dependency record, or
review finding was edited; no lifecycle transition or R4/R5 repair was applied.
No release-readiness, security-assurance, certification, sealing, professional-
approval, or code-compliance claim is made. Frozen-tree ignored-aware porcelain
remained restricted to the six addendum-9 allow-listed paths at close.
