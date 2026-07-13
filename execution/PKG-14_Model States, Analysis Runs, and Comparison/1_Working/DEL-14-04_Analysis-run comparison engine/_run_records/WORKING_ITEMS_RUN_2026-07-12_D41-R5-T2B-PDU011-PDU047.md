# WORKING_ITEMS Run — D-41 R5 T2B / PDU-011 and PDU-047

- Date: 2026-07-12
- Role: deliverable-owning implementation/evidence pilot
- Deliverable: DEL-14-04
- Decision inputs: PDU-011 supplementary check; PDU-047; E2/E4/E8 as evidence requirements only

## Work

For PDU-011, inspected the accepted schemas and comparison implementation. No schema governs `AnalysisRunComparison.to_dict()` output; `comparison_mapping.schema.json` and `comparison_tolerance.schema.json` govern inputs. A positive/negative output-conformance test would therefore create a test-authored contract, so the gap is held.

For PDU-047, confirmed the TP-PHYS-015 section-property oracle cannot validate unit-normalized comparison mechanics and `section_property` is not a currently supported result family. Adding it or selecting conversion/tolerance behavior would expand scope, so the engineering-suitability gap is held.

Recorded both residuals in the four-document kit, memory, and status. No comparison/schema behavior changed.

## Verification

- The focused Python command recorded by the DEL-03-08 owning record included `tests/test_analysis_run_comparison.py` and passed as part of 43 tests.
- Schema inventory inspection found no canonical comparison-output schema.
- `git diff --check` is required at fan-in.

## Preserved Boundaries

Lifecycle remains `IN_PROGRESS`. No output schema, result family, conversion, tolerance, threshold, validation outcome, review disposition, dependency, DAG, register, release decision, or engineering-validation claim was introduced.
