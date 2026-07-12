# Package Concordance Summary — PKG-08 (Reporting, Audit, and Reproducibility)

> **Epistemic status: agent-authored, non-operative evidence** (R2 wave W3,
> run DELIVERABLE_CONCORDANCE_2026-07-11_1305). Computed from the six W3
> claim ledgers after the fan-in verification pass, one owning-pilot
> byte-exactness correction, and four owning-pilot addendum-9 disclosure
> amendments. No disposition here is an owner or engineering ruling;
> nothing edits any deliverable. Frozen source state:
> `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.

## Census

6 deliverables (DEL-08-01 calculation report generator, DEL-08-02 audit
manifest and model hash, DEL-08-03 warnings/assumptions/provenance report
section, DEL-08-04 result export format, DEL-08-05 report
protected-content linter, DEL-08-06 state/comparison/handoff report
sections), all `IN_PROGRESS`; 132 claim rows; ledgers
`WAVES/W3/CLAIM_CONCORDANCE_DEL-08-0*.csv`. All pilots fable per the
Receipt-17 steer. Verification: `WAVES/W3/W3_VERIFICATION_PKG-08.md` —
**6/6 SOUND** (70 PASS / 11 QUALIFIED / 5 FAIL; all FAILs graded
string-correctable, none re-encoding-grade, all now corrected); the named
Part-C-style SECURITY spot-check of DEL-08-05's convention-6 encoding
**PASSED**. Post-fan-in corrections: DEL-08-06 DECL-005 en-dash
byte-exactness restored (`§§6–8`) and addendum-9 disclosure amendments on
DEL-08-01/04/05/06 (incident record below); dispositions, histograms, and
routing unchanged by all corrections.

## ClaimType × Disposition (computed from ledgers)

| ClaimType | total | ALIGNED | PARTIALLY_IMPLEMENTED | STALE_SETUP_SPECIFICATION | ACCEPTED_DIVERGENCE |
|---|---|---|---|---|---|
| REQUIREMENT | 70 | 69 | 1 | 0 | 0 |
| ACCEPTANCE | 4 | 4 | 0 | 0 | 0 |
| EXCLUSION | 16 | 16 | 0 | 0 | 0 |
| DECLARED_STATE | 37 | 15 | 0 | 22 | 0 |
| REMAINING_WORK | 5 | 5 | 0 | 0 | 0 |

`SelectableUnderCurrentLoop=YES`: 6 rows (DEL-08-01 DECL-005/REM-001..003,
DEL-08-04 DECL-005/REM-001) — mechanical derivation only; DEL-08-05's sole
residual is owner/stage-gated per DEC-058 → NO. No IMPLEMENTED_UNMAPPED,
UNKNOWN, AUTHORITY_CONFLICT, or ACCEPTED_DIVERGENCE rows in the package.

## Package reading

PKG-08's requirement substance is nearly uniformly ALIGNED (69/70), but
deliberately at **contract grain**: report-section contracts, schemas,
and crate/script suites verified against the frozen tree (all eleven
static test counts independently recounted by the verifier), while
rendered-report breadth is repeatedly judged via invented fixtures with
the deferred slice's downstream home named in-row. The single
PARTIALLY_IMPLEMENTED row (DEL-08-05-REQ-010) rests on independently
verified absent CI wiring, not on the SECURITY sufficiency deferral.
Weakest-leg discipline held package-wide: zero VETTED/REVIEWED;
recorded-only citations carry the `not re-executed at frozen SHA` marker;
no ATTESTED markers needed. Staleness concentrates entirely on
declared-state surfaces (22/37 STALE): setup-era "does not implement"
declarations verified verbatim against the implemented crates, overtaken
open-question registers, Procedures naming checker tools verified absent.
The corpus-wide rev-0.7→0.8 drift lands on a census surface only in
DEL-08-06's Datasheet (STALE-side there); in the other five kits it lives
off-census, carried as dated-MEMORY in-row notes with one caveat each —
a fact-correct departure from the W1 calibration item's assumed home.

## Decision findings (routing, not rulings)

AuthorityNeeded: NO 104, OWNER 28; no REVIEW/ENGINEERING/gate-named rows.
OWNER rows are the setup-era STALE DECL surfaces plus the convention-6
SECURITY marker rows (DEL-08-05 REQ-002/004/010/REM-001, DEL-08-02
REQ-002/008 — the latter also addendum-13 MEDIUM+OWNER over pending
finding PKG02-001). Confidence: 93 HIGH / 39 MEDIUM.

## Verification and repair record

Fan-in (fable, high effort, read-only): all six ledgers SOUND; all
histograms and requirement censuses reproduce; both addendum-10 qualifiers
independently re-run (ancestry confirmed, diffs empty over the claimed
paths). The five FAILs: (1) DEL-08-06's bootstrap cell transcribed `§§6-8`
with a hyphen for the frozen en dash — corrected by the owning pilot,
byte-exactness re-verified; (2–5) the **addendum-9 frozen-tree incident**:
four pilot re-executions wrote git-ignored artifacts into the frozen tree
— DEL-08-01/04 cargo tests (report_generator and result_export
Cargo.locks), DEL-08-06's py_compile (two __pycache__ dirs), DEL-08-05's
pytest (.pytest_cache) — plus one pre-existing W2-era
`validation/benchmarks/nonlinear/target/` found in the same sweep. Plain
porcelain checks stayed truthfully empty throughout (ignored files are
invisible to them); no tracked content changed and the test results are
not invalidated, but the "side-effect-free" claims were falsified.
Disclosure amendments now sit on every affected evidence cell (9/14/6/14
rows on 08-01/04/05/06); physical restoration is escalated to the owner.
W4 mitigation per the verifier: `--ignored=matching` porcelain checks,
copy-out cargo for lockless crates (DEL-08-02 executed the item-12
pattern flawlessly), `pytest -p no:cacheprovider`, no in-tree
`py_compile`. Named SECURITY spot-check: DEL-08-05's marker/no-marker
split (deferred-sufficiency rows marked and OWNER-routed;
verification-complete rows explicit-reason NOT_APPLICABLE) — PASS.

## Cross-ledger risks carried forward (W4–W5 calibration / R3)

1. **Frozen-tree containment (top risk)**: plain porcelain cannot detect
   ignored-path writes; until restoration, byte-identity copy checks over
   result_export/report_generator would silently copy foreign lockfiles.
   W4 rules above; candidate R3 addendum per the verifier's §3.1.
2. SECURITY convention-6 scoping: codify the DEL-08-05
   "sufficiency-deferral-only" marker rule run-wide so later SECURITY
   deliverables do not blanket-apply the marker to every SECURITY row.
3. rev-drift home rule: STALE encoding only where the 0.7/DAG-006 pointer
   sits on a census DECLARED_STATE surface; elsewhere MEMORY-row note +
   one caveat — codify for W4+.
4. Contract-vs-rendered grain: R3 must read PKG-08's ALIGNED REQUIREMENT
   rows as contract-grain dispositions, not rendered-report validations.
5. Guidance-surface split (08-01/03/05 STALE vs 08-02/04 ALIGNED) is
   fact-driven, same species as the W2 Procedure split — aggregate
   knowingly at R3.
6. ACC-row minting variance (four kits mint one, two mint zero) —
   candidate R3 rule: ACC only where a distinct acceptance section exists.
7. Addendum-13 cap scope on non-load-bearing pending findings (08-02
   REQ-006 HIGH+NO beside PKG02-002) — joins the W1/W2 R3 queue.
8. Boilerplate-wide evidence cells (08-04) — aggregation-benign; adopt a
   cite-once-in-notes alias for W4/W5 bulk control.

## Fences

Discovery read-only in intent; tracked porcelain empty before and after
all pilot, verifier, and correction operations, but four pilots breached
addendum 9 on git-ignored paths (disclosed in-row and above; restoration
escalated); no lifecycle/DAG/scope change proposed as operative; no
F-PIP-1..5 claim language outside attributed quotes (this package's
sealing/compliance/protected-content subject matter is quoted, never
asserted); all dispositions are agent judgments routed via AuthorityNeeded.
