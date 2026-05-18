# Guidance: DEL-08-04 Result export format

## Purpose

The result export format exists to make solver and rule-check outputs reviewable, reproducible, comparable, and consumable by downstream tools without losing the project boundaries that make OpenPipeStress code-neutral. The setup baseline is a schema-first JSON result envelope; other export formats remain `TBD`.

## Principles

- Preserve the result envelope as the governed contract. Downstream tools may consume exports, but they should not bypass unit checks, diagnostics, provenance, public/private data handling, or report controls.
- Keep the export format review-oriented. A result export can support professional review, but the software must not claim certification, sealing, approval, authentication, or automatic code compliance.
- Treat units as data, not context. Every exported numeric result needs explicit unit and dimensional meaning, or a diagnostic explaining why it is incomplete.
- Keep diagnostics structured. Review and regression tools need stable diagnostic codes/classes/severities and affected-object references, not prose-only warnings.
- Carry reproducibility references without copying private payloads. Model hashes, manifest IDs, solver versions, and rule-pack checksums are appropriate references; protected standards text, proprietary formulas, private rule-pack internals, and owner data are not public export payloads by default.
- Prefer deterministic ordering and stable identifiers for regression comparison.
- Use `TBD` for unresolved format decisions. Do not imply that a spreadsheet, CSV, FEA handoff, or other external export has been selected.

## Considerations

| Topic | Guidance |
|---|---|
| Schema-first baseline | Start from JSON result envelopes because they align with the architecture basis and can be validated before exporter code exists. |
| Results vs reports | Exports are machine-readable result contracts; reports are human-facing review artifacts. Reports may consume exports, but report wording and protected-content controls remain their own workflow. |
| Mechanics vs rule checks | Mechanics result values and user-rule-check outcomes should remain separately identifiable. A user-rule check is not professional approval. |
| Diagnostics | Export consumers should be able to filter by diagnostic class, severity, source, affected object, remediation, and provenance. |
| Regression comparison | Stable object identifiers, result-station identifiers, load-case/combination references, deterministic ordering, and unit metadata are more important than display formatting. |
| Interoperability | Downstream tools need a governed envelope first. Concrete external formats and transport protocols are later decisions. |
| Privacy and protected data | Public fixtures should use invented or permissively licensed data only. User-private exports remain user controlled and should not become public defaults. |

## Trade-offs

| Trade-off | Setup position |
|---|---|
| Single JSON envelope vs many early formats | Use the JSON envelope as the baseline. Additional formats remain `TBD` to avoid premature commitments and governance bypasses. |
| Verbose metadata vs compact exports | Preserve units, diagnostics, provenance, and status semantics even when compactness is desirable. Reviewability takes precedence over minimal payload size. |
| Human-readable labels vs stable machine keys | Include stable identifiers for comparison and downstream tooling. Human-readable labels may supplement them but should not be the only references. |
| Private rule-pack detail vs reproducibility references | Export identifiers, versions, checksums, source notes, and redistribution status; do not copy private formulas or protected values into public artifacts. |
| Report convenience vs professional boundary | Avoid report/export wording that could be read as approval or compliance. Human review remains external to the software result. |

## Examples

The following are setup-level examples of export concerns, not final schema definitions:

| Concern | Acceptable setup direction | Not acceptable in public baseline |
|---|---|---|
| A displacement result | Unit-aware value with node/result reference, load-case or combination basis, and source run metadata. | Numeric value without unit metadata or affected-object reference. |
| A rule-check outcome | User-rule-check result with rule-pack ID/version/checksum and status distinguishing incomplete inputs, checked, or failed. | Automatic `CODE_COMPLIANT` status or copied protected formula text. |
| A warning | Structured diagnostic with class, severity, source, affected object, message, remediation, and provenance. | Prose-only warning that cannot be traced or compared. |
| A downstream export | Adapter consumes governed JSON envelope and preserves units/provenance/diagnostics. | Adapter-specific shortcut that drops diagnostics or private-data handling. |

## Conflict Table (for human ruling)

No unresolved source conflicts were found during this setup run. Open format choices remain `TBD` by scope, not a source conflict.
