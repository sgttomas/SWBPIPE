# Guidance: DEL-01-02 Copyright and protected-data boundary policy

## Purpose

The policy exists to protect standards-body and vendor intellectual property while allowing OpenPipeStress to remain a public, educational, auditable implementation of open mechanics. It should give maintainers and contributors a conservative intake path: accept only content with documented rights, keep private/project data private, and escalate suspected protected content before it reaches the public repository.

## Principles

- Open mechanics are public; code-specific values, proprietary libraries, protected standards tables, and owner/project data are not public defaults.
- Provenance is part of the data, not an optional note.
- `TBD`, `UNKNOWN_SOURCE`, and `PROTECTED_CONTENT_SUSPECTED` are valid findings; they should not be silently converted into public content.
- Contributor review is a gate, not a courtesy label.
- Quarantine is protective and procedural; it is not a finding of infringement or a legal conclusion.
- Public examples should be original, invented, public-domain, or permissively licensed, with a clear non-engineering notice.

## Considerations

The existing draft `docs/IP_AND_DATA_BOUNDARY.md` already provides useful categories for allowed public content, prohibited public content, provenance fields, quarantine behavior, private user data, and reports. A future repo-level update should keep those categories but may need human decisions for exact quarantine storage path, contributor certification wording, license status vocabulary, reviewer roles, and escalation owner.

Architecture-basis items AB-00-01, AB-00-02, AB-00-06, and AB-00-08 matter as downstream constraints: protected-content and provenance gates should be testable, diagnostics should include `IP_BOUNDARY_WARNING` where implementation later touches contribution/report flows, and adapters/plugins should not bypass provenance or data-boundary controls. This document does not implement those controls.

## Trade-offs

| Trade-off | Guidance |
|---|---|
| Usability vs. protected-content risk | Prefer conservative rejection or quarantine when redistribution status is unclear. |
| Educational examples vs. accidental copying | Use invented examples and avoid code-derived values, table shapes, copied wording, and proprietary benchmark files. |
| Automation vs. legal judgment | Automated scans can flag risk but cannot be the sole legal control. Human/legal review remains required for suspicious or high-impact contributions. |
| Public transparency vs. private data protection | Record policy and review outcomes publicly where safe, but do not expose private rule packs, owner standards, project models, or quarantined content. |

## Examples

- Acceptable pattern: a small invented example model with original values, a non-engineering notice, and provenance marked `PUBLIC_DOMAIN_OR_ORIGINAL`.
- Review-required pattern: manufacturer data with unclear redistribution terms; record `source_license=TBD` and block public acceptance until resolved.
- Quarantine pattern: a contribution that appears copied from a standards table; stop ingestion, mark `PROTECTED_CONTENT_SUSPECTED`, move outside public examples, record issue, and request human/legal review.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
|---|---|---|---|---|---|---|
| C-001 | Exact quarantine storage path is required operationally but not specified in the accessible governance sources. | docs/IP_AND_DATA_BOUNDARY.md §5 | docs/CONTRACT.md OPS-K-IP-3 | Procedure §Steps; Specification R3 | Human project authority should define a private/quarantine path outside public examples. | TBD |
| C-002 | Contributor certification mechanism is required by policy intent but exact wording/mechanism remains unresolved. | docs/CONTRACT.md OPS-K-IP-2 | docs/DIRECTIVE.md §6 | Specification R2; Procedure checklist | Human/legal review should approve final attestation language. | TBD |
| C-003 | Final contribution checklist path and reviewer role are needed for execution but are not assigned by the sealed setup context. | _CONTEXT.md §Anticipated Artifacts | docs/CONTRACT.md OPS-K-GOV-2, OPS-K-GOV-4 | Specification R11; Procedure §Prerequisites | Human project authority should assign the owning governance surface and reviewer role. | TBD |
