# Inspection test identity repair

Only the two new positional state assertions in
unsupported_gap_inspection_preserves_mixed_computation_without_qualification
changed. They now locate the exact support IDs gap and friction, with explicit
missing-state errors, then retain the original expected Active and Sliding
states. State ordering is not inferred from construction order.

The parent's focused run exposed that final states are sorted by support ID;
friction/gap/one-way makes index2 one-way, not friction. This repair changes test
identity association only, not any source fixture, physics expectation, count,
criterion, solver or inspection/qualification behavior. The two other focused
negative suites reportedly passed; this TASK ran no tests.

SOURCE_FREEZE.json and SOURCE.diff pin the single-file delta relative to
REPO_ROOT. Exact before/after bytes and runtime attribution are under _run_records.
The before nonlinear source is572d0ed1…; product646b648b… was left unchanged.
REPAIR02 remains historical. Static replacement reversal recovers the complete
before source exactly. Same-reviewer backcheck and manager-owned focused rerun
remain required. No Cargo/build/Git or product edits occurred.

Actual author: native TASK /root/solver_manager/evidence_primitives under
/root/solver_manager; no descendants.
