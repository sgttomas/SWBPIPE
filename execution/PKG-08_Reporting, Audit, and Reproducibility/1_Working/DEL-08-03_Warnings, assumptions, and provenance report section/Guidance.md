# Guidance: DEL-08-03 Warnings, assumptions, and provenance report section

## Purpose

This deliverable exists so calculation reports make uncertainty, missing inputs, assumptions, and provenance visible to professional reviewers. The section should help a reviewer see what the software computed, what the user supplied, what remains incomplete, and what must not be treated as software-certified engineering approval.

## Principles

| Principle | Guidance |
|---|---|
| Findings over defaults | Missing solve-required or rule-check-required values should appear as explicit findings, not be filled by convenience defaults. |
| Provenance over opacity | Values affecting engineering reliance should carry source/provenance notes or a visible `TBD`/unknown-source finding. |
| Warnings preserve origin | Report rendering should carry diagnostic class, severity, source, affected object, and remediation from the producing layer when available. |
| Decision support only | Report language may support professional review but must not claim certification, sealing, approval, authentication, or automatic code compliance. |
| Protected-content restraint | Public templates/examples may identify rule-pack references and checksums but must not reproduce protected standards text, tables, copied formulas, or proprietary content. |
| Unit visibility | Values shown in the section should preserve unit context wherever units exist in upstream data. |

## Considerations

The report section should be a consumer of upstream diagnostics and provenance, not a source of new engineering meaning. If upstream data says a rule-pack input is missing, the report should expose that state. If a source note is missing, the report should expose that as a provenance warning. If a human has not accepted the output for project use, the report should not imply that acceptance.

The section should remain useful even when upstream components are incomplete. `TBD`, `UNKNOWN_SOURCE`, or missing manifest fields are acceptable setup outputs when the evidence is not available. They should be made visible rather than normalized away.

When upstream diagnostic or provenance fields are missing, the safer report behavior is to preserve the missingness as `TBD` or an explicit warning. Inferring a message, source, severity, or professional disposition inside the report renderer would create unaudited meaning and could blur the boundary between software output and engineering judgment.

## Trade-offs

| Trade-off | Preferred Direction |
|---|---|
| Readability vs audit detail | Keep the section readable, but retain machine-traceable warning/provenance fields in the report source data. |
| Private detail vs public safety | Public templates should reference private rule-pack/library identity, version, checksum, and source notes without exposing protected/private content by default. |
| Concise warnings vs remediation | Include enough remediation/source context for review; avoid turning warnings into instructions that imply professional acceptance. |
| Early implementation vs dependency maturity | Future code may begin with schema-backed fixtures and `TBD` integration points while upstream diagnostics, manifest, and linter deliverables mature. |

## Examples

The following examples are invented reporting patterns only. They do not encode code-specific requirements or protected standards content.

| Scenario | Report Section Behavior |
|---|---|
| Rule-pack required input is absent | Show a rule-check blocking finding, affected object/reference if supplied, and remediation such as "provide required user/rule-pack input"; do not report a pass/fail code result. |
| Component value has no source note | Show the value only with its unit context if otherwise permitted, mark source/provenance as `UNKNOWN_SOURCE` or `TBD`, and emit a provenance warning. |
| User entered an assumption for a support condition | Show the assumption as user/model-supplied and requiring review; do not treat it as verified project acceptance. |
| Private rule pack is used | Show rule-pack name or ID, version, checksum, private/public status, and source notice if available; do not quote protected formulas or standards clauses in public templates. |
| Solver emitted nonlinear uncertainty | Show nonlinear warning class, severity, source, affected object, and remediation if supplied by the result envelope. |

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No unresolved source conflict found during setup. | N/A | N/A | N/A | N/A | N/A |

## Open Questions

| Question ID | Question | Current Disposition |
|---|---|---|
| DEL-08-03-Q-001 | Exact report renderer API and template format | `TBD`; owned by future implementation and DEL-08-01 integration |
| DEL-08-03-Q-002 | Exact data contract for warning/provenance payloads | `TBD`; constrained by AB-00-06 diagnostics/result-envelope basis |
| DEL-08-03-Q-003 | Exact protected-content linter interface | `TBD`; likely dependent on DEL-08-05 |
| DEL-08-03-Q-004 | Exact audit manifest field names | `TBD`; likely dependent on DEL-08-02 |
| DEL-08-03-Q-005 | Canonical professional notice wording for final templates | `TBD`; align with product-claims policy or approved report notice before implementation release |
