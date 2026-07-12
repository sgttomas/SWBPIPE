# NOTES — DEL-15-04 External prover boundary metadata (R2 W5)

Frozen evidence: `main` @
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Deliverable:
PKG-15 / DEL-15-04, lifecycle `IN_PROGRESS`. Role/model: **GPT-5
deliverable-grained owning pilot**.

Unqualified deliverable documents refer to
`execution/PKG-15_Handoff and External Prover Workflow/1_Working/DEL-15-04_External prover boundary metadata/`
at the frozen SHA; other paths are relative to `projects/chirality-piping/`.

## Census and histograms

The exact 20-column ledger contains **25 rows**.

| ClaimType | Count |
|---|---:|
| REQUIREMENT | 10 |
| ACCEPTANCE | 5 |
| EXCLUSION | 3 |
| DECLARED_STATE | 6 |
| REMAINING_WORK | 1 |
| **Total** | **25** |

| Disposition | Count |
|---|---:|
| ALIGNED | 23 |
| STALE_SETUP_SPECIFICATION | 1 |
| REMAINING_STATE_MISMATCH | 1 |
| **Total** | **25** |

Confidence: HIGH 24 / MEDIUM 1. AuthorityNeeded: NO 23 / OWNER 2.
SourceReliability: UNVERIFIED 19 / NOT_APPLICABLE 6. Selectability: NO 25.
The declaration census is the four-document kit plus `_STATUS.md` and
`MEMORY.md`; no deliverable-owned README exists. Five acceptance rows preserve
the five distinct Specification verification targets.

## Self-flags for package fan-in

- **DECL-002 — STALE_SETUP_SPECIFICATION, OWNER.** Datasheet explicitly
  presents revision 0.7 as the accepted decomposition basis; frozen
  `SOFTWARE_DECOMP.md` is revision 0.8 `current_basis`. The stale judgment is
  confined to that declaration surface.
- **REM-001 — REMAINING_STATE_MISMATCH, MEDIUM, OWNER.** RF-001 is technically
  repaired but remains formally OPEN with `HumanDisposition=TBD`, and the sole
  status work surface contains only the D-41 bootstrap. The ledger does not
  perform the missing human disposition.
- **Resolved blocker is not a residual.** `DEL-15-04-PKG02-001` is
  `ACCEPT_AS_IS` / `RESOLVED` after the June 7 human gate. It is calibration
  evidence for notes/tags screening, not an open REM row.
- **REQ-007 and ACC-003 — conditional human-acceptance boundary.** Current
  software creates no human acceptance record; it prevents automatic reliance
  and preserves hash-bearing external references. ALIGNED is limited to that
  conditional/negative contract grain.
- **REQ-006 and the desktop unit-policy evidence — metadata grain only.** The
  packet discloses units without conversion and invokes no prover. This is not
  external verification, target compatibility, or engineering validation.
- SECURITY/governance negative-boundary rows use explicit-reason
  `NOT_APPLICABLE` and **zero convention-6 markers**; there is no named
  owner-gated sufficiency deferral to mark.

## Verification and addendum-9 containment

Re-executed in the frozen worktree with `PYTHONDONTWRITEBYTECODE=1` and
external `PYTHONPYCACHEPREFIX=/tmp/d41-del1504-pycache`:

- `python3 projects/chirality-piping/tests/test_external_prover_boundary_metadata.py`
  — PASS. Six direct tests cover deterministic/schema-valid metadata,
  prohibited authority/lifecycle claims, embedded attachments, boundary-flag
  mutation, notes/tags screening, and output language.
- `python3 projects/chirality-piping/tools/validation/validate_dependencies_schema.py <DEL-15-04>/Dependencies.csv`
  — VALID, 29 required columns, 15 rows.

No pytest command was needed; if used it would include
`-p no:cacheprovider`. No Cargo command or in-tree `py_compile` ran.
Ignored-aware porcelain before and after contained exactly the six known
allow-listed paths and no seventh path: project `.pytest_cache/`, the two
reporting `Cargo.lock` files, the state/comparison and tests `__pycache__/`
paths, and `validation/benchmarks/nonlinear/target/`. Tracked porcelain remained
empty; the frozen worktree was not modified or cleaned.

## Cross-ledger and R3 risks

1. **Metadata versus execution:** DEL-15-04 owns provider-neutral descriptive
   boundary metadata. It does not own external solver invocation, commercial
   parsing, target writing, external validation, or acceptance.
2. **Handoff/export consumers:** links to DEL-15-01/02/03 and DEL-14-01 are
   references/consumers, not duplicate ownership of those contracts.
3. **Rejected claims versus accepted state:** proposed authority text is
   retained only as a rejected diagnostic. R3 must not aggregate rejected
   probe text as an actual approval/compliance state.
4. **Human acceptance conditionality:** absence of an automatic record and a
   hash-bound future boundary do not prove any external reviewer accepted a
   payload.
5. **Units:** desktop `unit_policy_evidence` preserves source/result units and
   `conversion_performed=false`; it does not validate target conversion.
6. **Review-state deduplication:** count RF-001 once as a formal disposition/
   home mismatch; do not re-open the separately resolved PKG02 blocker.
7. **Dependency TBD cells:** five `SatisfactionStatus=TBD` values are
   currentness observations only and authorize no dependency/DAG edit.

## Fences

All dispositions are agent judgments, never human, owner, engineering,
professional, security, legal, validation, compatibility, or release rulings.
Writes were limited to this CSV and notes. No product, deliverable, lifecycle,
status, dependency, DAG, register, review, decision, R4, or R5 surface changed.
No external validation, approval, certification, sealing, authentication,
code-compliance, professional reliance, commercial compatibility, legal
clearance, or security assurance claim is made.
