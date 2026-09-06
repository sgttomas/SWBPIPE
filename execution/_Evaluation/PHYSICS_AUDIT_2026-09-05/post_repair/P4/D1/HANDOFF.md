# D1 current derivative capture

PASS for this evidence-only task. Nine corrected DEC053 observations are in `emitter.raw.json`; their historical comparison is in `comparison.csv`. All 98 finite, fixture, hash, existing bounded parity/residual/repeat/pivot and metadata checks pass (`QA.json`). Capture is bound to HEAD `2be412ccea62bdc4bd96deb082c46d7a792076ea` plus exact dirty source hashes and accepted KERNEL_CHECKPOINT_V1 manifest `ff28049d1ff3ea27feb8f8ec9759cef10335792797a01a4dede03b897de5eae1`.

The nine condition estimates changed; underlying fixture sizes, numerical solution parity, residuals, repeat differences, pivots and value-storage observations equal the historical packet exactly. Chain-8 changes from 619027.975799394 to 2158698.8565041865. Chain-48 changes from 18020.095204734607 to 2239508033.9473214. These are corrected diagnostic observations, not evidence of a corresponding deterioration in solve accuracy and not independently calibrated condition acceptance limits.

The final capture used approved host escalation of the same `cargo run --locked --offline` example because sandbox `ps` was denied. The sandbox attempt remains under `sandbox_attempt/`; its RSS zero is an emitter fallback sentinel. Final measured RSS is 9712 KiB. Timing and RSS vary between runs and are observational only.

## Authority and limitations

This is a NEW derivative package for the current audit/repair. Historical observation and policy bytes remain unchanged. Legacy tranche, record, status, solver-version and CI strings are retained verbatim inside raw emitter output; `capture_provenance.json` explicitly disclaims historical acceptance replay or a completed current DEC025 sweep. Accepted decomposition 0.12/SCA009/DAG010 remains upstream authority; this capture is not decomposition truth, lifecycle closure, engineering acceptance, or release approval.

Owning DEL04-05 OUT001/RQ001-007 and AC001 require an observer-only, reproducible and provenance-preserving harness without invented release thresholds. The original TP-R4-D7 run records dirty local observations separately from downstream exit verification. DEC053 and its existing policy supply only the bounded parity (1e-9), aggregate residual (1e-6), exact repeat (0), and nonpositive pivot (0) checks used here. The accepted unit-normalized design is expressly unimplemented and cannot be backfilled from mixed-DOF aggregate maxima. No new quantity fields or thresholds were adopted. Applicable invariants: OPS-K-SOLVER-1, OPS-K-UNIT-1, OPS-K-MECH-1, OPS-K-REPORT-1, OPS-K-AGENT-1..4, OPS-K-IP-1.

## Handoff and reruns

Manager validation/acceptance remains required. Root must retain the final exact clean-source sweep/capture requirement as applicable; this dirty local capture does not replace it. Rerun if any captured emitter/dependency source changes. Physical/public policy holds remain unchanged: friction/history, finite-end connector contract, pressure force basis, absolute pivot/contact initialization policy, public DTOs/fields/migrations and new physics or production acceptance criteria. DEL04-05 formal review/dimensional basis, release thresholds and hosted-CI residuals remain open. No blocker to manager review of this bounded capture.
