# Package Concordance Summary — PKG-07 (Graphical User Interface and Engineering Workflow)

> **Epistemic status: agent-authored, non-operative evidence** (R2 wave W3,
> run DELIVERABLE_CONCORDANCE_2026-07-11_1305). Computed from the eight W3
> claim ledgers after the fan-in verification pass and the applied string
> corrections. No disposition here is an owner or engineering ruling;
> nothing edits any deliverable. Frozen source state:
> `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.

## Census

8 deliverables (DEL-07-01 3D viewport and centerline editor, DEL-07-02
model tree and property inspector, DEL-07-03 material/component/rule-pack
editors, DEL-07-04 missing-data warning and blocking UX, DEL-07-05 results
viewer, DEL-07-06 accessibility and usability baseline, DEL-07-07 solve
execution UX, DEL-07-08 design-authoring state and comparison workspace),
all `IN_PROGRESS`; 168 claim rows (25/20/23/21/21/18/23/17); ledgers
`WAVES/W3/CLAIM_CONCORDANCE_DEL-07-0*.csv`. All pilots fable per the
Receipt-17 steer. Verification: `WAVES/W3/W3_VERIFICATION_PKG-07.md` —
**8/8 SOUND** (66 PASS / 25 QUALIFIED / 0 FAIL over 91 checks); no
re-runs; three string corrections applied post-fan-in and verified in the
current CSVs (DEL-07-04 EXC-001/EXC-002 gate cells `UNGATED` →
`NONE_RECORDED` per convention 5; DEL-07-03 and DEL-07-05 line endings
LF → CRLF; the optional ` ; ` joiner harmonization was not applied —
cosmetic). Package-specific F-PIP fence-adjacency scan: **clean** (§Fences).

## ClaimType × Disposition (computed from ledgers)

| ClaimType | total | ALIGNED | PARTIALLY_IMPLEMENTED | STALE_SETUP_SPECIFICATION | VERIFIED_NOT_VALIDATED | DOCUMENTED_UNIMPLEMENTED | IMPLEMENTED_DIFFERENTLY | UNKNOWN |
|---|---|---|---|---|---|---|---|---|
| REQUIREMENT | 84 | 69 | 6 | 0 | 7 | 2 | 0 | 0 |
| ACCEPTANCE | 15 | 15 | 0 | 0 | 0 | 0 | 0 | 0 |
| EXCLUSION | 14 | 13 | 0 | 0 | 0 | 0 | 1 | 0 |
| DECLARED_STATE | 48 | 19 | 0 | 29 | 0 | 0 | 0 | 0 |
| REMAINING_WORK | 7 | 6 | 0 | 0 | 0 | 0 | 0 | 1 |

`SelectableUnderCurrentLoop=YES`: 9 rows (DEL-07-02 DECL-005/REM-001..004,
DEL-07-05 DECL-005/REM-001, DEL-07-06 DECL-005/REM-001) — mechanical
derivation reproducing the R1 inventory grain; the owner suspension
remains a run-level caveat. No IMPLEMENTED_UNMAPPED, ACCEPTED_DIVERGENCE,
or AUTHORITY_CONFLICT rows in the package.

## Package reading

PKG-07 inverts the W2 pattern: the kits are setup-era documents while the
GUI slices they anticipated are now implemented, so staleness is heavy
(29/48 DECL rows STALE) and sits on kit surfaces verified to make
now-false claims about the frozen slice ("does not implement GUI source",
absent checker tools, rev-0.7 pointers, overtaken TBD registers).
Requirement substance is strong: 69/84 ALIGNED on frozen-tree inspection,
re-executed side-effect-free Python contract tests, recounted static
suites (57 Vitest, 6 crate tests), and verifier-re-run addendum-10 empty
diffs binding recorded Vitest/Playwright passes to the frozen SHA. W2's
top risk (PKG-05 SourceReliability leg-keying) is **closed here**: all
eight pilots applied the weakest-load-bearing-leg rule uniformly — zero
REVIEWED/VETTED; UNVERIFIED 118, NOT_APPLICABLE on the 48 DECL rows plus
two DEL-07-03 process-claim rows (REQ-001/REQ-013) — so the ladder
histograms are comparable package-wide.

Substantive non-staleness findings:

- 7 VERIFIED_NOT_VALIDATED rows: 6 in DEL-07-06 (deferred usability/
  accessibility validation basis) plus DEL-07-03-REQ-011 (SECURITY,
  owner-gated sufficiency review, exact convention-6 marker) — a
  claim-class posture, not an evidence regression.
- 6 PARTIALLY_IMPLEMENTED REQUIREMENT rows (per-entity vs contract grain;
  none disputed) and 2 DOCUMENTED_UNIMPLEMENTED (DEL-07-03 REQ-005/006:
  no load-case or support editor in this deliverable's orbit).
- DEL-07-03-EXC-001 IMPLEMENTED_DIFFERENTLY + `SCOPE_CHANGE` — the third
  overtaken-exclusion encoding shape in the corpus — and DEL-07-05-REM-002
  UNKNOWN + OWNER (rotational-deformation visualization candidate the
  governing plan closed over: a routed question, not a recorded residual).

## Decision findings (routing, not rulings)

AuthorityNeeded: NO 142, OWNER 25, SCOPE_CHANGE 1; zero REVIEW/ENGINEERING
rows. OWNER is dominated by 21 STALE DECL kit surfaces (rev-drift,
overtaken setup prose); the four substance OWNER rows are genuine
adjudications — DEL-07-02-REQ-002 (property-inspector architecture home,
two-endpoint with W1 DEL-00-05-REQ-004), DEL-07-03-REQ-011 (SECURITY
sufficiency review), DEL-07-06-REQ-004 (WCAG target deferral),
DEL-07-05-REM-002 (residual status). Executor-work TBDs uniformly NO; no
gate-named token routed open (D-02b used as DecisionBasis only).

## Verification and repair record

Fan-in (fable, high effort): all eight ledgers SOUND; 0 FAIL. The
verifier independently recounted every histogram, re-ran all claimed
addendum-10 diffs (all empty, ancestry confirmed), parsed the DEC-025
sweep JSON, verified all bootstrap `RecordedRemaining` cells byte-exact,
and re-derived selectability. The one convention-5 vocabulary error
(DEL-07-04 EXC gate cells) was adjudicated string-level on a SOUND ledger
(vs the W1 DEL-03-05 DEFECTIVE precedent: no residual, gate, or
selectability analysis touched) and corrected by the owning pilot with the
two LF→CRLF normalizations, revalidated, histograms unaffected. Upheld
adjudications include the DEL-07-08-REQ-002 non-resolving kit pointer
handled in-row (record present at the true path) and the single ATTESTED
use (DEL-07-01-ACC-005), a not-re-derivable-check variant needing an R3
scope ruling. No pilot used the byte-copy re-execution pattern.

## Cross-ledger risks carried forward (W4–W5 calibration / R3)

1. **Guidance/Procedure STALE-line split (top risk)**: 07-01/02/04/05/07/08
   STALE'd both surfaces, 07-03 kept both ALIGNED, 07-06 split — every call
   fact-checked correct on its own document, but the STALE histogram is not
   face-comparable. R3 rule candidate: STALE iff the surface carries a
   now-false factual declaration about the frozen slice; self-scoped
   completed-setup descriptions ALIGNED-with-note.
2. VERIFIED_NOT_VALIDATED vs ALIGNED+NOT_APPLICABLE-validation posture for
   GUI-class claims: R3 must not read the six DEL-07-06 VNV rows as
   regressions; candidate rule keys VNV to requirements that themselves
   demand a usability/conformance validation basis.
3. Overtaken-exclusion encoding species — three shapes corpus-wide
   (fold-into-DECL here, W2 ACCEPTED_DIVERGENCE, 07-03's
   IMPLEMENTED_DIFFERENTLY + SCOPE_CHANGE): dedupe at R3.
4. DEL-07-02-REQ-002 ↔ W1 DEL-00-05-REQ-004 property-inspector
   architecture-home question: surface as one owner question at R3, not two.
5. All-dated-MEMORY census inclusion reading (uniform DECL=6 here; W4/W5
   should adopt the always-census-with-note reading) — joint with the W2
   undated-head distinguisher item.
6. Conditional-clause vacuity grain (07-05/07-07/07-08) — one-line R3
   definition wanted; ATTESTED-marker scope (not-re-derivable checks vs
   record-not-present).
7. Cross-deliverable residual homing (07-01 residuals homed in 07-02
   `## Remaining`) — attribute to exactly one deliverable when aggregating
   remaining work.

## Fences

Discovery read-only; frozen tree porcelain empty before and after all
pilot, verifier, and correction operations; the package-specific F-PIP
fence-adjacency scan over all sixteen output files found no
release-readiness, issuance, certification, sealing, approval, or
code-compliance claim outside attributed quotes; no lifecycle/DAG/scope
change proposed as operative; all dispositions are agent judgments routed
via AuthorityNeeded.
