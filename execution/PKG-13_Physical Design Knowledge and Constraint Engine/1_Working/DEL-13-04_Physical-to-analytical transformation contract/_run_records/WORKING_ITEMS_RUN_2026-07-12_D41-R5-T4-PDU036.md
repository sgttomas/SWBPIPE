# WORKING_ITEMS Run — D-41 R5 T4 PDU-036 trace-gap fixture

- Added `fixtures/domain/invented_physical_to_analytical_trace_gap.json`.
- Focused test proves one unsupported component is omitted, emits a linked `ASSUMPTION_WARNING`, and receives no invalid field-scalar trace.
- Evidence remains project-owned verification; broader field/runtime trace coverage and independent validation remain open.
- Validation: cache-disabled focused Python package/consumer suite passed 52/52, including 15/15 transform tests.
- No validation promotion, product acceptance, lifecycle, dependency/DAG/register/decomposition, or `ISSUED` change.
