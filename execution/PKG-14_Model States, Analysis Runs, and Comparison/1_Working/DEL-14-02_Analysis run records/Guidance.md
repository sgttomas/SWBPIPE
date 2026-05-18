# Guidance: DEL-14-02 Analysis run records

## Purpose

Analysis run records exist so solver outputs can be reviewed as reproducible records bound to the exact model state, execution context, diagnostics, references, and hashes that produced them. The record is a traceability surface for design iteration and review; it is not an automatic professional approval or external validation state.

## Principles

| Principle | Guidance | Source |
|---|---|---|
| Bind results to their run basis | Treat the run record as the durable home for result evidence, not as a transient view of the current model. | SOW-072; `_CONTEXT.md` envelope notes |
| Preserve exact upstream references | Model state, solver version, settings, units, load-case basis, diagnostics, rule packs, libraries, and result hashes are binding categories, not optional narrative notes. | SOW-072 |
| Keep units explicit | Unit-bearing physical values need unit metadata; missing or ambiguous units become diagnostics rather than inferred defaults. | `docs/SPEC.md` section 4 |
| Make hashes scoped and reproducible | Hash records should say what was hashed and what canonicalization basis was used. JSON payloads use the accepted JCS-compatible basis where applicable. | `docs/SPEC.md` section 4.4 |
| Keep authority domains separate | Solver results and diagnostics are software outputs. User-rule checks use user-supplied rule-pack data. Human acceptance, if used, is external and hash-bound. | `docs/SPEC.md` analysis boundary section |
| Protect private/protected payloads | Rule-pack and library references may carry identity, version, checksum, provenance, privacy, and review metadata, but public artifacts must not copy private/protected content. | `docs/IP_AND_DATA_BOUNDARY.md`; `docs/SPEC.md` result export section |

## Considerations

- The folder is a setup-stage production unit. It contains no implementation evidence yet; statements about passing behavior, field names, storage APIs, or committed tests must remain TBD until produced by later Type 2 work.
- SOW-072 is sufficient to identify required binding categories, but it is not sufficient to define exact schema field names, cardinality, or migration behavior.
- The dependency mirror records approved DAG-002 predecessor evidence for architecture basis, immutable model state records, analysis status semantics, audit manifest/hash conventions, result export format, and persistence/round-trip support. Those rows are an evidence surface, not independent graph authority.
- The deliverable should align with `schemas/results.schema.yaml` and result-export envelopes where run records include or point to result payloads, but this folder does not define the final result export schema.
- `PRD v0.2` references are cited by decomposition/register rows but the PRD source text was not locally read in this task. Requirements must not exceed accessible SOW/decomposition wording.

## Trade-offs

| Topic | Conservative posture |
|---|---|
| Embedded results vs references | Use the source-supported term "results" without deciding whether the schema embeds result payloads, references result envelopes, or supports both. Exact representation is TBD. |
| Hash granularity | Require explicit payload scope. Do not decide final partitioning for non-JSON or binary payloads; `docs/SPEC.md` leaves that TBD. |
| Rule-pack/library metadata depth | Preserve identity/provenance/checksum/reference metadata without copying private formulas, protected tables, proprietary values, or private payloads. |
| Status vocabulary | Use software status/diagnostic categories only within the professional-boundary model. Do not introduce automatic human approval or code-compliance statuses. |
| Comparison readiness | DEL-14-02 should not pre-solve DEL-14-04 comparison semantics. It should preserve enough run basis and result hash evidence to support deterministic comparison later. |

## Examples

TBD. No approved example analysis-run payloads or fixture values are present in the accessible sources. Public examples must be invented or otherwise cleared for redistribution before use.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None identified | No direct source conflict was identified in the accessible source slices. Several details remain TBD because source text is absent or implementation-specific. | N/A | N/A | N/A | N/A | N/A |
