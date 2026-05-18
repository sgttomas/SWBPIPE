# Specification: DEL-01-02 Copyright and protected-data boundary policy

## Scope

This deliverable-local specification defines requirements for the future repo-level copyright and protected-data boundary policy and contribution review checklist. It covers public-repository data boundaries, contributor provenance controls, suspected protected-content quarantine, and review evidence. It does not edit repo-level artifacts, implement product code, decide the project license, provide legal advice, certify compliance, or approve engineering use.

## Requirements

| ReqID | Requirement | Source | Verification |
|---|---|---|---|
| DEL-01-02-R1 | The policy shall state that the public repository must not contain protected standards text, tables, figures, examples, copied code formulas, material allowables, SIF/flexibility tables, protected dimensional tables, or proprietary commercial data. | docs/CONTRACT.md §1 OPS-K-IP-1; docs/_Decomposition/SOFTWARE_DECOMP.md SOW-003 | Review policy prohibited-content section and protected-content checklist entries. |
| DEL-01-02-R2 | The policy shall require public data contributions to include source, provenance, license or redistribution status, contributor certification, and review disposition. | docs/CONTRACT.md §1 OPS-K-IP-2; docs/IP_AND_DATA_BOUNDARY.md §4 | Confirm contribution checklist has each metadata field. |
| DEL-01-02-R3 | The policy shall define suspected protected-content handling as stop, quarantine, issue record, and human/legal escalation. | docs/CONTRACT.md §1 OPS-K-IP-3; docs/IP_AND_DATA_BOUNDARY.md §5 | Confirm quarantine path/process and escalation fields are present; exact path is `TBD`. |
| DEL-01-02-R4 | The policy shall distinguish open mechanics from user-supplied code-specific and proprietary data. | INIT.md; docs/DIRECTIVE.md §1, §3; docs/CONTRACT.md OPS-K-DATA-1 | Review allowed/prohibited/private categories. |
| DEL-01-02-R5 | The policy shall state that missing, unknown, or undocumented provenance/redistribution status blocks public acceptance until resolved. | docs/CONTRACT.md OPS-K-DATA-2, OPS-K-IP-2; docs/TYPES.md §5, §7 | Checklist includes `TBD`, `UNKNOWN_SOURCE`, and rejection/quarantine outcomes. |
| DEL-01-02-R6 | The policy shall require public rule-pack examples and public report templates/examples to avoid protected standards content and use invented or permissively sourced data. | docs/CONTRACT.md OPS-K-RULE-1, OPS-K-REPORT-2; docs/SPEC.md §6, §8 | Review public-example and report-template review rows. |
| DEL-01-02-R7 | The policy shall make contributor review a maintainer gate for IP, provenance, privacy, and protected-content risks before accepting public contributions. | docs/CONTRACT.md OPS-K-GOV-4; docs/DIRECTIVE.md §6 | Checklist includes review disposition and responsible reviewer field. |
| DEL-01-02-R8 | The policy shall not claim legal advice, professional engineering approval, certification, sealing, code compliance, or standards-body endorsement. | docs/CONTRACT.md OPS-K-AUTH-1, OPS-K-AGENT-4; docs/DIRECTIVE.md §4.2, §6 | Review notices and forbidden-claim checks. |
| DEL-01-02-R9 | The policy shall preserve stable IDs and traceability to DEL-01-02, PKG-01, SOW-003, SOW-028, and OBJ-002 in the deliverable-local kit. | docs/CONTRACT.md OPS-K-HIER-1, OPS-K-ID-1; docs/TYPES.md §1-2 | Confirm IDs appear consistently in this kit. |
| DEL-01-02-R10 | Architecture-facing references to diagnostics, result envelopes, tests, and gates shall remain constraints for downstream implementation only unless this deliverable explicitly resolves them. | docs/_Decomposition/SOFTWARE_DECOMP.md AB-00-06, AB-00-08; sealed brief | Confirm policy text uses non-implementation wording and records downstream handoff. |
| DEL-01-02-R11 | The final repo-level checklist location, reviewer role, and owning governance surface shall remain `TBD` until assigned by the human project authority. | _SEMANTIC_LENSING.md F-001, E-001; docs/CONTRACT.md OPS-K-GOV-2, OPS-K-GOV-4 | Confirm repo-level artifact path and owner are not invented in this deliverable-local kit. |

## Standards

No protected standards text or clause-level standard requirements are used as source material. Standards-body content is treated as excluded public-repository content unless explicit redistribution rights are documented and human/legal review accepts it. Specific legal requirements, licenses, and quarantine storage paths are `TBD`.

## Verification

- Four-document kit exists in the deliverable folder.
- Default sections are present in Datasheet, Specification, Guidance, and Procedure.
- Requirements cite local project sources rather than protected standard text.
- Unknowns are marked `TBD`.
- No product implementation, protected standards/code data, legal conclusion, certification, sealing, approval, or compliance-for-reliance claim is introduced.
- The contribution checklist verification remains field-by-field but blocked on final repo-level checklist path/owner until human ruling.
- Dependency register validates against v3.1 schema after extraction.

## Documentation

Expected future repo-level artifacts are `docs/IP_AND_DATA_BOUNDARY.md` and a contribution review checklist. In this setup workflow, those artifacts are discussed only in this deliverable-local kit and are not edited.
