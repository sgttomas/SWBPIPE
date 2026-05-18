# Procedure: DEL-01-03 Contributor certification workflow

## Purpose

Define a repeatable local procedure for contributor certification intake, provenance review, protected-content screening, quarantine/rejection handling, and disposition records for public data contributions.

## Prerequisites

- Assigned deliverable context: DEL-01-03 under PKG-01.
- Current governing references: `docs/CONTRACT.md`, `docs/IP_AND_DATA_BOUNDARY.md`, `docs/DIRECTIVE.md`, `docs/SPEC.md`, and `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4.
- Human-owned governance decisions for exact license, maintainer authority, release authority, and legal-review thresholds are `TBD`.
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
   - Move or reference the artifact only through the future approved quarantine path.
   - Treat the quarantine path and access rule as `TBD` until human/legal authority records them.
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

8. Prepare future repo-level update.
   - Convert the accepted local workflow into a future `CONTRIBUTING.md` section only after human approval.
   - Keep exact legal mechanism and license language `TBD` until the human project authority records it.

## Verification

| Check | Pass condition |
|---|---|
| Field completeness | All required provenance and certification fields are present or explicitly `TBD`. |
| Protected-content stop rule | Suspected protected/private content is not reproduced and is routed to quarantine/human review. |
| Authority boundary | Records do not claim certification, sealing, legal clearance, code compliance, or professional approval. |
| License/governance uncertainty | License and maintainer/release authority unresolved items remain `TBD`. |
| Local write scope | No repo-level artifacts are edited by this setup workflow. |

## Records

- Contributor certification intake record.
- Provenance field checklist.
- Protected/private content screen.
- Quarantine issue record when applicable.
- Reviewer disposition record.
- Human/legal ruling reference when provided.
- Future `CONTRIBUTING.md` section draft pointer, if approved later.
