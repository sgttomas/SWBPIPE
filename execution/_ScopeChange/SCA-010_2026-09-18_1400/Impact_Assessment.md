# SCA-010 Impact Assessment

Compiled by the design program's ROOT on 2026-09-18 from RESEARCH-F §3 (the rename inventory behind D-71 item 3, retained in the run `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM/`) and ROOT's own search of `docs/PRD.md` and `docs/report_notice_template.md` at `0d0021db71022a615f6ac162f273c03695c7beab`.

## Live surfaces requiring edits (covered by A001 to A006)

- `docs/PRD.md`: title (line 1), working title (line 3), and 22 body occurrences, among them the §19.3 notice (line 1249) and its appendix copy (line 1786). **One occurrence is excepted:** line 15 carries the name only inside the path `docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md`, a live pointer to a preserved historical file that exists under that name; it is not renamed, since ruled history keeps its original tokens. Line 43 refers to the product's own earlier framing and takes the new name like the rest. So 23 of the 24 occurrences are replaced: title, working title, two notice copies and 19 others. A001 to A003. (Corrected after independent review REVIEW-02, which found that the first wording cleared all 24 and would have broken that path.)
- `docs/PRD.md` header: version 0.3 to 0.4, an "Amended" date, the authority line extended with D-71, `DEC-101` and SCA-010, and one new line "Formerly: OpenPipeStress". A004.
- `docs/report_notice_template.md`: lines 16, 41 and 68. A005.
- `execution/_ScopeChange/_LATEST.md`: moved to SCA-010 at acceptance. A006.

## Verified no-change (recorded)

- The claims lint's anchors `MISSING_PRD_NOTICE` and `MISSING_RENDERER_NOTICE` match fragments of the notice that do not contain the name (RESEARCH-F §3.2). A007.
- `execution/_Decomposition/SOFTWARE_DECOMP.md`: no scope, package or deliverable row changes, so revision 0.12 stands and the architecture-basis validator's revision pin is untouched. A008.
- The approved dependency graph named by `execution/_DAG/_LATEST.md`: no edge or node changes. A009.
- Citations of `docs/PRD.md` by section number throughout the project: section numbers do not change. A010.

## Outside this amendment, owned elsewhere

- The report renderer's emitted notice, every product string, `BS-IP`'s registry text, packaging names, the bundle identifier, the schema namespace and document-kind constants: `DEC-101`, executed by an implementation tranche, which must land the renderer's notice in step with the PRD text so the two never disagree in a released build.
- The other governance documents (`docs/CONTRACT.md`, `docs/PROFESSIONAL_BOUNDARY.md`, `docs/IP_AND_DATA_BOUNDARY.md` and the rest): they take the name at their next amendment, per the ruling; no bulk rewrite.
- Project-loop notices: RESEARCH-F found no other project loop pinning the PRD's name text; the repository's root documents that name the project directory are unaffected because the directory `projects/chirality-piping` is not renamed.

## Risk

Low. The one ordering risk is a build in which the PRD notice and the renderer's notice differ in their first word; it is removed by accepting this bundle when the implementation tranche that executes `DEC-101` is authorized, and executing both in that tranche.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
