# DEL-05-04 Analysis status semantics — worker notes (W3, PKG-05, worker G2)

Forward ledger `DEL-05-04_forward.csv`: 91 rows (88 required keys and 3
sub-claims). Sealed at SHA-256
`1dd3c26825f7632d6b52b096070027334fed1bdef58fda2a20c110ad80db7255`.
Reverse file `DEL-05-04_reverse.csv`: 419 capabilities (2 CLAIMED_BY,
22 COVERS, 395 NOT_MINE).

## Path aliases

- `docs/…` project documents are cited as `projects/chirality-piping/docs/…`.
  The architecture note `docs/architecture/analysis_status_semantics.md` exists
  only in the project copy.
- The SOW-stage-2 parity records live at the repository root
  (`execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/…`), not
  under the project.
- The deliverable-local evidence paths contain spaces and commas, and are
  cited as full `projects/chirality-piping/execution/PKG-05_…/…` tokens.
- Deliverable-local `Datasheet.md`, `Specification.md`, `Guidance.md` and
  `Procedure.md` are absent at the freeze; `ScopeOfWork.md` replaces them.

## Judgment calls

1. **FG-DEL-05-04-01: stale human-acceptance reuse (9 rows).** These rows
   are PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · INVARIANT · CLAIMS · OWNER:
   REQ-05-04-008, REQ-05-04-014, their CLM-013 verification rows, AC-001,
   CLM-004, CLM-020, and the D-41 declaration pair CLM-008/CLM-023.
   - What exists: a hash binding at declaration level only
     (`HumanAcceptanceRecord.bound_hashes`; persistence
     `HumanAcceptanceRef.invalidates_on_hash_change` const true).
   - What does not exist: a product workflow that creates acceptance records,
     a runtime check, or a negative test proving that a record stops applying
     after a bound hash changes.
   - Why it is not ALIGNED: the "does not survive content changes" element
     holds only because no records exist (CP-11).
   - The 2026-08-20 R9 tranche is the one I read as not closing this
     element. `_STATUS.md` and `MEMORY.md` say it closed "the exact PDU-037
     stale-hash runtime residual". R9 added a claimed-model-hash gate to the
     operation applier, which checks model-operation claims, not human
     acceptance records.
   - The verifier should check this reading: it decides nine rows. If the
     owner treats R9 as the intended closure, these rows would become
     accepted scope questions instead.
   - Tier INVARIANT (OPS-K-AUTH-2). Layer CLAIMS is the nearest C5 layer for
     the professional-reliance boundary. AuthorityNeeded OWNER, because
     OI-007 (the human approval workflow) is still TBD.
2. **The D-41 declarations (CLM-008, CLM-023) are PARTIALLY_IMPLEMENTED.**
   C6(b) disposes the declared subject, and part of that subject (the held
   stale-hash negative) is unimplemented. The declaration itself is accurate.
3. **`STATUS#remaining/R01` "None." is REMAINING_STATE_MISMATCH**
   (RECORD_DRIFT, MEDIUM confidence). It says nothing remains, while the
   SOW's own current declaration records a held element. Four-document
   residue is also open.
4. **Product callers.** The status vocabulary has product callers:
   - `product_physics` StatusEnvelope (mechanics / rule_check /
     professional_acceptance = NOT_PROVIDED);
   - the desktop rule-check command;
   - `rule_check_runner`, which downgrades any HumanApprovedForProject to
     HumanReviewRequired.

   F7 markers are therefore not needed on REQ rows that cite these. The
   `schemas/analysis_status.schema.yaml` envelope has no product validator,
   but every row citing it also cites a product surface.
5. **Rename residue (SOW.s01, CP-04 default variant).** The SOW text does not
   carry the former name. The schema it names does (`$id`
   `https://openpipestress.org/…`, title), and so does the architecture
   note's prose. The OPS-K invariant IDs are live identifiers in the project
   CONTRACT; I did not treat them as residue.
6. **REQ-05-04-010 is ALIGNED.** I read the DEC-025 five-surface sweep as the
   current gate that runs the non-collapse tests. I read the requirement's
   "future release gates" as covered by that gate for the collapse subject.
   The stale-reuse negative is REQ-014's subject.
7. **REQ-05-04-011 and CLM-013/REQ-011 are ALIGNED at MEDIUM confidence.**
   The evidence:
   - my inspection of the artifacts;
   - the 2026-06-05 QA_Report focused protected-content scan (the schema has
     not changed since);
   - the gated DEC-081 claims-language validator.

   The architecture note changed later, but only in the DEC-081 claims pass.
8. **`CONTEXT#architecture-basis-injection` sub-claims.**
   - `.s01` (PKG-00 SEMANTIC_READY) follows the W2 Agent 0 reading, with
     cause SCOPE_REDIRECTED_BY_RULING (SR-1 contested cluster).
   - `.s02` (Still TBD items since ruled: DEC-022/023/025/028/059/060) is
     STALE_SETUP_SPECIFICATION by the F3 origin test (`7bee9ae41`).
9. **F3 origin tests used.**
   - CLM-021 "Four-document kit exists": `7bee9ae41`, so
     STALE_SETUP_SPECIFICATION.
   - CLM-014 and CLM-019 four-document text: `cbb7c42fa` (2026-06-04), so
     STALE_REVIEW_OR_EVIDENCE.
   - CLM-018 revision 0.7 pin: `1f4de36d9` (2026-06-03); pins stay
     STALE_REVIEW_OR_EVIDENCE.
10. **CP-09 applied to VER-001** as well as the output-matrix OUT-001 (the
    contested cluster named in the W2 assessment). I judged the purpose-block
    OUT-001 on its substance: ALIGNED.

## Canonical departures

None. All seven CS rows take their assigned fields.

## Convention friction

- There is no C5 layer named for the professional boundary. CLAIMS was used
  for OPS-K-AUTH-2 gaps; BASELINE was the alternative.
- The SOW SURFACE row carries three common defects: the frontmatter pin, CP-04
  and four-document residue. I recorded the pin on SURFACE and CP-04 on
  `SOW.s01`. Four-document residue is block-specific, not common to every
  item, so it stays on the blocks.

## UNKNOWN rows

None.

## Reverse pass

The reverse pass did not change my view of any sealed row.
- Capabilities 0034 (load-case algebra rejects human-approval status), 0225
  and 0326 (MODEL_INCOMPLETE paths), and 0327 and 0360 (declaration-level
  hash invalidation) are consistent with the sealed dispositions.
- F5: every NOT_MINE capability whose EntryPoints hit a cited path
  (product_physics, solver diagnostics, the src-tauri lib, CONTRACT,
  registers) has a capability-specific reason.

## Batch consistency

`--batch` over both forward ledgers: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9). Piping
selects work through owner-steered work graphs.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). Agent
dispositions here are not owner rulings.
