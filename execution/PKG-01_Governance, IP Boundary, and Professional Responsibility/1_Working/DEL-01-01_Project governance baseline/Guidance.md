# Guidance: DEL-01-01 Project governance baseline

## Purpose

This deliverable-local kit gives later human or Type 1 governance work a structured baseline for the first public project governance surface. It is intentionally conservative: it records required policy boundaries and unresolved choices without asserting legal conclusions or final project policy.

## Principles

- **Open mechanics, protected standards.** Governance should support a public, inspectable mechanics platform while preventing protected standards content or proprietary commercial data from entering public artifacts.
- **Maintainer authority is project authority only.** Maintainers may govern repository process, releases, and contribution acceptance; they do not professionally approve piping calculations by maintaining or releasing software.
- **Release labels are software labels.** A release may describe maturity, validation evidence, and known limitations; it must not imply engineering compliance, certification, endorsement, or sealing.
- **Unknown policy choices remain TBD.** License, release quorum, maintainer roster, signing process, and legal review process require human project authority.
- **Drafts are not policy.** Agent outputs are proposals until accepted by the human gate and recorded in the appropriate public governance artifact.

## Considerations

The current governing documents already contain much of the baseline intent. Future repo-level edits should avoid duplicating or weakening existing invariants. If `governance/MAINTAINERS.md` is created or expanded, it should point back to `docs/CONTRACT.md` and `docs/DIRECTIVE.md` rather than restating every invariant in full.

Contribution review should be written as a gate with explicit evidence fields. At minimum, public data contributions need source, provenance, redistribution status, contributor certification, and review disposition. Suspected protected content should be quarantined and escalated rather than rewritten or paraphrased.

Release review should include a checklist for scope disclosure, validation status, known limitations, data-boundary constraints, and professional-responsibility limits. These are governance and communication controls, not proofs of engineering adequacy for a project.

The human project authority for license, maintainer, release, and policy acceptance decisions is not identified in the available sources. Until recorded, all such choices remain `TBD` and should be routed for human ruling before repo-level governance artifacts are issued.

## Trade-offs

| Trade-off | Guidance |
|---|---|
| Permissive vs copyleft license | Leave as `TBD`; human project authority must decide. |
| Central maintainer vs quorum model | Leave as `TBD`; record the selected model before treating it as policy. |
| Public examples vs educational usefulness | Public examples must remain invented, permissively sourced, or public-domain with provenance. |
| Automation vs human review | Automation can provide evidence, but protected-content, legal, and professional-boundary decisions remain human governance gates. |
| Concise policy vs complete policy | Prefer concise public policy backed by explicit checklists and decision records; avoid vague assurances. |

## Examples

Acceptable draft phrasing:

- "The project license is TBD until recorded by the human project authority."
- "Maintainer approval is repository governance and is not professional engineering approval."
- "Public releases disclose validation status and known limitations."

Unacceptable draft phrasing:

- "This release is code compliant."
- "Maintainer approval certifies calculations."
- "The project includes standards-derived allowables by default."

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| C-01-01-001 | Exact open-source license is required for public governance but is not selected. | SOW-048; OPS-K-GOV-1 | docs/DIRECTIVE.md Section 6 | Datasheet Conditions; Specification Requirements; Procedure Steps | Keep license as `TBD` until human project authority records a license. | TBD |
| C-01-01-002 | Maintainer roster, quorum, release signing, and release authority are required governance choices but are not yet recorded. | OPS-K-GOV-2; docs/DIRECTIVE.md Section 6 | _CONTEXT.md anticipated artifacts | Specification Requirements; Procedure Records | Treat all role/quorum/signing values as `TBD` and do not treat them as policy. | TBD |
