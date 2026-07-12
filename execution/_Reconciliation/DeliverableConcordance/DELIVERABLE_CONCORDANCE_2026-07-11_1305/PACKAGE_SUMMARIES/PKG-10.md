# Package Concordance Summary — PKG-10 (Build, Packaging, API, and Interoperability)

> **Epistemic status: derivative, agent-authored, non-operative evidence**
> (R2 wave W4, run DELIVERABLE_CONCORDANCE_2026-07-11_1305). Computed from
> the five corrected W4 ledgers after high-effort fan-in and owning-pilot
> corrections at wave commit `76aa949b530114196d0cb2b74d4ce3ca4db70433`.
> Accepted upstream evidence snapshot: frozen `main` at
> `551f84ef6be656f1603ce0acfa5e3935aa9683c7`. This summary is not
> decomposition truth and makes no release, compatibility, professional,
> owner, engineering, or lifecycle ruling.

## Census

5 deliverables (DEL-10-01 public API and plugin boundary, DEL-10-02
import-export adapter framework, DEL-10-03 local FEA handoff data contract,
DEL-10-04 build/packaging/CI-CD pipeline, DEL-10-05 headless CLI and structured
I/O runner), all `IN_PROGRESS`; **125 claim rows** (27/27/28/23/20). Discovery
was performed by GPT-5 pilots. Verification:
`WAVES/W4/W4_VERIFICATION_PKG-10.md` — highest-capability GPT-5, high effort;
**5/5 SOUND**, **60 PASS / 2 QUALIFIED / 2 FAIL** over mandatory checks. The
two DEL-10-05 correction clusters (gate/selectability defaults and REQ-007
SECURITY marker/routing) were applied by its owning pilot; no fresh discovery
was required. Current counts below come only from corrected CSVs.

## ClaimType × Disposition (computed from corrected ledgers)

| ClaimType | total | ACCEPTED_DIVERGENCE | ALIGNED | PARTIALLY_IMPLEMENTED | REMAINING_STATE_MISMATCH | STALE_REVIEW_OR_EVIDENCE | STALE_SETUP_SPECIFICATION |
|---|---:|---:|---:|---:|---:|---:|---:|
| REQUIREMENT | 54 | 2 | 46 | 6 | 0 | 0 | 0 |
| ACCEPTANCE | 17 | 0 | 15 | 2 | 0 | 0 | 0 |
| EXCLUSION | 14 | 0 | 14 | 0 | 0 | 0 | 0 |
| DECLARED_STATE | 30 | 0 | 13 | 0 | 0 | 0 | 17 |
| REMAINING_WORK | 10 | 4 | 4 | 0 | 1 | 1 | 0 |
| **Package total** | **125** | **6** | **92** | **8** | **1** | **1** | **17** |

`SelectableUnderCurrentLoop=YES`: **10 rows** — DEL-10-04 DECL-005,
REM-003/004 and DEL-10-05 REQ-002/004, ACC-001, DECL-005, REM-001..003. The
corrected total is mechanical; run-level suspension is not encoded per-row.
No DOCUMENTED_UNIMPLEMENTED, IMPLEMENTED_DIFFERENTLY, IMPLEMENTED_UNMAPPED,
UNKNOWN, VERIFIED_NOT_VALIDATED, or AUTHORITY_CONFLICT rows.

## Package reading

The package is strong at contract and bounded-runner grain: 46/54 REQUIREMENT,
15/17 ACCEPTANCE, and all 14 EXCLUSION rows are ALIGNED. The eight partial rows
preserve real runtime breadth gaps: explicit operation families/statuses in
DEL-10-01, rule/report runtime hooks in DEL-10-02, and persisted-project /
downstream export payload binding in DEL-10-05. Two DEL-10-05 requirement rows
are ACCEPTED_DIVERGENCE under DEC-065; four remaining-work divergences in
DEL-10-04/05 are likewise tied to explicit decisions, not inferred permission.

Seventeen of 30 declarations are STALE because setup/future/no-artifact prose
is overtaken by schemas, crates, docs, panels, packaging/readiness evidence,
and the local runner. The two adverse remaining-work encodings are distinct:
DEL-10-01's pending finding is a REMAINING_STATE_MISMATCH; DEL-10-05's
822-reference witness is STALE_REVIEW_OR_EVIDENCE against live 830-reference
output. Neither invalidates the independently re-executed bounded behavior.

SourceReliability: UNVERIFIED 94 / NOT_APPLICABLE 31. Confidence: HIGH 85 /
MEDIUM 40.

## Decision findings (routing, not rulings)

AuthorityNeeded: **NO 88 / OWNER 35 / D-12 2**. The two D-12 tokens are
gate-routing encodings on DEL-10-03 residual-bearing rows, not owner rulings or
work-queue instructions. Seven exact convention-6 markers remain after
correction: DEL-10-01 REQ-012; DEL-10-03 REQ-004/007 and EXC-002; DEL-10-04
REQ-006 and REM-001; DEL-10-05 REQ-007. Deterministic local/no-network,
schema, and adapter-boundary facts intentionally avoid blanket markers.

## Verification and repair record

Fan-in re-executed API/plugin static checks; adapter pytest 14 plus adjacent 7;
local-FEA pytest 1; readiness/sweep pytest 24; public-export non-cargo 16 with
4 deselected; adjacent SDK pytest 8; the DEL-10-05 Python script; and copied-
tree lockless Cargo tests 16 library + 1 preview + 5 final. DEL-10-05 was
corrected to use `NONE_RECORDED` on nine no-residual rows, YES on the three
residual-bearing substantive rows, and the exact marker+OWNER on REQ-007.
Corrected notes recount YES 7 / NO 13 for that ledger; the package total is the
10-row value above.

## Cross-ledger risks carried forward (W5 calibration / R3)

1. **Contract vs runtime grain:** ALIGNED API/adapter/handoff rows do not imply
   a public server, plugin loader, external FEA adapter, hosted CI, publication,
   or completed downstream runner payload.
2. **Acceptance grain:** DEL-10-02's eight, DEL-10-03's six, and DEL-10-05's
   bundled one-row acceptance are source-shape choices, not comparative
   maturity measures.
3. **SECURITY marker scope:** use the marker only for expressly deferred
   permission/provenance/protected-content/release sufficiency; deterministic
   private-data and local-execution boundaries remain explicit-reason
   NOT_APPLICABLE where appropriate.
4. **Residual-cell mechanics:** every row touched by an ungated recorded
   residual is selectable; rows without a residual use NONE_RECORDED, not
   UNGATED. The DEL-10-05 corrections are the W4 exemplar.
5. **Stale evidence vs current behavior:** DEL-10-05's stale 822-reference
   witness must not be flattened into a failed current runner; current copied-
   tree tests passed and the evidence defect is separately ledgered.
6. **Gate-token routing:** D-12 in AuthorityNeeded is a gate pointer, not a
   human authority category; normalize consciously in R3 rather than merging
   it with OWNER or executor work.
7. **Accepted divergence:** DEC-057/059/065-permitted packaging/runner states
   remain separate from ordinary implementation gaps and do not authorize
   release or publication.
8. **Lockless Cargo verification:** copy source plus relative dependencies and
   fixtures outside the frozen worktree; preliminary scratch-only dependency
   misses are not product-test failures.

## Fences

Frozen HEAD remained `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
Ignored-aware porcelain contained exactly the six addendum-9 allow-listed
incident paths before/after. Python used `PYTHONDONTWRITEBYTECODE=1`; pytest
used `-p no:cacheprovider`; lockless Cargo ran only on a copy with external
build output; no in-tree `py_compile`. No lifecycle, DAG, register, product,
publication, compatibility, release, professional, R4, or R5 action is
authorized by this derivative summary.
