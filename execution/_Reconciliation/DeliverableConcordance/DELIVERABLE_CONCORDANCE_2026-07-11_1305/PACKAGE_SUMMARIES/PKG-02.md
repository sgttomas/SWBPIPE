# Package Concordance Summary — PKG-02 (Domain Model, Units, and Core Schemas)

> **Epistemic status: agent-authored, non-operative evidence** (R2 wave W1,
> run DELIVERABLE_CONCORDANCE_2026-07-11_1305). Computed from the five W1
> claim ledgers after the fan-in verification pass and fable re-run. No
> disposition here is an owner or engineering ruling; nothing edits any
> deliverable. Frozen source state: `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.

## Census

5 deliverables (DEL-02-01..05), all `IN_PROGRESS`; 134 claim rows.
Verification: `WAVES/W1/W1_VERIFICATION_PKG-02.md` — 4 SOUND, 1 DEFECTIVE
(DEL-02-04, repaired by fable re-run); the verifier re-executed 49 pytest
checks plus cargo core/units 13/13 and canonical_json 8/8 at the frozen SHA.

## ClaimType × Disposition (computed from ledgers)

| ClaimType | total | ALIGNED | PARTIALLY_IMPLEMENTED | STALE_SETUP_SPECIFICATION | VERIFIED_NOT_VALIDATED |
|---|---|---|---|---|---|
| REQUIREMENT | 84 | 69 | 14 | 0 | 1 |
| EXCLUSION | 17 | 17 | 0 | 0 | 0 |
| DECLARED_STATE | 31 | 15 | 0 | 16 | 0 |
| REMAINING_WORK | 2 | 2 | 0 | 0 | 0 |

(No ACCEPTANCE rows — all pilots read verification tables as requirement
restatements under addendum 12.) `SelectableUnderCurrentLoop=YES`: 9 rows,
all DEL-02-05 (ungated recorded residuals; mechanical derivation only).

## Package reading

This is W1's substantive-gap center. The domain-model/units/schema core is
broadly implemented (69/84 requirement rows ALIGNED with re-executed
evidence), but two deliverables carry real partial-implementation chains:

- **DEL-02-02 (units core)**: 7 PARTIALLY_IMPLEMENTED rows on the
  core-contract vs system-wide-wiring split, plus REQ-006
  VERIFIED_NOT_VALIDATED (conversion catalog verified by unit tests only —
  no engineering validation basis). DEC-018 (D-01, 2026-06-10) accepted the
  SI-canonical catalog the setup-era kit still declares TBD → 4 STALE DECL
  rows. Duplicate-authority observation routed to DEL-16-02: the
  validation-preview engine mirrors (rather than calls) `core/units` —
  vocabulary currently an exact 30-id match at the frozen SHA (fan-in
  corrected an overtaken omission claim), but the mirror structure is the
  surviving drift risk.
- **DEL-02-05 (persistence/round-trip)**: 6 PARTIALLY_IMPLEMENTED rows; the
  concrete defect chain is the recorded H2 residual's stale `"0.1.0"`
  version-check literals (verified line-exact: ProjectValidationPanel
  L255/L580/L582, ExportReviewPanel L269, ReportPanel L798) causing current
  0.2.0 documents to be flagged `unsupported_schema_review_required`; plus
  the FR-001 explicit-migrate-op/`.opsproj`-container residual. Both carried
  in REM rows; 9 mechanically selectable ungated residual rows.
- **DEL-02-01 (canonical schema)**: substance ALIGNED (schema fully
  implemented, 46 KB, DEC-068 slots); kit is setup-era → 4 STALE DECL rows
  (R5 candidates, OWNER). The Receipt-10 parallel-surface item
  (`g_factors.{x,y,z}`/`exposed_element_refs` vs session shape) confirmed as
  downstream translated surfaces, not a schema gap.
- **DEL-02-04 (plugin contracts)**: post-repair 24 ALIGNED / 1
  PARTIALLY_IMPLEMENTED (REQ-014 layered checks) / 2 STALE DECL rows
  (kit declares the implemented manifest schema/fixture/test "future");
  DEC-012 over-attribution corrected to the precise DEC-012/SOW-038 vs
  requirement-declared partition.
- **DEL-02-03 (analysis boundary)**: clean (22 ALIGNED, 2 STALE on
  draft-framing surfaces); F-PIP-2-adjacent no-compliance boundary held.

**Part C repair executed here:** the wave's first convention-6 SECURITY
encodings (DEL-02-04 REQ-013; DEL-02-05 REQ-013/REQ-025) were
reviewer-spot-checked — encoding PASSES (exact deferred-sufficiency string,
no VERIFIED_NOT_VALIDATED downgrade), with two harmonization notes below.

## Decision findings (routing, not rulings)

- OWNER: DEL-02-01 DECL-001..004 (kit rewrite candidates); DEL-02-02
  REQ-010/015/016 ("Not ruled here" open decisions) ; DEL-02-04
  DECL-002/004 + sandbox/registry sufficiency (REQ-013/REQ-017); DEL-02-05
  DECL-001..004 + version-policy chain.
- ENGINEERING: none ledgered (REQ-006's validation gap is routed as
  verification/validation distinction, NO — candidate engineering item at
  R3 if the catalog is promoted).
- REVIEW: DEL-02-02 REQ-002 basis.

## Unmapped implementation

None ledgered at material grain.

## Conflicts and unknowns

None (zero AUTHORITY_CONFLICT / UNKNOWN rows).

## Verification and repair record

DEL-02-04 DEFECTIVE → fable re-run: DECL-002/004 ALIGNED→STALE (kit
declares implemented artifacts "future"), REQ-014 ALIGNED→PARTIALLY_IMPLEMENTED
(package-consistent with DEL-02-02), DEC-012 over-attribution reworded;
independently re-verified, zero disagreements. Owning-pilot corrections on
SOUND ledgers: DEL-02-02 overtaken force_per_length claim corrected
(frozen-tree set is an exact 30-id match). Structural sweep + histogram
recount clean across all five ledgers. Two earlier structural repairs
pre-verification: DEL-02-02 DECL-007 SourceReliability (addendum 6),
DEL-02-04 ClaimID 3-digit re-pad (addendum 12).

## Cross-ledger risks carried forward (W2–W5 / R3)

1. **SECURITY-marker dash variant** (em-dash in DEL-02-04 vs ASCII hyphen in
   DEL-02-05): aggregate on the substring "sufficiency review deferred,
   owner-gated"; standardize the em-dash form for W2+.
2. **AuthorityNeeded on owner-gated SECURITY deferrals**: OWNER (DEL-02-04)
   vs NO (DEL-02-05) — harmonize to OWNER for W2+.
3. RecordedRemaining "§"→"section" transliteration breaks byte-exact
   verbatim joins (also seen in PKG-00/03) — treat as equivalent at R3.
4. EXCLUSION/ACCEPTANCE row grain varies (see PKG-00 risk 3).
5. Route the DEL-16-02 duplicate-authority observation to R3 WITHOUT the
   corrected (stale) omission detail.

## Fences

Discovery read-only; frozen tree clean throughout (all re-executions
sandboxed per addendum 9); no lifecycle/DAG/scope change proposed as
operative; no F-PIP-1..5 claim language outside attributed quotes; all
dispositions are agent judgments.
