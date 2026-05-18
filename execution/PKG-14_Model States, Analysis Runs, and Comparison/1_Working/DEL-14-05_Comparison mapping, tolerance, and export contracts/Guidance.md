# Guidance: DEL-14-05 Comparison mapping, tolerance, and export contracts

## Purpose

DEL-14-05 exists to define the comparison-facing contracts that let model-state and analysis-run comparison results be mapped, interpreted with tolerances, and exported for review without turning diagnostic comparison output into professional approval or external validation.

Sources: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#DEL-14-05`; `execution/_Decomposition/SOFTWARE_DECOMP.md#SOW-073`.

## Principles

| Principle | Guidance |
|---|---|
| Deterministic identity first | Stable IDs are the baseline for comparison. Manual mapping should be explicit evidence where stable-ID alignment is insufficient. Source: `execution/_Decomposition/SOFTWARE_DECOMP.md#SOW-073`. |
| Missing values stay visible | Tolerance defaults, mapping workflow details, and exact export field sets remain `TBD` where sources do not define them. Do not substitute engineering defaults. Source: `docs/CONTRACT.md#OPS-K-DATA-2`; `execution/_Decomposition/SOFTWARE_DECOMP.md#OI-014`. |
| Units are contract data | Unit-normalized deltas still need explicit unit/dimensional metadata or diagnostics. Source: `docs/CONTRACT.md#OPS-K-UNIT-1`; `docs/SPEC.md#result-export-format`. |
| Export is review evidence | Exports should carry enough context for review, regression comparison, report consumption, and downstream tooling without claiming external validation. Source: `docs/SPEC.md#result-export-format`; `execution/_Decomposition/SOFTWARE_DECOMP.md#SOW-073`. |
| Professional authority remains human | Avoid terms or statuses that imply software certification, sealing, professional approval, authentication, or code compliance. Source: `docs/CONTRACT.md#OPS-K-AUTH-1`; `docs/TYPES.md#Analysis-status-vocabulary`. |
| Public artifacts stay clean | Public examples and contract documentation must not embed protected standards text, protected tables, code-specific values, proprietary data, or private rule-pack payloads. Source: `docs/IP_AND_DATA_BOUNDARY.md#public-repository-must-not-contain`. |

## Considerations

- The comparison mapping contract depends on upstream model-state and analysis-run identity surfaces. The local approved DAG-002 mirror records upstream dependencies on DEL-14-01 and DEL-14-02.
- Unit-aware tolerance behavior depends on the unit-system contract. The local approved DAG-002 mirror records an upstream dependency on DEL-02-02.
- Export semantics should remain compatible with result export envelopes. The local approved DAG-002 mirror records an upstream dependency on DEL-08-04.
- Architecture basis rows in the local mirror are context evidence, not independent Type 2 dispatch authority.
- The accessible sources do not define exact mapping enums, unmatched classification values, tolerance formulas, tolerance default values, CSV columns, JSON property names, or report-section layout. These remain `TBD`.

## Trade-offs

| Trade-off | Direction |
|---|---|
| Strict schema versus future external result states | Prefer schema-first contracts with explicit extension/TBD slots because `_CONTEXT.md` notes that future external result states may be represented without changing core compare logic. |
| Human readability versus machine validation | Preserve both reviewable export forms and schema validation hooks; exact CSV/report fields remain TBD until contract details are sourced. |
| Default tolerances versus no silent defaults | Keep defaults TBD unless a future governed source supplies them. Silent tolerances would conflict with the no-silent-defaults principle. |
| Broad external comparison support versus bounded MVP | Stay within deterministic state/run comparison and comparison export semantics; comprehensive commercial prover ingestion is excluded by PKG-14. |

## Examples

TBD. No locally accessible source provides approved example mappings, tolerance profiles, CSV rows, JSON payloads, or report-section examples for DEL-14-05.

## Conflict Table (for human ruling)

No source conflict requiring human ruling was identified during the P1/P2 setup pass. The main unresolved items are source gaps, recorded as `TBD`: tolerance defaults, mapping workflow details, exact export fields, and report-section layout.
