# NOTES — DEL-15-01 Canonical handoff package schema and manifest (R2 W5)

Frozen evidence: `main` @
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Deliverable:
PKG-15 / DEL-15-01, lifecycle `IN_PROGRESS`. Role/model: **GPT-5
deliverable-grained owning pilot**.

Unqualified deliverable-document names refer to
`execution/PKG-15_Handoff and External Prover Workflow/1_Working/DEL-15-01_Canonical handoff package schema and manifest/`
at the frozen SHA. Other paths are relative to `projects/chirality-piping/`.

## Census and histograms

The ledger contains **31 rows** in the exact adopted 20-column schema.

| ClaimType | Count |
|---|---:|
| REQUIREMENT | 12 |
| ACCEPTANCE | 6 |
| EXCLUSION | 4 |
| DECLARED_STATE | 6 |
| REMAINING_WORK | 3 |
| **Total** | **31** |

| Disposition | Count |
|---|---:|
| ALIGNED | 23 |
| ACCEPTED_DIVERGENCE | 2 |
| STALE_SETUP_SPECIFICATION | 4 |
| REMAINING_STATE_MISMATCH | 2 |
| **Total** | **31** |

Confidence: HIGH 27 / MEDIUM 4 / LOW 0. AuthorityNeeded: NO 23 / OWNER 8.
SourceReliability: UNVERIFIED 25 / NOT_APPLICABLE 6. Selectability: NO 31.

The declaration census is the four-document kit plus `_STATUS.md` and
`MEMORY.md`; no deliverable-owned README exists. The six ACCEPTANCE rows
preserve Specification V-01 through V-06 as distinct source-shaped checks.

## Self-flags for package fan-in

- **REQ-012 and ACC-003 — ACCEPTED_DIVERGENCE, MEDIUM, OWNER.** The schema
  and kit still fix the physical package container to `TBD`; DEC-028 is a
  named human ruling selecting a multi-member archive strategy while expressly
  leaving physical mechanics to bounded implementation. The addendum-11
  permission threshold is met. This does not claim the archive mechanics are
  implemented or authorize document/schema repair.
- **DECL-001 through DECL-004 — STALE_SETUP_SPECIFICATION.** Each active kit
  document carries the overtaken container-unresolved statement. Surviving
  target-neutral, provider-neutral, boundary, and physical-mechanics deferrals
  remain valid; staleness is confined to the false current-strategy fact.
- **REM-002 and REM-003 — REMAINING_STATE_MISMATCH, MEDIUM, OWNER.** RF-001
  and RF-002 were technically addressed, and later review recommended/received
  lifecycle advancement, but `Review_Findings.csv` still records both as OPEN
  with `HumanDisposition=TBD`. No formal closure or status home exists. The
  ledger does not perform that human disposition.
- **REQ-004 — ALIGNED at contract/preservation grain.** Schema fields and the
  desktop handoff witness preserve explicit units without conversion. This is
  not validation of conversion correctness or downstream target suitability.
- **REQ-006, REQ-011, ACC-004, EXC-003, and EXC-004 — bounded negative
  boundary claims.** They use explicit-reason `NOT_APPLICABLE` validation and
  zero convention-6 markers because no named owner-gated sufficiency deferral
  exists.
- **REM-001 and DECL-005 — current but non-selectable.** Phase H is stage-gated
  at v0.2 R6; the DEL-17-01 vendor-question clause further gates CAEPIPE
  MBF-specific claims. The D-41 bootstrap is transcribed only on DECL-005 and
  excluded from residual/gate/selectability analysis.

## Verification and addendum-9 containment

Re-executed from the frozen evidence worktree with
`PYTHONDONTWRITEBYTECODE=1` and external
`PYTHONPYCACHEPREFIX=/tmp/d41-del1501-pycache`:

- `python3 projects/chirality-piping/tests/test_handoff_package_schema.py` —
  PASS (exit 0). This direct assertion script runs
  `Draft202012Validator.check_schema`, validates the invented fixture, checks
  required fields/enums, scans prohibited automatic-status text, and asserts
  the professional boundary.
- `python3 projects/chirality-piping/tools/validation/validate_dependencies_schema.py <DEL-15-01>/Dependencies.csv`
  — VALID, 29 required columns, 15 rows.

No pytest command was needed; if used it would have included
`-p no:cacheprovider`. No Cargo command or in-tree `py_compile` ran.
Ignored-aware porcelain before and after contained exactly the six known
allow-listed incident paths and no seventh path:

1. `projects/chirality-piping/.pytest_cache/`
2. `projects/chirality-piping/core/reporting/report_generator/Cargo.lock`
3. `projects/chirality-piping/core/reporting/result_export/Cargo.lock`
4. `projects/chirality-piping/core/reporting/state_comparison_handoff_sections/__pycache__/`
5. `projects/chirality-piping/tests/__pycache__/`
6. `projects/chirality-piping/validation/benchmarks/nonlinear/target/`

Tracked porcelain remained empty; the frozen worktree was not modified or
cleaned.

## Cross-ledger and R3 risks

1. **Authoritative contract versus consumers.** DEL-15-01 owns the provider-
   neutral handoff envelope. The desktop `HandoffPanel` is a preview consumer;
   DEL-15-03 owns runtime export workflow; DEL-17 owns wire-format adapters.
   Do not convert shared use into duplicate ownership.
2. **Container strategy versus mechanics.** DEC-028 resolves archive strategy,
   not deterministic ordering, atomic save, compatibility windows, naming, or
   completed packaging. Deduplicate the ruled strategy from surviving
   implementation residue.
3. **Hash vocabulary.** The schema permits `JCS` and
   `JCS_compatible_json_payload_hash`; this contract verification does not
   independently prove emitted RFC-8785 fidelity. Join with PKG-08/14
   canonicalization findings rather than multiplying the same issue.
4. **Units grain.** Explicit unit disclosure/preservation is not conversion
   validation. Target-format conversions remain separate from the canonical
   manifest contract.
5. **Target metadata grain.** Reserved mapping and unsupported-behavior fields
   are ALIGNED at schema grain; they do not establish mapping production,
   target coverage, external-solver compatibility, or target suitability.
6. **Formal finding state.** Technical remediation, lifecycle recommendation,
   and formal human finding disposition are distinct. RF-001/RF-002 remain
   OPEN/TBD until their owning workflow records otherwise.
7. **Dependency TBD cells.** Six local dependency `SatisfactionStatus=TBD`
   values are currentness observations only and authorize no dependency or DAG
   edit.
8. **Phase-H deduplication.** The broad status residual spans PKG-13/15/17;
   aggregate its governed work once by owning deliverable rather than once per
   ledger row.

## Fences

All dispositions are agent judgments, never human, owner, engineering,
professional, security, legal, validation, compatibility, or release rulings.
Writes were limited to this CSV and notes. No product, deliverable, lifecycle,
status, dependency, DAG, register, review, decision, R4, or R5 surface changed.
No approval, certification, sealing, authentication, code-compliance,
professional reliance, external validation, commercial compatibility, legal
clearance, or security assurance claim is made.
