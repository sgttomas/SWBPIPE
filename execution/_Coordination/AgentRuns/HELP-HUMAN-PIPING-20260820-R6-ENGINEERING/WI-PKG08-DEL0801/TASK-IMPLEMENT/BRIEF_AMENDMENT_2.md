---
run_id: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
parent_instance_id: WI-PKG08-DEL0801
child_instance_id: TASK-IMPLEMENT
node: N2-I1
amendment: 2
status: frozen
reason: integrated review v4 exact-session basis mismatch
accepted_basis: 357a58b56726feba49507534159c3fbc4656b818
current_head: f8e0b496b223b847c82a6aba03b1b67e48048de9
---

# Amendment 2 — exact current-session evidence basis

The original brief, Amendment 1, writes, exclusions, production-boundary objective, and acceptance remain in force. Integrated review v4 found one exact proof defect:

`reportPackageRequest.test.ts` built mechanics result, input manifest, and analysis run from the original model, then cloned and removed `component:C-140` provenance only before calling `buildReportPackageRequest`. The manifest/hash evidence therefore bound a different model from the report provenance under test.

Required bounded remediation:

1. Mutate `component:C-140` provenance to missing before mechanics, input-manifest, and analysis-run construction.
2. Rebuild every dependent evidence object from that same mutated model; do not reuse any result, manifest, comparison, or analysis-run object derived from the original model.
3. Assert that the input-manifest payload/model basis itself carries the same missing provenance before comparing the production report output with the shared projection. Use the live manifest shape and exact source-backed field; do not invent a parallel marker.
4. Preserve the complete-root/version binding, Rust typed/version/unknown-field gates, shared fixture, present/missing/malformed assertions, private/pending classifications, and package-output proof.
5. Keep production unchanged unless the corrected exact-session proof exposes a real production defect. If it does, stop and return the concrete seam before widening scope.
6. Run focused Vitest, report-package Cargo tests and doc tests, Cargo fmt check, scoped containment, and diff check.
7. Return exact final N2 product/test paths and hashes frozen from original basis `357a58b...`; do not commit, edit DEL-08-01/manager/handoff/FAN_IN closeout, or touch N1/N3.

Write an immutable `AMENDMENT_RETURN_2.md` under the managed TASK folder. Do not alter earlier TASK run/return records.
