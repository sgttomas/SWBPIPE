# Guidance: DEL-12-05 Security threat model

## Purpose

This guidance explains how to interpret and maintain the OpenPipeStress security threat model setup content for private data handling. The deliverable exists to make privacy, protected-content, report-sharing, plugin/import, and supply-chain risks visible before implementation details harden.

The threat model is a development artifact. It supports architecture and review; it is not legal advice, professional engineering approval, certification, or a compliance attestation.

## Principles

| Principle | Guidance |
|---|---|
| Local-first by default | Treat ordinary modeling, solving, rule checking, reporting, and private-library use as local workflows. Cloud operation requires separate approval. |
| Explicit disclosure | Treat private data leaving local control as a user-intent event requiring warning, review, redaction, or explicit attachment/export action. |
| Private by default | Treat project files, rule packs, private material/component libraries, owner standards, diagnostics, reports, and secrets as private unless the user intentionally contributes or exports them with documented rights. |
| Provenance before reuse | Treat imported or contributed data without source/license/redistribution metadata as incomplete or suspect. |
| Sandboxed extensibility | Treat plugins, adapters, importers, script APIs, and rule evaluators as untrusted boundaries until validation, sandboxing, permissions, diagnostics, and no-bypass controls are specified. |
| Reports are decision support | Reports may disclose hashes, checksums, warnings, assumptions, and source notes, but public templates must not reproduce protected code text/tables or proprietary formulas. |
| Human authority preserved | Software statuses and reports must not claim certification, seal, approval, authentication, or automatic code compliance. |

Source basis: `docs/DIRECTIVE.md` sections 3-6, `docs/CONTRACT.md` invariant index, `docs/IP_AND_DATA_BOUNDARY.md`, `docs/SPEC.md` sections 6-8, and `docs/PRD.md` sections 17-18.

## Considerations

### Trust Boundaries

| Boundary | Why it matters | Conservative handling |
|---|---|---|
| User-controlled local filesystem | Private rule packs, material/component libraries, reports, and project files may coexist near public examples or source checkouts. | Separate public examples from private paths; warn on public/export destinations; scan contribution paths where applicable. |
| Report/export boundary | Reports, result exports, screenshots, and shared model packages may expose private project data or protected code-derived values. | Redaction, export warnings, protected-content checks, and explicit user action. |
| Bug-report boundary | Logs and diagnostics can disclose paths, project names, result values, rule-pack metadata, or private attachments. | Redacted default package; explicit attachment review; telemetry exclusion. |
| Telemetry boundary | Even coarse diagnostics can become sensitive if tied to private engineering work. | Off by default; opt-in only; no private engineering/code data unless user explicitly chooses a payload. |
| Plugin/import boundary | External code or file formats may bypass domain validation or introduce protected/proprietary data. | Validate through domain contracts; require provenance; sandbox where execution exists; record diagnostics. |
| Rule-evaluator boundary | Rule expressions may include private licensed/proprietary logic and must not execute arbitrary code. | Sandboxed, deterministic, unit-aware evaluator with private/public markings and checksums. |
| Supply-chain boundary | Dependencies, plugins, adapters, packages, and build artifacts may introduce malicious code or licensing/provenance gaps. | Dependency/license review, reproducible build gates, artifact integrity checks, and package provenance `TBD`. |

### Update Triggers

Update this threat model when any of these change:

- Public API transport, plugin permission model, or import/export format list is selected.
- Physical project package/container, migration mechanism, or encryption/key-management choice is selected.
- Rule expression grammar/library or sandbox approach is selected.
- Telemetry is introduced, even as an opt-in feature.
- Report/export behavior changes to include new fields, attachments, copy paths, or templates.
- Build/package/release signing or dependency review policy changes.
- A protected-content, private-data, or supply-chain issue is found in review.

### Open Questions

| Question | Current disposition |
|---|---|
| Should private rule packs or libraries be encrypted by default? | `TBD`; PRD notes optional encrypted storage and asks this as an open question. |
| What is the concrete plugin permission model? | `TBD`; no-bypass architecture basis applies until resolved. |
| What public API transport and import/export formats are supported? | `TBD`; schema-first envelopes are the current architecture basis. |
| What secret storage and signing-key process is used? | `TBD`; no real secrets may appear in setup or public examples. |
| What package/container format stores project data? | `TBD`; canonical JSON/JCS-compatible hash basis applies for JSON payload hashes. |

## Trade-offs

| Trade-off | Conservative position |
|---|---|
| Usability vs redaction friction | Prefer visible warnings and explicit user action where private/protected disclosure is plausible. |
| Extensibility vs no-bypass controls | Prefer slower plugin/API enablement over adapters that bypass domain validation, provenance, diagnostics, or report controls. |
| Rich reports vs protected-content risk | Prefer identifiers, checksums, source notes, warnings, and user-private templates over public templates that embed protected text or formulas. |
| Local convenience vs secret safety | Prefer `TBD` and explicit secret-handling design over ad hoc credentials in project files, examples, logs, or reports. |
| Open examples vs realism | Prefer invented/public/permissive examples with provenance over realistic data copied from standards, vendors, or commercial tools. |
| Supply-chain speed vs review evidence | Prefer dependency/license/provenance review and reproducible package evidence over unreviewed package additions. |

## Examples

These examples are invented setup scenarios and contain no real project data, real secrets, or protected standards content.

| Scenario | Threat-model interpretation |
|---|---|
| A user exports a calculation report for outside review. | The report path crosses the disclosure boundary; warnings, redaction options, provenance summaries, and professional-boundary notices must be present. |
| A user attaches a private project file to a bug report. | Default bug-report packaging should redact private data; explicit attachment should be treated as a user disclosure action. |
| A plugin imports a component library from an external file. | The import must pass schema, unit, provenance, redistribution, and protected-content checks before data is treated as usable. |
| A private rule pack is referenced in a report. | The report may identify name, version, checksum, and source note; it should not expose protected or proprietary rule content unless the user intentionally uses a private template. |
| A public example is proposed using vendor catalog values. | Accept only if redistribution rights and provenance are documented; otherwise quarantine or reject. |

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No active source conflict was identified in this setup run. | N/A | N/A | N/A | N/A | N/A |

## Pass 3 Semantic-Lensing Notes

The lensing pass emphasized three conservative enrichments that are now represented in the documents: update triggers, explicit supply-chain treatment, and explicit plugin/import no-bypass boundaries. Source rereads used `docs/PRD.md` sections 17-18, `docs/SPEC.md` sections 1 and 6-8, `docs/IP_AND_DATA_BOUNDARY.md` sections 4-7, and `_CONTEXT.md` Architecture Basis Injection.

