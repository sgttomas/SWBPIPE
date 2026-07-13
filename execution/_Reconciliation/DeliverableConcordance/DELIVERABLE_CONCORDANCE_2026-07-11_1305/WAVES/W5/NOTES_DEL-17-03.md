# NOTES — DEL-17-03 Native open JSON export package (R2 W5)

Frozen evidence: `main` @
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Deliverable:
PKG-17 / DEL-17-03, lifecycle `IN_PROGRESS`. Role/model: **GPT-5
deliverable-grained owning pilot**.

Unqualified deliverable documents refer to
`execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-03_Native open JSON export package/`
at the frozen SHA; other paths are relative to `projects/chirality-piping/`.

## Census and histograms

The exact 20-column ledger contains **28 rows**.

| ClaimType | Count |
|---|---:|
| REQUIREMENT | 8 |
| ACCEPTANCE | 8 |
| EXCLUSION | 5 |
| DECLARED_STATE | 6 |
| REMAINING_WORK | 1 |
| **Total** | **28** |

| Disposition | Count |
|---|---:|
| ALIGNED | 26 |
| PARTIALLY_IMPLEMENTED | 1 |
| REMAINING_STATE_MISMATCH | 1 |
| **Total** | **28** |

Confidence: HIGH 26 / MEDIUM 2. AuthorityNeeded: NO 26 / OWNER 1 /
ENGINEERING 1. SourceReliability: UNVERIFIED 22 / NOT_APPLICABLE 6.
Selectability: NO 28. The declaration census is the four-document kit plus
`_STATUS.md` and `MEMORY.md`; no deliverable-owned README exists. Acceptance
rows retain six Verification Requirements plus two distinct Acceptance
Criteria from Specification.

## Self-flags for package fan-in

- **REQ-006 — PARTIALLY_IMPLEMENTED, MEDIUM, ENGINEERING.** Python sorted
  compact JSON is deterministic and excludes runtime timestamps from the hash
  seed, but the schema and emitted checksum records call it
  `JCS_compatible_json_payload_hash` without RFC 8785/JCS conformance vectors.
  Determinism is not treated as proof of canonicalization interoperability.
- **REM-001 — REMAINING_STATE_MISMATCH, MEDIUM, OWNER.** RF-002 remains
  OPEN/TBD and unhomed. Its fact is not merely historical: current canonical
  DAG-007 still records all four artifact-presence flags false while the four
  documents, semantic matrix, lensing artifact, and review surface exist.
  This ledger does not authorize a DAG edit or human disposition.
- **RF-001 is not a residual.** The implementation-posture conflict received
  named human `REVISE` disposition and is RESOLVED; the revised four-document
  kit now recognizes the bounded implementation outputs.
- **ACC-005 is bounded.** Successful import and tests establish the builder's
  current behavior, not the disputed JCS-compatible label.
- **REQ-005/ACC-006/EXC-005 are negative-boundary grains.** They establish
  false software-authority flags and blocking diagnostics, not solver
  validation, professional acceptance, legal clearance, or security
  assurance.
- SECURITY/governance negative-boundary rows use explicit-reason
  `NOT_APPLICABLE` and **zero convention-6 markers**; no named owner-gated
  sufficiency deferral exists.

## Verification and addendum-9 containment

Re-executed in the frozen worktree with `PYTHONDONTWRITEBYTECODE=1`, external
`PYTHONPYCACHEPREFIX=/tmp/d41-del1703-pycache`, and pytest cache disabled:

- `python3 -m pytest -q -p no:cacheprovider tests/test_native_json_export_package.py`
  — PASS, 6 tests.
- `python3 tools/validation/validate_dependencies_schema.py <DEL-17-03>/Dependencies.csv`
  — VALID, 29 required columns, 14 rows.
- `python3 -m json.tool` over the schema and invented fixture — both PASS.

No Cargo command or in-tree `py_compile` ran. Ignored-aware porcelain after
execution contained exactly the six known allow-listed paths and no seventh
path: project `.pytest_cache/`, the two reporting `Cargo.lock` files, the
state/comparison and tests `__pycache__/` paths, and
`validation/benchmarks/nonlinear/target/`. Tracked porcelain remained empty;
the frozen worktree was not modified or cleaned.

## Cross-ledger and R3 risks

1. **Contract ownership:** DEL-17-03 owns the project-native package
   foundation. It consumes DEL-17-01 source boundaries and DEL-17-02 profile,
   stable-ID, manifest, and loss-taxonomy contracts; references do not create
   duplicate ownership.
2. **Package versus handoff preview:** desktop native-package and DEL-15-01
   handoff unit-witness surfaces are consumers/parallel packets. They do not
   by themselves prove integration with the Python package builder or its
   schema.
3. **Canonicalization species:** deduplicate the JCS-compatible-label gap with
   PKG-08/14/15 canonicalization findings while preserving this emitted
   checksum-label surface.
4. **Units:** source/result unit witness counts and
   `conversion_performed=false` demonstrate disclosure, not conversion
   correctness, target suitability, or engineering validation.
5. **Target boundary:** a project-owned adapter-input package is not a public
   API, target writer, external compatibility promise, or commercial-format
   validation.
6. **DAG flag finding:** count RF-002 once as a canonical graph-currentness
   and formal disposition/home issue, not once per false boolean or snapshot.
7. **Dependency PENDING cells:** seven architecture-basis satisfaction values
   are currentness observations and authorize no dependency/DAG mutation.

## Fences

All dispositions are agent judgments, never human, owner, engineering,
professional, security, legal, validation, compatibility, or release rulings.
Writes were limited to this CSV and notes. No product, deliverable, lifecycle,
status, dependency, DAG, register, review, decision, R4, or R5 surface changed.
No public API, target adapter, target compatibility, project-store export,
solver validation, release, approval, certification, sealing, authentication,
code-compliance, professional reliance, legal clearance, or security assurance
claim is made.
