# NOTES — DEL-17-09 Export adapter SDK and additional targets (R2 W5)

Frozen evidence: `main` @
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Deliverable:
PKG-17 / DEL-17-09, lifecycle `IN_PROGRESS`. Role/model: **GPT-5
deliverable-grained owning pilot**.

Unqualified deliverable documents refer to
`execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-09_Export adapter SDK and additional targets/`
at the frozen SHA; other paths are relative to `projects/chirality-piping/`.

## Census and histograms

The exact 20-column ledger contains **37 rows**.

| ClaimType | Count |
|---|---:|
| REQUIREMENT | 13 |
| ACCEPTANCE | 11 |
| EXCLUSION | 6 |
| DECLARED_STATE | 6 |
| REMAINING_WORK | 1 |
| **Total** | **37** |

| Disposition | Count |
|---|---:|
| ALIGNED | 25 |
| PARTIALLY_IMPLEMENTED | 7 |
| STALE_SETUP_SPECIFICATION | 4 |
| ACCEPTED_DIVERGENCE | 1 |
| **Total** | **37** |

Confidence: HIGH 29 / MEDIUM 8. AuthorityNeeded: NO 25 / OWNER 11 /
ENGINEERING 1. SourceReliability: UNVERIFIED 31 / NOT_APPLICABLE 6.
Selectability: NO 37. The declaration census is the four-document kit plus
`_STATUS.md` and `MEMORY.md`; no deliverable-owned README exists. Acceptance
rows retain all five grouped Verification rows and all six category-level
REQ-007 acceptance records.

## Self-flags for package fan-in

- **REQ-001 — PARTIALLY_IMPLEMENTED, MEDIUM, ENGINEERING.** The foundation
  consumes DEL-17-02 concepts, but its JSON checksums label sorted compact
  Python JSON as JCS-compatible without RFC 8785/JCS conformance vectors.
- **REQ-007 plus ACC-006/008/010/011 — PARTIALLY_IMPLEMENTED, MEDIUM, OWNER.**
  The nine implemented checklist categories do not preserve every specified
  grain. Syntactic/schema validation, source location/admitted-use/review
  disposition, mechanics readiness, rule-check readiness, and reviewer-role/
  signoff-format fields are absent or collapsed.
- **ACC-005 — PARTIALLY_IMPLEMENTED, MEDIUM, OWNER.** Its Phase A write-scope
  check passed historically, but the current deliverable owns a later bounded
  schema/builder/fixture/test/manifest foundation. The acceptance wording does
  not distinguish those time slices.
- **DECL-001..004 — STALE_SETUP_SPECIFICATION, OWNER.** All four documents
  still describe an implementation-free contract-only state. The substantive
  requirement rows assess the implemented slice separately.
- **REM-001 — ACCEPTED_DIVERGENCE, MEDIUM, OWNER.** RF-001 is OPEN, while the
  named human-approved CHECKING transition explicitly accepted the warning as
  non-blocking. This permits bounded deferral but does not close the finding or
  authorize document cleanup here.
- **REQ-008/ACC-003 establish admission metadata only.** Denied grant records
  and blocking diagnostics are not proof of a production sandbox or runtime
  security boundary.
- SECURITY/governance negative-boundary rows use explicit-reason
  `NOT_APPLICABLE` and **zero convention-6 markers**; no owner-gated
  sufficiency deferral is present.

## Verification and addendum-9 containment

Re-executed in the frozen worktree with `PYTHONDONTWRITEBYTECODE=1`, external
`PYTHONPYCACHEPREFIX=/tmp/d41-del1709-pycache`, and pytest cache disabled:

- `python3 -m pytest -q -p no:cacheprovider tests/test_export_adapter_sdk.py`
  — PASS, 8 tests.
- `python3 tools/validation/validate_dependencies_schema.py <DEL-17-09>/Dependencies.csv`
  — VALID, 29 required columns, 18 rows.
- `python3 -m json.tool` over the schema and invented package fixture — both
  PASS.

No Cargo command or in-tree `py_compile` ran. Ignored-aware porcelain before
execution contained exactly the six known allow-listed paths. The same exact
six-path check was repeated after execution from the frozen root; no seventh
path appeared. Tracked porcelain remained empty and the frozen tree was not
modified or cleaned.

## Cross-ledger and R3 risks

1. **Contract ownership:** DEL-17-09 consumes DEL-17-01 source admission,
   DEL-17-02 export contracts, and DEL-10 plugin/adapter boundaries. Reused
   policy fields do not transfer those authoritative owners.
2. **Admission versus execution:** target registry, checklist, denied grants,
   and manifest are evidence records only. They implement no loader, process,
   network access, public endpoint, sandbox, writer, or target validation.
3. **Target evidence:** an invented candidate and even a source-basis-admitted
   record are not target support, compatibility, release readiness, or vendor
   validation.
4. **Checklist aggregation:** R3 should preserve the missing category grains
   without counting REQ-007 and four acceptance rows as independent product
   implementations.
5. **Units:** DEC-018 unit-policy evidence records no conversion and does not
   validate target conversion correctness or engineering suitability.
6. **Canonicalization:** deduplicate the JCS-label issue with DEL-17-03/06 and
   earlier package ledgers while retaining this adapter-manifest surface.
7. **Documentation cleanup:** four stale surfaces plus RF-001 are one setup-
   wording species with a separate formal finding state, not five product gaps.
8. **Dependencies:** seven PENDING and four TBD satisfaction values are
   currentness observations only and authorize no DAG/dependency edit.

## Fences

All dispositions are agent judgments, never human, owner, engineering,
professional, security, legal, validation, compatibility, or release rulings.
Writes were limited to this CSV and notes. No product, deliverable, lifecycle,
status, dependency, DAG, register, review, decision, R4, or R5 surface changed.
No runtime loader, public endpoint, sandbox approval, target writer, target
support, compatibility, solver validation, release, code compliance,
professional reliance, legal clearance, or security assurance claim is made.
