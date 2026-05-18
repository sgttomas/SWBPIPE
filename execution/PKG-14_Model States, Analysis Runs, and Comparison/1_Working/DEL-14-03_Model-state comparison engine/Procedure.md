# Procedure: DEL-14-03 Model-state comparison engine

## Purpose

Produce or use the model-state comparison engine in a way that remains deterministic, source-grounded, unit-aware where applicable, and bounded to diagnostic/audit comparison of immutable model states.

## Prerequisites

| Prerequisite | Status / source |
|---|---|
| Immutable model-state record contract | Upstream dependency `DEL-14-01`; approved DAG-002 mirror row `DAG-002-E0792`. |
| Mapping/tolerance contract | Upstream dependency `DEL-14-05`; approved DAG-002 mirror row `DAG-002-E0793`; details `TBD` per `OI-014`. |
| Unit system and dimensional-analysis contract | Upstream dependency `DEL-02-02`; approved DAG-002 mirror row `DAG-002-E0794`. |
| Architecture basis | `AB-00-01`, `AB-00-02`, `AB-00-03`, `AB-00-04`, `AB-00-06`, `AB-00-07`, and `AB-00-08` in `_CONTEXT.md`. |
| Professional/IP boundary | `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, and `docs/IP_AND_DATA_BOUNDARY.md`. |

## Steps

1. Confirm the work is limited to `DEL-14-03` model-state entity diffs. Escalate if the task expands into analysis-run result deltas, export formats, GUI overlays, external prover status, or professional approval workflow.

2. Identify the immutable state input contract from `DEL-14-01`. If the contract is unavailable, record the input schema as `TBD` and do not invent entity fields.

3. Identify the comparison mapping contract from `DEL-14-05`. If unavailable, implement only stable-ID matching assumptions that are source-supported, and mark manual mapping behavior `TBD`.

4. Identify unit-bearing fields through the unit contract from `DEL-02-02`. Do not compare unit-bearing values as bare numbers; missing or incompatible unit metadata should produce diagnostics.

5. Define the comparison input envelope as two model-state references or payloads plus any explicit mapping/settings record. Exact service/API syntax is `TBD`.

6. Match entities by stable ID. Apply explicit mapping records where the mapping contract allows them. Treat unresolved correspondence as unmatched or diagnostic rather than hidden equivalence.

7. Classify entities into added, removed, changed, and unchanged categories. Exact field-level normalization and change significance rules are `TBD` pending state schema and mapping/tolerance contract detail.

8. Preserve review context: state hashes, warnings, unresolved assumptions, notes, external references, provenance, and diagnostics where present.

9. Return a deterministic result envelope with professional-boundary language. Do not emit automatic certification, sealing, code-compliance, external-validation, or human-approval statuses.

10. Build state diff tests using invented/public-safe fixtures. Cover stable-ID matching, added/removed/changed/unchanged classifications, mapping behavior once available, deterministic repeatability, unit metadata diagnostics, and forbidden-claim checks.

## Verification

| Check | Expected evidence |
|---|---|
| Stable-ID determinism | Repeated comparisons of the same fixtures produce identical classifications and result serialization where implemented. |
| Mapping behavior | Explicit mappings produce expected correspondence; missing mappings remain visible. `TBD` until `DEL-14-05` exists. |
| Changed classification | Source-supported relevant field changes are reported as changed. Exact normalization policy is `TBD`. |
| Unit metadata | Unit-bearing fields require unit/dimension context or produce diagnostics. |
| Metadata preservation | Warnings, assumptions, external references, notes, provenance, and hashes are not silently dropped. |
| Boundary language | No result label claims professional acceptance, code compliance, certification, sealing, authentication, or external validation. |
| Fixture governance | Test fixtures are invented/public-safe and do not include protected standards text, proprietary values, or private project data. |

## Records

- Implementation/module path: `TBD`.
- State comparison service or engine contract: `TBD`.
- State diff test artifact path: `TBD`.
- Fixture provenance notes: `TBD`.
- Dependency notes: preserve the existing DAG-002 mirror rows as ACTIVE unless a future approved coordination workflow changes the mirror.
