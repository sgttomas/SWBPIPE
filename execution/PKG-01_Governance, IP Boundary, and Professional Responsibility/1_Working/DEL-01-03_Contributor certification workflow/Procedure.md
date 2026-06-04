# Procedure: DEL-01-03 Contributor certification workflow

## Purpose

Define a repeatable local procedure for contributor certification intake, provenance review, protected-content screening, quarantine/rejection handling, and disposition records for public data contributions.

## Prerequisites

- Assigned deliverable context: DEL-01-03 under PKG-01.
- Current governing references: `docs/CONTRACT.md`, `docs/IP_AND_DATA_BOUNDARY.md`, `docs/DIRECTIVE.md`, `docs/SPEC.md`, `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7, and approved `DAG-006`.
- Draft repo-level workflow surfaces exist at `CONTRIBUTING.md`, `governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md`, and `governance/CONTRIBUTION_REVIEW_CHECKLIST.md`.
- Human-owned governance decisions for exact license, final contributor legal mechanism, maintainer authority, reviewer role, release authority, and legal-review thresholds are `TBD`.
- No protected standards, proprietary vendor data, or private project/rule-pack content may be copied into the public workflow records.

## Steps

1. Receive contribution package.
   - Record contributor identity and contribution description.
   - Assign an intake record ID using the future project convention, or `TBD` if no convention exists.

2. Collect required provenance fields.
   - `source_name`
   - `source_location`
   - `source_license`
   - `contributor`
   - `contributor_certification`
   - `redistribution_status`
   - `review_status`

3. Check completeness.
   - If source, license, contributor certification, or redistribution status is missing, set review status to `pending` or `rejected` with a `TBD` note.
   - Do not infer redistribution rights from usefulness or public availability.

4. Screen protected and private content.
   - Look for protected standards text, tables, figures, examples, copied formulas, material allowables, SIF/flexibility tables, protected dimensional tables, proprietary vendor catalog data, commercial software examples/templates, private project data, private rule packs, owner standards, or company design bases.
   - If suspected, stop intake and set `redistribution_status=protected_suspected` or equivalent note.

5. Quarantine suspected protected/private submissions.
   - Do not reproduce the suspected content in public notes.
   - Move or reference the artifact only through `quarantine/protected-content/` or a maintainer-approved equivalent.
   - Treat quarantine access rule, escalation owner, and final legal disposition as `TBD` until human/legal authority records them.
   - Record issue metadata, not protected content.
   - Request human/legal review.

6. Route review.
   - Provenance-complete and public-permissive contributions proceed to maintainer review.
   - Unknown, private-only, or protected-suspected contributions do not enter public examples or libraries.
   - Legal or license uncertainty remains `TBD` pending human/legal decision.

7. Record disposition.
   - Use one of: `pending`, `accepted`, `rejected`, `quarantined`.
   - Include reviewer, date, evidence reviewed, rationale, and limitations.
   - State that disposition is repository governance review only and not engineering approval.

8. Maintain repo-level workflow evidence.
   - Treat `CONTRIBUTING.md` and `governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md` as draft governance surfaces until human acceptance.
   - Edit repo-level contributor artifacts only under an explicitly approved write scope.
   - Carry the selected project license as `PolyForm-Noncommercial-1.0.0`; keep exact contributor legal mechanism, legal sufficiency, and reviewer/release authority `TBD` until the human project authority records them.

## Verification

| Check | Pass condition |
|---|---|
| Field completeness | All required provenance and certification fields are present or explicitly `TBD`. |
| Protected-content stop rule | Suspected protected/private content is not reproduced and is routed to quarantine/human review. |
| Authority boundary | Records do not claim certification, sealing, legal clearance, code compliance, or professional approval. |
| Contributor-governance uncertainty | Contributor legal mechanism, legal sufficiency, and maintainer/release authority unresolved items remain `TBD`; selected project license remains recorded as `PolyForm-Noncommercial-1.0.0`. |
| Local write scope | This current-basis refresh edits only DEL-01-03 local artifacts; repo-level artifacts are read as evidence unless separately approved for editing. |

## Records

- Contributor certification intake record.
- Provenance field checklist.
- Protected/private content screen.
- Quarantine issue record when applicable.
- Reviewer disposition record.
- Human/legal ruling reference when provided.
- `CONTRIBUTING.md` and contributor certification template draft pointers.
