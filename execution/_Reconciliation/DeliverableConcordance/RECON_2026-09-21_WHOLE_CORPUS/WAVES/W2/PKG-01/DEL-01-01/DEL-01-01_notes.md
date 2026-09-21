# DEL-01-01 notes — W2 PKG-01, worker G1

Forward ledger sealed (SHA-256 in `DEL-01-01_SEAL.txt`): 116 rows (68 required
keys, 34 `.rNN` rows for split blocks CLM-002/004/005/019/025, 14 `.sNN`
sub-claims). Reverse: 199 routed capabilities. DEL-01-01 is the only ISSUED
deliverable in the corpus. These are agent judgments, not owner rulings.

## Path aliases

- Deliverable folder: `projects/chirality-piping/execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-01_Project governance baseline/`. It contains spaces and commas, so its files (ScopeOfWork.md, _STATUS.md, _CONTEXT.md, MEMORY.md, `_run_records/*`) are cited only in `ContextRefs`; the validator rejects spaces in evidence columns. Review snapshots under `execution/_Reconciliation/Reviews/REV_DEL-01-01_*` carry the review evidence in `VerificationEvidence`.
- `NormativeSource` is `ScopeOfWork.md Lnn-Lmm` (or `_STATUS.md`, `_CONTEXT.md`, `MEMORY.md`) relative to that folder.
- Root AgentRuns paths (`execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/…`) are repository-root paths.
- `GATE:` tokens are not used in this ledger.

## Judgment calls

1. **ISSUED TBDs (ruling item 4).** Sixteen rows form `FG-DEL-01-01-01`, `LIFECYCLE_REASSESSMENT_REQUIRED`: the SOW's roster, quorum, release-authority, signing, human-authority and related TBDs, overtaken by DEC-027, DEC-057 and DEC-089. CLM-004.r05 (maturity labels and validation disclosure format) is in the group at MEDIUM confidence: validation wording was ruled by DEC-081 (BS-VALID), while BS-MATURITY was retired by DEC-105 with no replacement. The owner may read that slot as outside the group. Legal-review process (CLM-005.r06) is still open (DEC-079), so it is `ALIGNED`.
2. **MAINTAINERS.md lag (`FG-DEL-01-01-03`).** DEC-027 is recorded in the decision log and in RELEASE_QUALITY_GATES L183, but MAINTAINERS L35-L36 still say TBD and L44 forbids stating a final authority. REQ-01-01-05 and AC-01-01-04 are `PARTIALLY_IMPLEMENTED`, `LOCAL_DESIGN` under F8, with OPS-K-GOV-2 named. The repair is a bounded MAINTAINERS edit, not an ISSUED-text change. DEC-079 keeps "deliberate TBDs" in MAINTAINERS; I read that as covering the legal-instrument TBDs only (MEDIUM).
3. **Posture conflict (CLM-009.s01, `AUTHORITY_CONFLICT`).** PRD v0.4 L24 says "free and open-source". ScopeLedger SOW-001, DIRECTIVE L73 and the PolyForm Noncommercial license say source-available noncommercial. PRD L44's supersession sentence concerns the stress-analysis framing, not the license. The top-level README.md L10 says FOSS; docs/README.md L10 says source-available.
4. **AC-001 `ACCEPTED_DIVERGENCE` on DEC-081 (MEDIUM).** The frozen SOW differs from the RECON-I0-PKG01 parity production at six lines (L55, L149, L251, L268, L272, L317). All six are D-48 Wave 2 GF-TOKEN or PRD section 21.2 edits (commit 8fac6631a; its message says 82 live SOWs were aligned). The owner is asked to confirm DEC-081's reach to ISSUED text (the profile routes ISSUED changes through scope change; DEC-081 says it is not an SCA).
5. **Rename residue (`FG-DEL-01-01-02`).** This is recorded once on the SOW surface (CLM-009 L133). CANONICAL_DEPARTURE from CP-04: disposition `LIFECYCLE_REASSESSMENT_REQUIRED` under C6(d), because the ISSUED text was true at issuance and was overtaken by DEC-101 / SCA-010. Cause `RENAME_OR_IDENTITY` and AuthorityNeeded `OWNER` are kept so R3 can cluster it with the rename class.
6. **Pins and four-document residue in ISSUED text.** Pins use CP-02 and four-document residue uses CP-01, with BaselineClass `ISSUED` and AuthorityNeeded `OWNER`, because any edit goes through the ISSUED change path. These are CANONICAL_DEPARTUREs on non-checked fields (`FG-DEL-01-01-05`, `-06`). I treated pins as stale pointers rather than as C6(d) lifecycle items, because they are metadata, not substance.
7. **Run-record protected-content confirmation (`FG-DEL-01-01-04`).** AC-01-01-05 and CLM-019 are `PARTIALLY_IMPLEMENTED`. The license/ISSUED closeout record lists its edited files but gives no protected-content statement. Own reading of those edits finds license-notice text only, so the tier is `LOCAL_DESIGN` (F8, IP boundary named).
8. **MEMORY** is `STALE_REVIEW_OR_EVIDENCE` · `RECORD_DRIFT` (MEDIUM). The 2026-06-03 refresh entry names DAG-006, but its run record names DAG-005. The token was rewritten by commit 11c5497f6.
9. **Parity discovery.** EVIDENCE_MAP lists NONE_FOUND for DEL-01-01. The A3 `git grep` found RECON-I0-PKG01 (RETURN.md, reproduced-production.md), so VER-001 and matrix OUT-001 are CP-09 `EVIDENCE_OVERTAKEN`, as the R0 repair requires.

## R0 repairs carried

- OUT-001 is consistent with AC-001: parity does not bind the post-DEC-081 bytes, so the matrix OUT-001 and VER-001 rows are `STALE_REVIEW_OR_EVIDENCE`.
- The DEC-027/057 rows are one FindingGroup with `LIFECYCLE_REASSESSMENT_REQUIRED` (decision 4).
- BaselineClass is `NONE` on the `_CONTEXT.md`, `_STATUS.md` and MAINTAINERS.md rows (MAINTAINERS.md is draft, with no baseline). It is `ISSUED` only on rows whose diverging artifact is the ISSUED SOW.
- The rename is now a finding (ruling item 3). Cause tags and tiers are remapped to the bound vocabulary.

## Canonical departures

- SOW surface: CP-04 disposition, as in judgment call 5.
- CP-02 and CP-01 rows: BaselineClass and AuthorityNeeded, as in judgment call 6.
- CS rows follow their assignments with no departure. CS-04 has `.s01` (PKG-00 status) and `.s02` (still-TBD list).

## Convention friction

- Evidence columns cannot hold the deliverable folder, because of the spaces in its path, so deliverable-local evidence goes in `ContextRefs`. A path-alias token would help.
- F3 and CP-04 disagree for rename text first present at the initial migration. I followed CP-04 (a pattern entry) in the non-ISSUED ledgers.
- C1 asks for `.sNN` sub-claims "whenever parts of a block would take different dispositions". Where one element of a block was overtaken and the rest held, I assessed the block directly with the gap's disposition (CLM-006, CLM-014, CLM-023). I split only where the parts carried different findings.
- The claims lint (GEN-13) excludes `governance/` and top-level files, so MAINTAINERS.md and LICENSE.md claims were checked by reading.

## UNKNOWN rows

None.

## Reverse pass

The reverse pass did not change my view of any sealed row. It confirmed ownership of MAINTAINERS.md and LICENSE.md (`CLAIMED_BY`), and partial ownership of docs/README.md, CONTRACT.md and DIRECTIVE.md (registered anticipated artifacts whose content extends beyond this deliverable). The routing note on RC-01-0006 (the top-level README diverges from docs/README.md) matches CLM-009.s01. All F5 overlaps have capability-specific reasons.

## Batch consistency

PASS, 0 findings, over the four PKG-01 forward ledgers.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9). Piping selects work through owner-steered work graphs.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
