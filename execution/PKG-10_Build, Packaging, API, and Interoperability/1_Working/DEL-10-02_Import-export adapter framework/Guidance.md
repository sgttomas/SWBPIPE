# Guidance: DEL-10-02 Import/export adapter framework

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-10-02-DECL-003`.

## Purpose

This deliverable provides a format-neutral adapter declaration validator and a deny-only runtime admission gate so later implementation cannot treat declaration acceptance as runtime authority. The framework is an extension surface, not a route around domain validation, unit safety, provenance, rule-pack sandboxing, reporting controls, or professional-responsibility limits.

## Principles

- Treat adapters as translators at the edge of the application service boundary.
- Require schema-first envelopes for nontrivial import/export operations.
- Validate units and dimensional meaning before external data becomes domain data.
- Preserve source/provenance, redistribution status, and private/public data markings.
- Emit diagnostics for missing fields, missing provenance, unclear redistribution, suspected protected content, unit inconsistencies, and private-data export risk.
- Keep concrete format support `TBD` until a human decision records the external format, license posture, redistribution status, and test obligations.
- Use invented data only for public samples.
- Treat `ACCEPTED_FORMAT_NEUTRAL_DECLARATION` as declaration evidence only. Runtime dispatch remains blocked until a separately governed execution/plugin model is selected.

## Considerations

An adapter may be technically able to parse many files, but parsing does not establish redistribution rights, engineering suitability, code compliance, or professional acceptance. The framework should therefore separate:

- syntactic parse success;
- schema and unit validation;
- provenance and redistribution review;
- mechanics-readiness;
- rule-check-readiness;
- human-review-needed state.

Adapters that import private material libraries, component records, rule-pack references, or project data should default to local/private handling. Export operations should warn before writing private or protected-suspected values to shared payloads.

The current O7/E5 seam is deliberately narrow: a declaration disabling unit,
provenance, privacy, protected-content, rule-pack sandbox, checksum, report, or
private-transmission controls is rejected; suspected protected content is
quarantined; a valid declaration is still not dispatched. This does not prove
other adapters, plugins, reports, exports, telemetry, or storage seams.

## Trade-offs

| Trade-off | Guidance |
|---|---|
| Flexible adapter ecosystem vs. governance control | Favor a narrow framework with mandatory validation hooks; add format-specific adapters later. |
| User convenience vs. data provenance | Missing source or redistribution status is a finding, not an auto-filled default. |
| Broad public formats vs. IP safety | Do not bundle protected or proprietary defaults without documented rights and human approval. |
| Fast export vs. auditability | Exports should carry diagnostics, hashes/manifests where applicable, and warning state. |

## Examples

- Acceptable public sample: an invented adapter that imports a small invented component record with invented dimensions, invented source metadata, and permissive redistribution marked as invented/original.
- Not acceptable public sample: a bundled adapter fixture copied from a standards table, vendor catalog, commercial software example, or private project library without documented redistribution rights.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No current conflicts detected in setup sources. | NA | NA | NA | NA | TBD |
