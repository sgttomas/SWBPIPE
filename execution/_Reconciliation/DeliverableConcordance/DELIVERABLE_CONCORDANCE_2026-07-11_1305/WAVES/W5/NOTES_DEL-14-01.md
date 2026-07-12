# NOTES — DEL-14-01 Immutable model state records (R2 wave W5)

Frozen source tree: `main` @
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Deliverable:
PKG-14 / DEL-14-01, lifecycle `IN_PROGRESS`. Discovery role/model:
deliverable-grained GPT-5 owning pilot.

Unqualified deliverable-document names in the ledger refer to
`execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-01_Immutable model state records/`
at the frozen SHA. All code, schema, test, and governance paths are relative to
`projects/chirality-piping/` unless stated otherwise.

## Census and histograms

The ledger has **26 claim rows** and the exact adopted 20-column schema.

ClaimType:

| ClaimType | Count |
|---|---:|
| REQUIREMENT | 10 |
| ACCEPTANCE | 5 |
| EXCLUSION | 4 |
| DECLARED_STATE | 6 |
| REMAINING_WORK | 1 |
| **Total** | **26** |

Disposition:

| Disposition | Count |
|---|---:|
| ALIGNED | 20 |
| PARTIALLY_IMPLEMENTED | 2 |
| STALE_SETUP_SPECIFICATION | 4 |
| **Total** | **26** |

Confidence: HIGH 23 / MEDIUM 3 / LOW 0. AuthorityNeeded: NO 20 / OWNER 6.
SourceReliability: UNVERIFIED 20 / NOT_APPLICABLE 6. Mechanical
selectability: NO 26 / YES 0 / UNKNOWN 0.

The declaration census is exactly the four-document kit plus `_STATUS.md` and
`MEMORY.md`. No deliverable-owned README exists. The five acceptance rows
preserve the five distinct grouped entries in Specification `## Verification`.

## Self-flagged rows for package fan-in

- **REQ-009 and ACC-005 — PARTIALLY_IMPLEMENTED, MEDIUM, OWNER.** The strict
  schema requires external-reference hashes, privacy classification, and
  provenance and keeps the professional boundary explicit. The focused
  persistence fixture exercises one invented reference. No mapped evidence
  demonstrates end-to-end runtime ingestion/consumption binding to schema
  validation, privacy controls, protected-content screening, and the
  professional boundary. This is a bounded control-reach judgment, not a
  security finding, legal ruling, or authorization to design the missing path.
- **DECL-001 through DECL-004 — STALE_SETUP_SPECIFICATION.** Specification,
  Datasheet, Guidance, and Procedure retain setup/future/TBD statements now
  overtaken by the frozen schema, persistence service, tests, DEC-017/DEC-028,
  or the revision-0.8 current authority. Staleness is confined to the affected
  declaration surfaces; the substantive schema rules are judged separately.
- **EXC-002 — ALIGNED, MEDIUM.** The comprehensive-commercial-prover-ingestion
  exclusion is consistent with the mapped slice, but the evidence is a bounded
  source search, not a repository-wide absence proof.
- **REM-001 and DECL-005 — ALIGNED, non-selectable.** The Phase G residual is
  an accepted, current stage-gated program home under DEC-056/SCA-005. The
  D-41 bootstrap is reproduced only on DECL-005 and excluded from residual,
  gate, and selectability analysis.
- **REQ-008, REQ-010, ACC-004, EXC-001, and EXC-003 — bounded negative
  boundary claims.** They intentionally do not use the convention-6 marker:
  no row asserts an accepted owner-gated security-sufficiency deferral. Their
  `NOT_APPLICABLE` validation text states the exact non-assurance reason.

## Verification and addendum-9 containment

The following were re-executed in the frozen worktree with
`PYTHONDONTWRITEBYTECODE=1`, external
`PYTHONPYCACHEPREFIX=/tmp/d41-del1401-pycache`, and pytest cache disabled:

- `python3 -m json.tool projects/chirality-piping/schemas/model_state.schema.json`
  — PASS.
- `python3 -m pytest -q -p no:cacheprovider projects/chirality-piping/tests/test_model_state_schema.py projects/chirality-piping/tests/test_project_persistence_service.py`
  — **16 passed**.
- `python3 projects/chirality-piping/tools/validation/validate_dependencies_schema.py <DEL-14-01>/Dependencies.csv`
  — VALID, 29 required columns, 13 data rows.

No Cargo command or in-tree `py_compile` ran. Lockless-Cargo copy-out was not
needed. Ignored-aware porcelain before and after contained exactly the six
allow-listed pre-existing incident paths and no seventh path:

1. `projects/chirality-piping/.pytest_cache/`
2. `projects/chirality-piping/core/reporting/report_generator/Cargo.lock`
3. `projects/chirality-piping/core/reporting/result_export/Cargo.lock`
4. `projects/chirality-piping/core/reporting/state_comparison_handoff_sections/__pycache__/`
5. `projects/chirality-piping/tests/__pycache__/`
6. `projects/chirality-piping/validation/benchmarks/nonlinear/target/`

Tracked porcelain remained empty. The frozen evidence worktree was not
modified or cleaned.

## Convention and cross-ledger notes

- **Residual grain:** the Phase G item is repeated on substantive rows whose
  model-state work it touches and is represented once as REM-001. It remains
  mechanically `NO` because its exact suffix is `stage-gated: v0.2 R3`.
- **Bootstrap discipline:** the D-41 item is byte-exact only on DECL-005 and
  never becomes a REM row or a gate/selectability input.
- **Declared-state currentness:** dated MEMORY entries remain historical. The
  active four-document surfaces receive fact-specific stale judgments where
  they still describe implemented/ruled details as future or TBD.
- **Contract versus runtime:** ALIGNED schema/persistence rows establish the
  strict record and focused round-trip/hash slice only. They do not establish
  state-browser, comparison, reporting, handoff, runtime screening,
  professional reliance, or release-readiness breadth.
- **Duplicate ownership risk for R3:** model-state hashing/persistence is shared
  with DEL-02-05 and DEL-08-02; desktop model-hash review surfaces also cite
  DEL-14-01. These are consumer/supporting surfaces, not evidence that
  DEL-14-01 owns the canonical project schema, the audit-manifest contract, or
  every desktop persistence behavior.
- **Physical-container history:** the setup kit's former container TBD is
  partly overtaken by DEC-017 and DEC-028. Archive mechanics remain bounded
  downstream work; do not flatten the ruled strategy and surviving mechanics
  residue into one closed/unclosed claim.
- **No unmapped implementation row:** the model-state schema and focused
  persistence support are explicitly mapped to DEL-14-01 in the R1 surface
  index. Shared desktop/project-persistence consumers remain co-attributed.

## Fences

All dispositions are agent judgments, never owner, engineering, security,
legal, professional, validation, or release rulings. Discovery wrote only this
CSV and notes file. No product, deliverable, lifecycle, status, dependency,
DAG, register, review, decision, R4, or R5 surface changed. No professional
approval, certification, sealing, authentication, code-compliance, legal
clearance, security assurance, or external validation claim is made.
