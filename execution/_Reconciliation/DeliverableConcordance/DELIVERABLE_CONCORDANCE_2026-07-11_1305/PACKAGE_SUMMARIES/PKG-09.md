# Package Concordance Summary — PKG-09 (Verification, Validation, and Quality Oracles)

> **Epistemic status: derivative, agent-authored, non-operative evidence**
> (R2 wave W4, run DELIVERABLE_CONCORDANCE_2026-07-11_1305). Computed from
> the five corrected W4 ledgers after high-effort fan-in and owning-pilot
> corrections at wave commit `76aa949b530114196d0cb2b74d4ce3ca4db70433`.
> Accepted upstream evidence snapshot: frozen `main` at
> `551f84ef6be656f1603ce0acfa5e3935aa9683c7`. This summary is not
> decomposition truth and makes no owner, engineering, validation,
> release-readiness, or lifecycle ruling.

## Census

5 deliverables (DEL-09-01 mechanics benchmark suite, DEL-09-02 stress recovery
benchmark suite, DEL-09-03 nonlinear support regression suite, DEL-09-04
validation manual skeleton, DEL-09-05 release quality gate checklist), all
`IN_PROGRESS`; **120 claim rows** (25/18/21/26/30). Discovery was performed by
GPT-5 pilots. Verification:
`WAVES/W4/W4_VERIFICATION_PKG-09.md` — highest-available-capability GPT-5,
high effort; pre-correction verdict 3 SOUND / 2 DEFECTIVE and **51 PASS / 1
QUALIFIED / 12 FAIL** across 64 checked rows. The two DEL-09-01 cell defects
and DEL-09-02 UTF-8 normalization defect were corrected by owning pilots; no
rediscovery or evidence rerun was required. Current counts below come only
from the corrected CSVs.

## ClaimType × Disposition (computed from corrected ledgers)

| ClaimType | total | ACCEPTED_DIVERGENCE | ALIGNED | IMPLEMENTED_DIFFERENTLY | PARTIALLY_IMPLEMENTED | REMAINING_STATE_MISMATCH | STALE_SETUP_SPECIFICATION |
|---|---:|---:|---:|---:|---:|---:|---:|
| REQUIREMENT | 45 | 0 | 42 | 1 | 2 | 0 | 0 |
| ACCEPTANCE | 20 | 0 | 18 | 0 | 2 | 0 | 0 |
| EXCLUSION | 10 | 0 | 10 | 0 | 0 | 0 | 0 |
| DECLARED_STATE | 34 | 0 | 13 | 0 | 0 | 0 | 21 |
| REMAINING_WORK | 11 | 3 | 4 | 0 | 0 | 4 | 0 |
| **Package total** | **120** | **3** | **87** | **1** | **4** | **4** | **21** |

`SelectableUnderCurrentLoop=YES`: **5 rows** — DEL-09-01 ACC-007,
DECL-005, REM-001 and DEL-09-04 DECL-005, REM-001. This is the mechanical
ledger result; the run-level owner suspension remains separate. No
DOCUMENTED_UNIMPLEMENTED, VERIFIED_NOT_VALIDATED, IMPLEMENTED_UNMAPPED,
UNKNOWN, STALE_REVIEW_OR_EVIDENCE, or AUTHORITY_CONFLICT rows.

## Package reading

Requirement substance is strong at the recorded benchmark/manual/checklist
grain: 42/45 REQUIREMENT rows ALIGNED. The four non-ALIGNED substantive rows
preserve bounded distinctions rather than adverse validation rulings:
DEL-09-01 coverage/suitability breadth and DEL-09-02 result-envelope breadth
are PARTIALLY_IMPLEMENTED; DEL-09-02's analytic-seed convention is
IMPLEMENTED_DIFFERENTLY under DEC-026. Acceptance is similarly strong (18/20
ALIGNED), while the two partial rows preserve benchmark coverage limitations.

Kit-side drift is substantial: 21/34 DECLARED_STATE rows are STALE because
setup/future prose, obsolete tool/evidence statements, and overtaken
implementation declarations no longer describe the frozen slice. The 11
remaining-work rows require more care than a gap count: three
ACCEPTED_DIVERGENCE rows are explicitly permitted by DEC-054/DEC-046, four
ALIGNED rows accurately represent governed residuals, and four
REMAINING_STATE_MISMATCH rows expose evidence-backed work absent from the sole
status surface. The 21-fixture/21-witness DEL-09-01 census is the corrected
binding W4 value.

SourceReliability is uniform at weakest-leg grain: UNVERIFIED 79 /
NOT_APPLICABLE 41; no REVIEWED or VETTED row. Confidence: HIGH 83 / MEDIUM 37.

## Decision findings (routing, not rulings)

AuthorityNeeded: **NO 75 / OWNER 41 / ENGINEERING 4**. OWNER is dominated by
stale declaration repair, pending human dispositions, owner-gated
SECURITY/release sufficiency, and status-homing questions. ENGINEERING routes
four bounded benchmark/threshold questions; it is not an engineering approval
or work authorization. Five exact convention-6 marker rows remain:
DEL-09-02 REQ-002/003, DEL-09-03 REQ-003, and DEL-09-05 REQ-003/008.

## Verification and repair record

Fan-in independently checked the 21 benchmark fixtures/witness notes, 63-case
validation-manual census, DEC-026/046/054/058/060/062 bases, all mandatory
self-flags/non-ALIGNED rows, RFC-4180/ID/header rules, and ignored-aware
containment. DEL-09-01 REQ-001 SourceReliability was corrected to UNVERIFIED
and ACC-007's actual DEL-10-05/DEL-09-04 homes restored. DEL-09-02's
double-decoded UTF-8 was normalized artifact-wide without changing semantics.
Corrected CSVs and notes contain no mojibake remnant and reproduce the matrix
above.

## Cross-ledger risks carried forward (W5 calibration / R3)

1. **Residual semantics are not histogram-equivalent:** DEC-054/046-permitted
   ACCEPTED_DIVERGENCE, accurately recorded ALIGNED work, and omitted-home
   REMAINING_STATE_MISMATCH must remain separate in R3.
2. **Evidence grain:** DEL-09-05 REQ-002's aggregate clean sweep is process
   evidence, not candidate-specific validation; its QUALIFIED fan-in reading
   must not become a release-readiness claim.
3. **Acceptance census grain:** package counts vary 7/0/1/5/7 by source shape;
   do not infer comparative maturity from raw ACCEPTANCE totals.
4. **SECURITY/release marker scope:** exact marker rows identify expressly
   owner-gated sufficiency/release judgments; deterministic benchmark facts do
   not receive the marker.
5. **Threshold routing:** ENGINEERING/OWNER cells route unresolved judgments;
   numeric floors, tolerances, or adequacy were not invented by discovery.
6. **Currentness deduplication:** benchmark evidence-system residuals span
   DEL-09-01/04/05 and downstream DEL-10-05; aggregate each governed work item
   once, using the named home rather than each citing row.
7. **Encoding integrity:** DEL-09-02 proved that semantic correctness does not
   excuse corrupt governed strings; W5 fan-in should retain artifact-wide UTF-8
   scans where suspicious sequences appear.

## Fences

Discovery and verification used the frozen SHA read-only. Ignored-aware
porcelain before/after contained exactly the six addendum-9 allow-listed
incident paths: project `.pytest_cache/`, two reporting `Cargo.lock` files,
two `__pycache__/` trees, and `validation/benchmarks/nonlinear/target/`.
W4 mitigation remained binding: `--ignored=matching`; copy-out cargo for
lockless crates; pytest `-p no:cacheprovider`; no in-tree `py_compile`. No
lifecycle, DAG, scope, product, validation, release, or R4/R5 action is
authorized by this derivative summary.
