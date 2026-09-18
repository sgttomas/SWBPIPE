# SCA-010 Brief — PRD name amendment: the product is SWBPIPE

**Amendment ID:** SCA-010
**Date:** 2026-09-18
**Status:** PROPOSED — prepared for the owner's separate acceptance. Nothing in `docs/PRD.md` changes, and `execution/_ScopeChange/_LATEST.md` is not moved, before that acceptance.
**Authority:** D-71 item 3, ruled by the owner on 2026-09-18 as option B with amendment (`execution/_Coordination/_DECISIONS/D-71_RULING_2026-09-18.md`), codified as `DEC-101` in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12, which expressly leaves the PRD-level act to this bundle.
**Precedent and form:** the SCA-007 bundle (`execution/_ScopeChange/SCA-007_2026-07-16_2026/`): brief, impact assessment, amendment preview, amendment actions.

## Scope

1. Amend the adopted PRD v0.3 to v0.4 by replacing the product name "OpenPipeStress" with "SWBPIPE" at its 24 occurrences, including the §19.3 required report notice and its appendix copy, retitling the document, and adding one header line recording the former name. No requirement, section number, claim, non-goal or notice wording other than the name changes.
2. Propagate the same replacement into the notice's template home, `docs/report_notice_template.md` (three occurrences).
3. Record the amendment: `execution/_ScopeChange/_LATEST.md` moved to SCA-010 at acceptance; PRD header amended with the date and authority.

## Non-scope

No scope item, package, deliverable, dependency, lifecycle, stage or release change, and therefore no decomposition revision advance and no DAG rebuild (recorded in `Impact_Assessment.md`). No change to the §19.3 notice beyond its first word: it keeps every clause, including "does not certify, seal, approve, authenticate, or determine code compliance". No change to PRD §5.9, §21.1 to §21.3 or the boundary statement at the head of the PRD. No product, schema, registry or packaging change: those are `DEC-100` to `DEC-103` work for an owner-authorized implementation tranche. No edit to ruled history: earlier records keep the former name permanently.

## Acceptance basis

None yet. The owner ruled the rename itself; the governing instructions for the design program require the PRD text change to be accepted separately. The owner accepts, amends or declines this bundle in session; acceptance is then recorded in an `ACCEPTANCE_RECORD.md` added to this snapshot, and the actions are executed by whichever loop the owner names.

## Execution form

Docs-only. One branch, one PR, the claims-language lint (its `MISSING_PRD_NOTICE` anchor matches a fragment that does not contain the name, so it is unaffected), the path-anchor check and the owning loop's self-check. No multi-agent run is warranted for a mechanical replacement of 27 occurrences in two files.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
