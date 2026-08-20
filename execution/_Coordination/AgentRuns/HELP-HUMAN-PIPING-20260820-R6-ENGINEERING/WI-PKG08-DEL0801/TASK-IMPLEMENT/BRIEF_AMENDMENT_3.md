---
run_id: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
parent_instance_id: WI-PKG08-DEL0801
child_instance_id: TASK-IMPLEMENT
node: N2-I1
amendment: 3
status: frozen
reason: integrated review v8 production identity defect
accepted_basis: 357a58b56726feba49507534159c3fbc4656b818
current_head: 3662143bc9558d2da32e74e068768ba81edc0a74
---

# Amendment 3 — production model/manifest payload identity gate

The original brief and Amendments 1–2 remain in force. Integrated review v8 found a production defect in `buildReportPackageRequest`: after internally verifying the input manifest, it binds separately supplied `model` and `result` only by project ID. A stale or different model payload with the same ID can therefore drive rendering while the analysis/input-manifest evidence attests another model payload.

This defect belongs to the authorized exact cross-layer current-session identity seam. Production write authority is explicitly confirmed for:

- `apps/desktop/src/features/report/reportPackageRequest.ts`
- `apps/desktop/src/features/report/reportPackageRequest.test.ts`

Required remediation:

1. Before `buildRenderableReportInput` or any report/package assembly, deterministically prove canonical equality/hash equality between the supplied `model` and `inputManifest.manifest.model_basis.model_payload` using the existing canonical hash/canonical JSON service. Same-ID payload differences must fail closed with one stable report-package error code.
2. Preserve the existing internal manifest verification, model/result reference check, analysis-run manifest ref/hash binding, and all prior provenance/fixture/Rust gates. Do not add a second canonicalization implementation.
3. Add a negative regression where the supplied model keeps the same project ID but differs from the manifest model payload; it must reject before rendering/package construction.
4. Add ordering/canonicalization regression evidence showing semantically identical model payloads with reordered object keys remain equal under the canonical service, while changed values reject.
5. Preserve positive current-session and missing-provenance paths. Strengthen assertions as needed so the gate is proven on the live production function.
6. Keep all changes minimal and confined to the report-package identity seam. No schema, dependency, public-policy, fixture-content, Rust behavior, lifecycle, N1/N3, Git, receipt, or PR change.
7. Run focused reportPackageRequest Vitest; registered desktop build/typecheck if available; report-package Cargo tests/doc tests/fmt; scoped containment; and basis-scoped diff check.
8. Freeze the complete amended N2 product/test diff from original basis `357a58b...`, including production TS, test TS, Rust, and fixture, with exact hashes. Write immutable `AMENDMENT_RETURN_3.md`. Do not update DEL-08-01 manager/handoff/FAN_IN unless the subsequent fresh review passes.

If canonical comparison cannot run at this boundary without new production dependency or authority, stop and return the exact blocker instead of inventing a fallback.
