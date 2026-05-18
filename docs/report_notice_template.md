---
doc_id: OPS-REPORT-NOTICE-TEMPLATE
doc_kind: report.notice_template
status: draft
created: 2026-05-01
deliverable_id: DEL-01-04
refs:
  - rel: governed_by
    to: OPS-PROFESSIONAL-BOUNDARY
  - rel: governed_by
    to: OPS-IP-DATA-BOUNDARY
---

# Report Notice Template

This template defines baseline notice language for OpenPipeStress-generated
reports and public report-template examples. Report generators may format this
content for the target output, but the professional-boundary meaning must be
preserved.

## Required Notice

```text
OpenPipeStress is decision-support software for transparent piping mechanics
analysis. This report may include mechanical results, diagnostics, warnings,
assumptions, provenance notes, software versions, model hashes, rule-pack
references, and user-rule check results.

Mechanical solve results are not professional engineering approval. User-rule
check results are computed from user-supplied rule packs and user-supplied
data; they are not an automatic code-compliance determination.

Before professional reliance, a competent human reviewer must review the model,
inputs, units, load basis, private libraries, rule-pack basis, source and
provenance records, warnings, assumptions, limitations, and report contents for
the specific project use.

OpenPipeStress does not provide legal advice, standards-body approval,
professional engineering certification, sealing, endorsement, authentication,
or project-specific code-compliance approval.
```

## Report Metadata Slots

Generated reports should include these fields or equivalent structured
metadata:

| Field | Requirement |
|---|---|
| Software version | OpenPipeStress version or commit basis used to generate the report. |
| Model hash | Hash of the model basis used for the reported results. |
| Input manifest hash | Hash or reference for the input manifest where implemented. |
| Rule-pack reference | Rule-pack ID, version, checksum, source note, and public/private marker when rule checks are included. |
| Report hash | Hash or stable identifier for the rendered report where implemented. |
| Units basis | Recorded unit system and unit-conversion basis. |
| Warnings and assumptions | All active warnings, missing-input findings, assumptions, and limitations relevant to review. |
| Provenance notes | Source/provenance/license or private-source notes for user-supplied data where engineering reliance may be affected. |
| Human acceptance reference | Optional external human-review record, if one exists, bound to the applicable hashes. |

## Optional Human Acceptance Notice

Use this section only when an external human acceptance record exists. Do not
populate it automatically from solver or rule-pack output.

```text
Human acceptance reference: <TBD external record ID or location>
Accepted scope: <TBD scope statement>
Accepted by: <TBD human authority>
Acceptance date: <TBD date>
Bound model hash: <TBD hash>
Bound rule-pack checksum: <TBD checksum or not applicable>
Bound report hash: <TBD hash>

This acceptance reference applies only to the bound model, rule-pack, report,
input manifest, software basis, assumptions, warnings, and limitations recorded
above. Changes to those bound items require re-review before reliance.
```

## Protected-Content Notice

Public report templates and public examples must not include protected
standards text, protected tables, protected examples, proprietary engineering
values, private project data, private owner standards, private rule packs, or
proprietary component libraries.

Private project reports may reference user-controlled private material when
the user intentionally includes it in a private workflow. That private use does
not make the material suitable for public repository fixtures, public examples,
or public documentation.

## Review Checklist

Before publication or release of a report template, verify:

| Check | Pass condition |
|---|---|
| Boundary notice | Required notice is present or preserved in equivalent wording. |
| Status separation | Mechanics solve, user-rule check, warnings, and human acceptance are separate. |
| Hash binding | Human acceptance reference, if present, binds to model, rule-pack, report, and input basis. |
| Protected content | No protected standards text, tables, examples, copied formulas, or proprietary values appear in public template text. |
| Private data | Public examples omit private project, owner-standard, material-library, component-library, and rule-pack data. |
| Claims | The report does not state or imply certification, sealing, endorsement, automatic approval, or code compliance for reliance. |
