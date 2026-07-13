# Guidance: DEL-03-04 Branch connection component model fields

## Purpose

This deliverable exists to keep branch connection component data explicit,
unit-aware, provenance-bearing, and separated from protected standards content.
The current component schema and invented fixture provide branch schema slots
and missing-data diagnostics so users or lawful private libraries can supply
branch-specific data without the public project embedding protected branch
connection, SIF, flexibility, or reinforcement tables.

## Principles

- Treat branch connection data as user/imported input unless a public, rights-cleared source is documented.
- Preserve the difference between data capture, solver mechanics, rule-pack evaluation, and human professional judgment.
- Prefer explicit `TBD` or diagnostics over inferred defaults.
- Keep provenance and redistribution status close to the values they qualify.
- Make dimensional fields unit-aware; do not accept unitless dimensional values unless a later schema deliberately defines a safe representation.
- Use the implemented branch field names in `schemas/component.schema.yaml` when
  documenting current schema evidence; reserve `TBD` for unresolved policy,
  source, lifecycle, dependency, or human-disposition questions.

## Considerations

The branch connection model has more local field slots than simpler component
families. Current evidence covers branch run/header size, connection angle/type,
reinforcement area/reference, branch geometry source reference, user SIF, and
user flexibility-factor field kinds. The decomposition still leaves specialized
branch local-check methods, concrete import formats, and GUI behavior outside
this reconciliation.

Architecture basis implications:

- AB-00-02 and AB-00-07 point toward domain contracts and validation boundaries rather than GUI or adapter shortcuts.
- AB-00-04 points toward deterministic, provenance-preserving persistence when serialized.
- AB-00-06 points toward structured diagnostics for blocking missing data and boundary warnings.
- AB-00-08 points toward layered tests, including protected-content/provenance gates.

## Trade-offs

| Topic | Guidance |
|---|---|
| Field specificity vs protected content | Capture field categories and user-supplied values; do not encode protected lookup content. |
| Completeness vs implementation timing | Treat current branch schema field names as implemented evidence; keep specialized local-check needs, source catalogs, import formats, and lifecycle acceptance as `TBD`. |
| Public examples vs validation | Use the existing invented non-engineering fixture as schema/diagnostic evidence only; do not treat its missing branch values as engineering examples. |
| User flexibility vs diagnostics | Permit user-supplied branch data, but report missing or unverifiable values explicitly. |

## Examples

The current public fixture is an invented schema fixture, not an engineering
example. It intentionally omits branch geometry, reinforcement, SIF, and
flexibility values and emits `BRANCH_RULE_INPUT_MISSING` rather than supplying
public defaults. Future value-bearing examples, if any, must be invented
non-code values with clear non-engineering notices or must come from reviewed
public-permissive/private sources; they must not reproduce protected tables,
formulas, or code examples.

## Review Posture

Local review findings for unit-dependency satisfaction and component diagnostic
envelope compatibility are `ACCEPT_AS_IS` / `RESOLVED` in
`Review_Findings.csv` under the recorded 2026-06-05 Gate C disposition.
Narrative evidence may cite the technical branch schema, fixture, and diagnostic
updates, while dependency and lifecycle state remain governed separately.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
|---|---|---|---|---|---|---|
| None identified | No conflict was detected between the current branch schema/fixture/test evidence and the local review/memory evidence. | `schemas/component.schema.yaml`; fixture; test | `MEMORY.md`; `_REVIEW.md`; `Review_Findings.csv` | N/A | Treat implementation evidence as current while preserving policy, dependency, human-disposition, and lifecycle gates. | TBD |
## D-41 R5 T3 PDU-019 Boundary

Schema rejection evidence is not a formal review outcome and must not be represented as one.
