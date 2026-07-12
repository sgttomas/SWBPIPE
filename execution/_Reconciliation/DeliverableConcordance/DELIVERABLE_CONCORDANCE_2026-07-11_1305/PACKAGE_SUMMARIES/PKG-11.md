# Package Concordance Summary — PKG-11 (Documentation, Examples, and Education)

> **Epistemic status: derivative, agent-authored, non-operative evidence**
> (R2 wave W4, run DELIVERABLE_CONCORDANCE_2026-07-11_1305). Computed from
> the five corrected W4 ledgers after high-effort fan-in and owning-pilot
> corrections at wave commit `76aa949b530114196d0cb2b74d4ce3ca4db70433`.
> Accepted upstream evidence snapshot: frozen `main` at
> `551f84ef6be656f1603ce0acfa5e3935aa9683c7`. This summary is not
> decomposition truth and makes no owner, source-license, professional,
> educational-suitability, or lifecycle ruling.

## Census

5 deliverables (DEL-11-01 user guide skeleton, DEL-11-02 developer guide for
solver and rule packs, DEL-11-03 theory notes, DEL-11-04 invented educational
example models, DEL-11-05 contributor tutorial/onboarding), all `IN_PROGRESS`;
**115 claim rows** post-correction (19/31/24/16/25; 114 at fan-in). Discovery
was performed by GPT-5 pilots. Verification:
`WAVES/W4/W4_VERIFICATION_PKG-11.md` — highest-available-capability GPT-5,
high-effort adversarial fan-in; pre-correction verdict 1 SOUND / 4 DEFECTIVE,
claim-row **69 PASS / 3 QUALIFIED / 10 FAIL** and combined **78 PASS / 4
QUALIFIED / 11 FAIL**. All bounded corrections were applied by owning pilots;
DEL-11-03 required none. Current counts below come only from corrected CSVs.

## ClaimType × Disposition (computed from corrected ledgers)

| ClaimType | total | ALIGNED | PARTIALLY_IMPLEMENTED | REMAINING_STATE_MISMATCH | STALE_SETUP_SPECIFICATION |
|---|---:|---:|---:|---:|---:|
| REQUIREMENT | 52 | 50 | 2 | 0 | 0 |
| ACCEPTANCE | 17 | 16 | 1 | 0 | 0 |
| EXCLUSION | 12 | 12 | 0 | 0 | 0 |
| DECLARED_STATE | 30 | 11 | 0 | 0 | 19 |
| REMAINING_WORK | 4 | 0 | 0 | 4 | 0 |
| **Package total** | **115** | **89** | **3** | **4** | **19** |

`SelectableUnderCurrentLoop=YES`: **0 rows**. All five `_STATUS.md` files are
bootstrap-only; four evidence-backed residuals are precisely mismatches because
they are absent from the sole status surface, so addenda 2/12 keep them
NONE_RECORDED and non-selectable. No ACCEPTED_DIVERGENCE,
DOCUMENTED_UNIMPLEMENTED, IMPLEMENTED_DIFFERENTLY, IMPLEMENTED_UNMAPPED,
UNKNOWN, VERIFIED_NOT_VALIDATED, STALE_REVIEW_OR_EVIDENCE, or
AUTHORITY_CONFLICT rows.

## Package reading

Product-document substance is strong: 50/52 REQUIREMENT, 16/17 ACCEPTANCE,
and all 12 EXCLUSION rows are ALIGNED at guide/theory/fixture/setup-run grain.
The three partial rows form one currentness species: DEL-11-01's broad guide
inventory and DEL-11-02's requirement/acceptance treatment still call resolved
typed-AST grammar/current authority details TBD. The DEL-11-02 correction adds
the corresponding fourth REMAINING_STATE_MISMATCH row rather than leaving the
defect as a notes-only caveat.

Staleness is wholly declaration-side: 19/30 DECL rows. Fan-in made the
Guidance/Procedure rule fact-driven. Surviving advisory Guidance in DEL-11-01
and DEL-11-05 is ALIGNED; DEL-11-02/03/04 Guidance remains STALE only where a
specific grammar/license/source/materialization fact is overtaken. Historical
setup Procedures remain ALIGNED when still executable and self-scoped;
Procedures with dead helper commands, obsolete authority instructions, or
false future-dependency facts remain STALE on those grounds—not merely because
later authorized product documentation exists.

SourceReliability: UNVERIFIED 83 / REVIEWED 2 / NOT_APPLICABLE 30. The two
REVIEWED rows are DEL-11-01 canonical status-wording claims covered by the
named human `ACCEPT_AS_IS` disposition; they do not lift unrelated technical
or source-license evidence. Confidence: HIGH 88 / MEDIUM 27.

## Decision findings (routing, not rulings)

AuthorityNeeded: **NO 87 / OWNER 28**. OWNER routes stale-document repair,
four residual-homing questions, and pending PKG-02/source-selection
dispositions. Post-correction, PKG-11 has **zero convention-6 marker rows**:
guide and invented-fixture content exclusions are bounded document/fixture
checks, not accepted owner-gated security-sufficiency reviews. DEL-11-04's
REQ-003/006 and REM-001 retain OWNER for their actual pending human findings.

## Verification and repair record

Fan-in checked 82 required claim rows plus package mechanics, re-ran the user-
guide status test (1) and invented-example suite (7), validated all five
dependency CSVs, and verified the three explicit external-source scopes in
DEL-11-03 remained bounded. Corrections removed three over-applied SECURITY
markers, restored advisory Guidance to ALIGNED in DEL-11-01/05, rebased three
Procedure staleness rows on actual dead/overtaken instructions, changed
DEL-11-02 REQ-012/ACC-007 to partial, and added its currentness REM-001. The
matrix and routing totals above reproduce the corrected ledgers.

## Cross-ledger risks carried forward (W5 calibration / R3)

1. **Guidance/Procedure staleness:** STALE requires a falsified present-state
   fact/register/operational instruction; surviving advisory principles and
   self-scoped completed setup history are ALIGNED-with-note.
2. **Acceptance grain:** 0/8/1/0/8 ACCEPTANCE rows reflect source-document
   structure. DEL-11-02/05's eight-row encodings preserve review obligations,
   but raw counts are not maturity measures.
3. **Product-currentness deduplication:** DEL-11-01 and DEL-11-02 both expose
   rev/TBD guide refresh work. Preserve deliverable-local rows but synthesize
   the shared authority-pointer/grammar species once at R3.
4. **SECURITY marker restraint:** protected-content scans and invented-fixture
   boundaries do not automatically create an owner-gated sufficiency review;
   require a row-specific accepted deferral before using convention 6.
5. **External-source evidence:** DEL-11-03's NASA/MIT/Open Textbook assertions
   remain agent spot-check evidence, not legal clearance; three claim-limited
   source scopes remain human-DEFERRED and unhomed.
6. **Pending-disposition scope:** DEL-11-04's two PKG-02 findings cap and route
   only load-bearing rows; do not spread OWNER/MEDIUM to independent fixture
   content facts.
7. **Dead tool paths:** direct artifact inspection can support achieved setup
   acceptance while the Procedure pointer itself is STALE; ledger those two
   facts separately.

## Fences

Frozen reads and re-execution remained within addendum-9 controls. Ignored-
aware porcelain contained exactly the six allow-listed incident paths before
and after. Python used `PYTHONDONTWRITEBYTECODE=1`, external pycache, and
pytest `-p no:cacheprovider`; no cargo or `py_compile` ran in the frozen tree.
No product document, example, source-selection record, review disposition,
lifecycle, DAG, scope, professional, legal, R4, or R5 action is authorized by
this derivative summary.
