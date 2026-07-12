# Package Concordance Summary — PKG-03 (Piping Components, Materials, and Library Data Model)

> **Epistemic status: agent-authored, non-operative evidence** (R2 wave W1,
> run DELIVERABLE_CONCORDANCE_2026-07-11_1305). Computed from the eight W1
> claim ledgers after the fan-in verification pass and fable re-runs. No
> disposition here is an owner or engineering ruling; nothing edits any
> deliverable. Frozen source state: `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.

## Census

8 deliverables (DEL-03-01..08), all `IN_PROGRESS`; 146 claim rows.
Verification: `WAVES/W1/W1_VERIFICATION_PKG-03.md` — 6 SOUND, 2 DEFECTIVE
(DEL-03-04, DEL-03-05; both repaired by fable re-runs); all four cited
pytest suites re-executed by the verifier (24/24).

## ClaimType × Disposition (computed from ledgers)

| ClaimType | total | ALIGNED | ACCEPTED_DIVERGENCE | PARTIALLY_IMPLEMENTED | STALE_SETUP_SPECIFICATION | VERIFIED_NOT_VALIDATED |
|---|---|---|---|---|---|---|
| REQUIREMENT | 74 | 66 | 2 | 5 | 0 | 1 |
| EXCLUSION | 24 | 24 | 0 | 0 | 0 | 0 |
| DECLARED_STATE | 48 | 36 | 0 | 0 | 12 | 0 |

(No ACCEPTANCE or REMAINING_WORK rows at package grain; no
IMPLEMENTED_UNMAPPED.) `SelectableUnderCurrentLoop=YES`: 0 rows.

## Package reading

The component/material/library data model is substantively implemented and
schema-verified (66/74 requirement rows ALIGNED, most re-executed at the
frozen SHA). Findings cluster in three places:

- **Overtaken review-disposition kit prose** (the package's dominant
  staleness species): DEL-03-04 and DEL-03-05 kits still declare PKG-02
  review findings "TECHNICALLY_ADDRESSED_PENDING_HUMAN / TBD" although the
  frozen `Review_Findings.csv` records Gate C ACCEPT_AS_IS/RESOLVED
  (2026-06-05) — encoded STALE_SETUP_SPECIFICATION on 7 DECL rows
  (AuthorityNeeded=REVIEW). DEL-03-08 carries the same species as an
  in-row note (single Procedure line; disposition ALIGNED).
- **Deferred adapter/round-trip and release-scope proofs**: DEL-03-05
  REQ-007/008/010 PARTIALLY_IMPLEMENTED, routed ENGINEERING (rigid-component
  behavior proofs, release benchmark/tolerance scope).
- **Verification-not-validation**: DEL-03-08 REQ-003 — hand-calc witnesses
  name `calculator.py` as governing `calculator_ref`, but no test executes
  the Python production path against the witness oracle values (unit suite
  re-derives closed forms independently); routed ENGINEERING. This is the
  package's one numeric-evidence gap.
- **DEL-03-06**: two ACCEPTED_DIVERGENCE rows on Gate-C-accepted taxonomy
  deferrals (addendum-11 bar met); 3 STALE DECL rows (OWNER).
- **DEL-03-01/02/03/07**: clean or near-clean; DEL-03-07's SECURITY rows
  (public/private separation, protected-content quarantine) carry the exact
  convention-6 encoding — Part C reviewer spot-check **PASS** (DEC-036
  named ruling governs).

Known R1 anomaly resolved: the DEL-03-03 / DEL-03-05 bare R01–R11
requirement-token collision is label-only (different requirement text);
neither deliverable's records cross-reference the other via bare tokens
(grep-verified by both pilots and the verifier); ClaimIDs disambiguate.
The all-ALIGNED DEL-03-03 vs mixed DEL-03-05 divergence is real kit-text
difference, not a misread (verifier-adjudicated).

## Decision findings (routing, not rulings)

- ENGINEERING: DEL-03-05 REQ-007/008/010; DEL-03-08 REQ-003 (production-path
  validation binding).
- REVIEW: DEL-03-04 DECL-002/003/004 + REQ-006; DEL-03-05 DECL-001..004;
  DEL-03-08 DECL-001/002 (overtaken review prose refresh).
- OWNER: DEL-03-06 DECL rows; DEL-03-07 SECURITY sufficiency (REQ-003/004);
  DEL-03-08 mill_tolerance kit refresh (its 2 STALE rows record the
  TP-PMM-P3-MILLTOL-001 slot absent from every kit doc).
- Named gate: none newly implicated beyond frozen register states.

## Unmapped implementation

None ledgered at material grain (third-party fixture writes inside mapped
crates recorded as hygiene notes).

## Conflicts and unknowns

None (zero AUTHORITY_CONFLICT / UNKNOWN rows). DEL-03-06 provenance note:
MEMORY-cited `RESOLUTION_MATRIX.csv` (DEV001 2026-05-16 review folder) is
absent from the frozen tree; no DecisionBasis relies on it (the A2
attestation pattern, correctly handled).

## Verification and repair record

DEL-03-04 DEFECTIVE (missed overtaken review prose) → fable re-run:
DECL-002/003/004 ALIGNED→STALE with Gate C run record as DecisionBasis,
REVIEW routing; zero disagreements (one line-span bookkeeping nit).
DEL-03-05 DEFECTIVE (convention-5 gate-column violation) → fable re-run:
19 cells (14× GateOrStageConstraint UNGATED→NONE_RECORDED, 3× RemainingSource,
2× SourceReliability EXC rows per addendum-6), dispositions untouched.
Owning-pilot string corrections on SOUND ledgers: DEL-03-02 and DEL-03-08
addendum-10 qualifier corrected (the "diff empty over … docs/" literal was
false — docs/AGENTIC_DEVELOPMENT_WORKFLOW.md and TYPES.md differ in the
range; both pilots re-ran the diff and corrected the string), DEL-03-08
DECL-004 overtaken-prose note added. Structural sweep + histogram recount
clean across all eight ledgers.

## Cross-ledger risks carried forward (W2–W5 / R3)

1. **Overtaken review-disposition kit prose** was handled three ways before
   repair (STALE / in-row note / silent miss) — W2–W5 pilots should encode
   it STALE on the affected declared-state surface rows with the Gate
   record as DecisionBasis (the W1-adjudicated form).
2. **Addendum-10 qualifier truncation**: pilots copying R1's content-identity
   claim must carry the docs-exclusion clause or re-run the diff over their
   exact cited paths (two W1 ledgers required correction).
3. Bootstrap `RecordedRemaining` transliteration (§§/en-dash) and DECL-005
   gate-cell annotation variants (see PKG-01 risk 1).
4. Forward-looking capability gaps encode as judgment splits
   (ALIGNED-at-contract-grain vs PARTIALLY_IMPLEMENTED) — histogram as
   judgment splits at R3, not drift.

## Fences

Discovery read-only; frozen tree clean throughout; no lifecycle/DAG/scope
change proposed as operative; no F-PIP-1..5 claim language outside
attributed quotes (all compliance-adjacent text is quoted exclusions or
disclaimers); all dispositions are agent judgments.
