# Package Concordance Summary — PKG-14 (Model States, Analysis Runs, and Comparison)

> **Epistemic status: derivative, agent-authored, non-operative evidence**
> (R2 wave W5, run DELIVERABLE_CONCORDANCE_2026-07-11_1305). Computed from
> five corrected W5 ledgers after independent high-effort fan-in and an
> owning-pilot correction. Accepted upstream evidence snapshot: frozen `main`
> at `551f84ef6be656f1603ce0acfa5e3935aa9683c7`. This summary is not
> decomposition truth and makes no owner, engineering, lifecycle,
> professional, security, or release ruling.

## Census

Five `IN_PROGRESS` deliverables cover immutable model states, analysis-run
records, model-state comparison, analysis-run comparison, and shared
mapping/tolerance/export contracts. The corrected ledgers contain **141 claim
rows** (26/28/30/24/33). Under the owner-authorized temporary runtime
exception, five distinct deliverable-pilot assignments ran in two
capacity-bounded batches (three then two) using GPT-5 deliverable pilots.
Independent highest-capability high-effort verification is recorded in
`WAVES/W5/W5_VERIFICATION_PKG-14.md`: pre-correction 4 SOUND / 1 DEFECTIVE and
77 PASS / 0 QUALIFIED / 2 FAIL across 79 scoped rows. The two failures were
the same bounded factual defect on DEL-14-04 REQ-001/ACC-002 and were corrected
by their owning pilot without changing dispositions or histograms.

## ClaimType × Disposition (computed from corrected ledgers)

| ClaimType | total | ALIGNED | PARTIALLY_IMPLEMENTED | REMAINING_STATE_MISMATCH | STALE_SETUP_SPECIFICATION | VERIFIED_NOT_VALIDATED |
|---|---:|---:|---:|---:|---:|---:|
| REQUIREMENT | 51 | 41 | 9 | 0 | 0 | 1 |
| ACCEPTANCE | 36 | 27 | 8 | 0 | 0 | 1 |
| EXCLUSION | 21 | 21 | 0 | 0 | 0 | 0 |
| DECLARED_STATE | 30 | 10 | 0 | 0 | 20 | 0 |
| REMAINING_WORK | 3 | 1 | 0 | 2 | 0 | 0 |
| **Package total** | **141** | **100** | **17** | **2** | **20** | **2** |

All 141 rows are mechanically `SelectableUnderCurrentLoop=NO`. DEL-14-01's
Phase G residual is stage-gated; DEL-14-03's two evidence-backed open findings
are unhomed rather than authorized work; the remaining deliverables are
bootstrap-only. There are no IMPLEMENTED_UNMAPPED, UNKNOWN,
AUTHORITY_CONFLICT, ACCEPTED_DIVERGENCE, or lifecycle-reassessment rows.

## Package reading

PKG-14 has a substantial implemented schema, persistence, and comparison
slice, but 100/141 ALIGNED is not a maturity score. The partial rows identify
bounded gaps in external-reference screening, required-value and unit
diagnostics, full JCS/RFC-8785 canonicalization, unit-field normalization,
automatic stable-ID matching, entity-category coverage, canonical comparison
output, report-section breadth, and CSV/report fixture coverage. DEL-14-04's
mixed-unit comparison behavior is verified but lacks an independent
engineering validation basis, yielding the two VERIFIED_NOT_VALIDATED rows.

Twenty setup-document declarations are stale at their own surface grain; they
do not invalidate implemented schemas or tests. DEL-14-03's two
REMAINING_STATE_MISMATCH rows preserve open, unhomed review findings. The
package reliability histogram is UNVERIFIED 111 / NOT_APPLICABLE 30;
Confidence is HIGH 113 / MEDIUM 28.

## Decision findings (routing, not rulings)

AuthorityNeeded is **NO 96 / OWNER 39 / ENGINEERING 6**. OWNER routes stale
authority/currentness declarations, gated or unhomed findings, and bounded
product/interface decisions. ENGINEERING routes unit-normalization and
comparison-validation questions; these counts are agent routing judgments,
not engineering rulings. SECURITY rows use explicit-reason `NOT_APPLICABLE`
where applicable; no convention-6 sufficiency marker was manufactured.

## Verification and correction record

Pilots and verifier exercised focused model-state, analysis-run, comparison,
schema, persistence, dependency, and contract surfaces under addendum-9
controls. The only fan-in correction changed DEL-14-04 REQ-001/ACC-002
evidence and remaining-work text to state the frozen fact: the engine
deterministically consumes caller-supplied mappings but does not derive
automatic stable-ID matches. `PARTIALLY_IMPLEMENTED`, MEDIUM confidence, NO
authority routing, and every package histogram remained unchanged.

## Cross-ledger risks carried forward (remaining W5 calibration / R3)

1. **Shared ownership versus consumption:** model-state/run schemas,
   persistence, comparison engines, reporting, audit, handoff, and export
   surfaces overlap PKG-08, PKG-10, PKG-15, and PKG-17; R3 must preserve the
   authoritative owner and treat downstream users as consumers.
2. **Canonicalization fidelity:** stable sorted compact JSON is not by itself
   proof of the emitted JCS/RFC-8785 label; deduplicate this species with audit
   and persistence hashing findings.
3. **Mapping grain:** caller-supplied mapping consumption is implemented;
   automatic stable-ID matching, heuristic equivalence, and upstream mapping
   production are distinct residuals and must not be collapsed.
4. **Units and validation:** required unit metadata and conversion guards do
   not establish numeric conversion correctness or engineering suitability.
5. **Export/report breadth:** schema vocabulary and preservation flags do not
   establish runtime CSV round-trip, canonical output, rendered reports, or
   downstream handoff completion.
6. **Unhomed findings and dependencies:** DEL-14-03's open findings and
   DEL-14-05's TBD dependency-satisfaction cells are closure/currentness
   observations, not authority to mutate dependency registers or product
   state.
7. **Setup-era staleness:** document-surface staleness must be deduplicated
   from genuine governed-value or implementation gaps and authorizes no R5
   rewrite.

## Fences

Frozen HEAD remained `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
Ignored-aware porcelain before/after contained exactly the six addendum-9
allow-listed paths and no seventh path. Python used
`PYTHONDONTWRITEBYTECODE=1`, external pycache, and pytest
`-p no:cacheprovider`; no in-tree `py_compile` or lockless Cargo run occurred.
No product, deliverable, lifecycle, DAG, register, dependency, R4, or R5
change is authorized or performed by this derivative summary.
