# DEL-11-01 calibration notes (R0)

Deliverable: DEL-11-01 User guide skeleton (PKG-11, DOC_UPDATE). Frozen state
`00115c71931bcae79909602d653740d3bb72dfa1`. Forward ledger sealed at
`2b5f98931ff41f9caed1575cdb67e2b97b1626ceae2b03347ebcd1bac0173907` (92 rows:
66 issued keys plus 26 `.sNN` sub-claims). These are calibration findings, not
owner rulings. Standard claim fence applies (F-PIP-2; claims taxonomy per
DEC-081).

## Path aliases

- `NormativeSource` uses short forms `ScopeOfWork.md#Lnn`, `_STATUS.md#Lnn`,
  `_CONTEXT.md#Lnn`, `MEMORY.md#Lnn`, all inside
  `projects/chirality-piping/execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-01_User guide skeleton/`.
  The evidence columns use full repository-relative paths.
- `GATE_EVIDENCE/...` tokens are relative to the run folder
  `RECON_2026-09-21_WHOLE_CORPUS/`. They are not in the frozen tree, so the
  validator does not path-check them. `GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json`
  stands for suite-level PASS of `pytest -q tests`. `GATE_EVIDENCE/PR834_CI/harness-run.json`
  stands for the governance-harness CI pass.
- "the guide" means `projects/chirality-piping/docs/user_guide/index.md`.
- `R18` means the owner adoption record
  `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260725-CANDIDATE-BRIEFS-R18/instances/WI-PKG11-DEL1101-EXECUTION/OWNER_ADOPTION.md`
  and its 2026-07-25 run record.
- `SOFTWARE_DECOMP.md` in `DecisionBasis` means
  `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md` (revision 0.12).
- The tools `tools/validation/check_four_documents.sh`, `validate_enum.py` and
  `validate_dependencies_schema.py`, cited by the SOW with no prefix, resolve at
  the repository root.

## Judgment calls

- **Guide read-only exclusion (CLM-004.s02, CLM-010.s02, CLM-014, CLM-017,
  CLM-018).** I marked these `STALE_SETUP_SPECIFICATION` / `DOC_BEHIND_CODE`,
  not `ACCEPTED_DIVERGENCE`. The Deliverables register (governing) names the
  guide as this deliverable's artifact. The R18 owner adoption granted a
  guide-only exception and deliberately left the SOW unedited, but A3 makes
  that record context only. DEC-107 (governing) directed guide edits but is
  scoped to the acceptance-text withdrawal.
- **Conditional TBD clauses (UG-REQ-002, CLM-024).** "TBD where not yet
  decided" is read as conditional, so these are `ALIGNED`: the guide states the
  items decided since (DEC-022, DEC-023, DEC-057, SCA-003, SCA-004) as
  decisions. The unconditional list in CLM-004.s08 and the CONTEXT
  architecture block are stale.
- **CLM-025 "accepted implementation deliverable".** I recorded this as
  `IMPLEMENTED_DIFFERENTLY` with LOW confidence. It only diverges if
  "accepted" means lifecycle acceptance: the guide describes renderer, preview,
  desktop shell and save as existing, while their deliverables are IN_PROGRESS.
- **Product rename.** The SOW's OpenPipeStress mentions are recorded once, at
  the SOW surface row. Blocks whose substance is otherwise correct stay
  `ALIGNED` with a note, so the rename is not repeated across six rows.
- **CLM-013.s02 vs CLM-020.s02.** The first names SOW *sections* that still
  exist and is `ALIGNED`. The second names *files* (`Datasheet.md`,
  `Specification.md`) that are gone and is stale.
- **STATUS R01.** `ALIGNED`: its text, scoped to REM-001, matches the evidence
  (A4 declared-state audit). It is not evidence that no work remains.
- **MEMORY.** Dated entries are treated as history. The undated PDU-055
  "current declaration" header in MEMORY is overtaken, so the single MEMORY row
  is `STALE_REVIEW_OR_EVIDENCE`.
- **Selectability.** `NOT_APPLICABLE` on every row. Since 2026-09-19, work is
  selected through owner-steered work graphs, not `## Remaining` (C9).
- **Source reliability.** `NOT_APPLICABLE` on every row: a documentation
  deliverable with no engineering-source claims. `ValidationEvidence` is also
  `NOT_APPLICABLE` throughout. No row needed `VERIFIED_NOT_VALIDATED`.

## Convention friction

1. **Owner adoption records vs A3.** An AgentRuns owner-adoption record that
   quotes the owner verbatim and grants a scoped exception carries the owner's
   direct words. Still, A3 classes it as context and C6 withholds
   `ACCEPTED_DIVERGENCE` from it. The row reads as an unresolved divergence
   even though the owner has already decided it. Smallest fix: allow
   `ACCEPTED_DIVERGENCE` when a verbatim, hash-bound owner decision covers the
   exact divergence, flagged `OWNER_DIRECTION_RECORD` in `Notes`. Or add a
   `BaselineClass`/`Notes` flag that routes such rows to confirmation rather
   than decision at R4.
2. **Section-heading BLOCK keys do not parent their CLM blocks.** The four
   ontology/epistemology/praxeology/axiology keys span CLM blocks, but those
   blocks are issued with `ParentKey=SOW`. C1's `CONTAINER` rule says "fully
   carried by their ITEM children", which does not fit literally. Smallest fix:
   the extractor sets `ParentKey` of nested CLM blocks to the section block, or
   C1 adds "or by nested BLOCK keys".
3. **No disposition for declared-state metadata drift.** A stale "Last
   Updated", out-of-order history, or a revision pointer that is merely old fit
   none of the dispositions well. I used `STALE_REVIEW_OR_EVIDENCE` plus
   `RECORD_DRIFT` or `EVIDENCE_OVERTAKEN`. Smallest fix: state in C6 that
   `STALE_REVIEW_OR_EVIDENCE` covers declared-state metadata drift, or add
   `RECORD_DRIFT` guidance to it.
4. **Gate-evidence citation form.** The schema asks for repository-relative
   paths, but `GATE_EVIDENCE/` is not in the frozen tree. I cited run-relative
   tokens. Smallest fix: name `GATE_EVIDENCE/...` as an allowed run-relative
   token form in Part D.
5. **Tier for claims-boundary guidance.** CLM-025 is local guidance, but its
   subject is the BS-MATURITY claims boundary (layer 1). It is unclear whether
   that tier is `LOCAL_DESIGN` or `INVARIANT`. Smallest fix: C3 says the tier
   follows the claim's authority, not its subject, and the `CLAIMS` layer
   carries the subject.
6. **HISTORY rows.** The conventions exempt MEMORY dated entries from
   staleness but say nothing about `STATUS#history`, and nothing about undated
   current declarations embedded in history surfaces. Smallest fix: extend the
   MEMORY rule to `STATUS#history`, and say that undated current declarations
   inside a history surface are assessed as declared state.
7. **Duplicate claims inside one SOW.** CLM-002, CLM-009 and CLM-016 are the
   same declaration, and CLM-013 and CLM-020 largely duplicate each other, so
   the same finding is counted up to three times. Smallest fix: allow a
   `DUPLICATE_OF <key>` note so synthesis de-duplicates without a new
   disposition.

## Unit grain

For this SOW deliverable the grain was mostly right. The UG-REQ items were
well-sized, and most CLM blocks were one claim. It was too coarse for tables
without numbered IDs: CLM-004 (8 attributes), CLM-006 (outline plus implicit
completeness), CLM-010 (three distinct scope and exclusion statements), and
CLM-013/020 (six checks each). These needed `.sNN` splits because their parts
got different dispositions. It was too fine for the three identical PDU-055
declarations and the four section-heading blocks. CONTEXT's eleven blocks
added little: eight are identity restatements that were trivially `ALIGNED`.
The deliverable's real divergence (setup-era SOW vs authored guide) sits at
the surface grain and spreads across about ten rows.

## Smallest next check for UNKNOWN rows

No row is `UNKNOWN`. Two `PARTIALLY_IMPLEMENTED` rows (VER-001 and the output
matrix OUT-001) depend on a negative search. Smallest next check: search the
2026-07-14 contract-replacement tranche (commits `4756e403e`..`f074936e9`) and
its run records for a DEL-11-01 source-parity report. If one exists, those rows
move toward `ALIGNED`, except for accessibility.

## Effect of the reverse pass

All 96 pilot capabilities are `NOT_MINE`. They are solver, security, adapter,
model-tree and workspace code, and DEL-11-01 is documentation only. The
reverse pass did not change my view of any sealed row. Two observations:

- The pilot inventory has no documentation capabilities, so the reverse pass
  could not independently test ownership of `docs/user_guide/index.md`, the
  one artifact this deliverable owns. R3 should include documentation surfaces
  in the reverse inventory, or DOC_UPDATE deliverables get no reverse signal.
- CAP-WS-008 (accessible disabled-control reasons) shows that accessibility
  work exists in the product. That supports the sealed AC-001 finding: the
  contract names accessibility intent but has no content for it. The gap is in
  the DEL-11-01 contract and guide, not in product code.
