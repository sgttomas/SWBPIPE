---
doc_id: OPS-IP-DATA-BOUNDARY
doc_kind: governance.policy
status: draft
created: 2026-04-30
refs:
  - rel: governed_by
    to: OPS-CONTRACT
---

# IP and Data Boundary Policy

## 1. Purpose

This policy keeps the public OpenPipeStress repository aligned with the project intent: public open mechanics, private/user-supplied code and proprietary data.

This policy is a project governance control, not legal advice, professional engineering approval, certification, sealing, standards-body endorsement, or a code-compliance claim. Human/legal review remains required when redistribution rights or protected-content status are uncertain.

## 2. Public repository may contain

- solver algorithms based on open mechanics;
- geometry, unit, load, stress-recovery, and report schemas;
- blank templates and private-library import mechanisms;
- invented example data with clear notices;
- public-domain or permissively licensed data with documented provenance;
- validation benchmarks from original, public-domain, or permissively licensed sources;
- review checklists, manifests, and empty schema slots that help users record private/code-specific data without bundling those values publicly.

## 3. Public repository must not contain

- ASME or other standards-body code text, tables, figures, examples, or commentary;
- material allowable tables copied from standards;
- B31J stress-intensification or flexibility-factor content copied from standards;
- B16/B36/MSS/ISO/EN/CSA dimensional or rating tables copied from standards;
- proprietary vendor catalogs without redistribution rights;
- commercial software examples, report templates, or benchmark files without permission;
- user private rule packs, owner standards, or company design bases;
- code-specific load combinations, allowables, SIFs, flexibility factors, acceptance criteria, or owner requirements unless they are invented examples or otherwise cleared for public redistribution.

## 4. Required provenance fields

Every public data record must include:

| Field | Meaning |
|---|---|
| `source_name` | Human-readable source. |
| `source_location` | URL/path/document identifier or `TBD`. |
| `source_license` | Redistribution basis or `TBD`. |
| `contributor` | Person/org submitting. |
| `contributor_certification` | Statement that data is not copied from protected source unless licensed. |
| `redistribution_status` | `public_permissive`, `private_only`, `unknown`, `protected_suspected`. |
| `review_status` | `pending`, `accepted`, `rejected`, `quarantined`. |

Records with `source_license = TBD`, `redistribution_status = unknown`, or missing contributor certification are not acceptable as public data. They remain pending, private-only, rejected, or quarantined until a maintainer records a review disposition.

## 5. Quarantine rule

If data appears to derive from a protected standard or proprietary source, agents and maintainers must:

1. stop ingestion;
2. mark the record `protected_suspected`;
3. move the artifact to `quarantine/protected-content/` or another maintainer-approved non-example quarantine path;
4. record the issue;
5. request human/legal review.

Quarantined material must not be used as public fixture data, copied into documentation, summarized into public tables, or converted into public defaults while its status is unresolved.

## 6. Private user data

Private material libraries, rule packs, component catalogs, and owner design bases belong in user-controlled private paths. The public project may provide schemas and importers, but must not assume those files are redistributable.

Importers, examples, reports, telemetry, and issue templates must default to excluding private project, material, component, rule-pack, owner-standard, and company design-basis data unless a user intentionally exports or contributes it with documented redistribution rights.

## 7. Report boundary

Generated reports may reference a user rule-pack ID, version, checksum, and source note. Public report templates must not embed protected formulas or tables. Users are responsible for private report templates that quote licensed standards.

## 8. Contribution review checklist

Public contributions that add data, examples, report templates, schemas with example payloads, benchmark cases, import fixtures, or documentation excerpts must be reviewed against `governance/CONTRIBUTION_REVIEW_CHECKLIST.md` before acceptance.

The checklist records:

- contribution identity and scope;
- source, provenance, license, redistribution, and contributor certification evidence;
- protected-content and private-data risk;
- test, validation, and documentation evidence;
- reviewer disposition;
- quarantine or escalation record when needed.

Maintainers may reject or defer contributions that do not provide enough evidence to satisfy this policy. Acceptance of repository content is project governance only; it is not professional approval of engineering use.
