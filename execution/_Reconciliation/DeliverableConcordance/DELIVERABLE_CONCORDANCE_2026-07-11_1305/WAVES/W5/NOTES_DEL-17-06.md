# NOTES — DEL-17-06 Stress-neutral CSV/JSON package (R2 W5)

Frozen evidence: `main` @
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Deliverable:
PKG-17 / DEL-17-06, lifecycle `IN_PROGRESS`. Role/model: **GPT-5
deliverable-grained owning pilot**.

Unqualified deliverable documents refer to
`execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-06_Stress-neutral CSV JSON package/`
at the frozen SHA; other paths are relative to `projects/chirality-piping/`.

## Census and histograms

The exact 20-column ledger contains **35 rows**.

| ClaimType | Count |
|---|---:|
| REQUIREMENT | 11 |
| ACCEPTANCE | 12 |
| EXCLUSION | 5 |
| DECLARED_STATE | 6 |
| REMAINING_WORK | 1 |
| **Total** | **35** |

| Disposition | Count |
|---|---:|
| ALIGNED | 26 |
| PARTIALLY_IMPLEMENTED | 4 |
| STALE_SETUP_SPECIFICATION | 4 |
| ACCEPTED_DIVERGENCE | 1 |
| **Total** | **35** |

Confidence: HIGH 30 / MEDIUM 5. AuthorityNeeded: NO 26 / OWNER 8 /
ENGINEERING 1. SourceReliability: UNVERIFIED 29 / NOT_APPLICABLE 6.
Selectability: NO 35. The declaration census is the four-document kit plus
`_STATUS.md` and `MEMORY.md`; no deliverable-owned README exists. Twelve
acceptance rows preserve all five Phase A and seven future-implementation
Verification IDs from Specification.

## Self-flags for package fan-in

- **REQ-007 and ACC-006 — PARTIALLY_IMPLEMENTED, MEDIUM, OWNER.** The package
  preserves diagnostics, provenance, source refs/hashes, validation, and
  professional boundaries but has no input/contract fields for conditional
  unresolved-assumption or general reproducibility-reference passthrough.
  Source hashes are not silently treated as the whole reproducibility model.
- **ACC-005 — PARTIALLY_IMPLEMENTED, MEDIUM, OWNER.** Its setup-era check says
  exact CSV/JSON/manifest layout and comparison semantics remain TBD. Concrete
  package layout and diagnostic-only comparison semantics now exist, while
  target behavior and pass/fail tolerances remain genuinely deferred.
- **ACC-008 — PARTIALLY_IMPLEMENTED, MEDIUM, ENGINEERING.** CSV hashes use an
  explicit normalized ASCII/LF basis, but JSON checksums label sorted compact
  Python JSON as JCS-compatible without RFC 8785 conformance vectors.
- **DECL-001..004 — STALE_SETUP_SPECIFICATION, OWNER.** Every four-document
  surface still presents materialized schema/builder/fixture/test outputs as
  Phase A-only future or absent work. This is declaration staleness, not a
  conclusion that the implemented requirements are absent.
- **REM-001 — ACCEPTED_DIVERGENCE, MEDIUM, OWNER.** RF-001 remains OPEN, but
  the named human-approved CHECKING transition explicitly accepted the
  Phase A wording warning as non-blocking. That permits bounded deferral; it
  does not close the finding or authorize this run to rewrite documents.
- **RF-002 is not a residual.** Its active dependency-authority warning is
  RESOLVED; older run-record language remains dated history only.
- SECURITY/governance negative-boundary rows use explicit-reason
  `NOT_APPLICABLE` and **zero convention-6 markers**; no owner-gated
  sufficiency deferral is present.

## Verification and addendum-9 containment

Re-executed in the frozen worktree with `PYTHONDONTWRITEBYTECODE=1`, external
`PYTHONPYCACHEPREFIX=/tmp/d41-del1706-pycache`, and pytest cache disabled:

- `python3 -m pytest -q -p no:cacheprovider tests/test_stress_neutral_export_package.py`
  — PASS, 8 tests.
- `python3 tools/validation/validate_dependencies_schema.py <DEL-17-06>/Dependencies.csv`
  — VALID, 29 required columns, 23 rows.
- `python3 -m json.tool` over the schema and invented package fixture — both
  PASS.

No Cargo command or in-tree `py_compile` ran. Ignored-aware porcelain before
execution contained exactly the six known allow-listed paths. The same exact
six-path check was repeated after execution from the frozen worktree root; no
seventh path appeared. Tracked porcelain remained empty and the frozen tree
was not modified or cleaned.

## Cross-ledger and R3 risks

1. **Contract ownership:** DEL-17-06 consumes DEL-17-02 package/profile/ID-map
   contracts, DEL-08-04 result export, DEL-14-02 run records, and DEL-14-05
   comparison exports. Source-basis refs do not transfer ownership.
2. **Stress-neutral versus native JSON:** DEL-17-06 owns paired result CSV/JSON
   review/regression packaging; DEL-17-03 owns the native project JSON package.
   Shared manifest/loss/hash ideas are contract reuse, not duplicate surfaces.
3. **Desktop witness consumption:** later unit-disclosure and
   unit-preservation-witness UI packets do not prove Python schema/writer
   integration unless explicitly bound; they preserve units and perform no
   conversion.
4. **Units:** unit/dimension presence and `conversion_performed=false` do not
   establish numeric conversion correctness or engineering suitability.
5. **Canonicalization:** deduplicate the JCS-label species with DEL-17-03 and
   earlier package findings while preserving the distinct CSV text partition.
6. **Comparison semantics:** diagnostic-only export and source refs are not
   numeric tolerance validation, pass/fail authority, or model equivalence.
7. **Documentation cleanup:** four stale surfaces plus RF-001 are one
   setup-wording species with a formal open-finding state, not five product
   implementation gaps.
8. **Dependency state:** seven PENDING and five TBD satisfaction values are
   currentness observations only and authorize no DAG/dependency mutation.

## Fences

All dispositions are agent judgments, never human, owner, engineering,
professional, security, legal, validation, compatibility, or release rulings.
Writes were limited to this CSV and notes. No product, deliverable, lifecycle,
status, dependency, DAG, register, review, decision, R4, or R5 surface changed.
No vendor format, target compatibility, project-store export, solver
validation, comparison pass/fail, release, approval, certification, sealing,
authentication, code-compliance, professional reliance, legal clearance, or
security assurance claim is made.
