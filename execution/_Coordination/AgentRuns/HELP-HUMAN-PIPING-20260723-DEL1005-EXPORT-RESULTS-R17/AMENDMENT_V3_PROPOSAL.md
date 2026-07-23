---
doc_id: R17-DEL1005-AMENDMENT-V3-PROPOSAL
doc_kind: coordination.candidate_amendment
status: adopted_execution_released
created: 2026-07-23
candidate: CB-2026-07-23-DEL-10-05-EXPORT-RESULTS-001
supersedes_execution_basis: v2_only_if_owner_adopted
---

# Proposed sealed v3 remediation

This amendment responds only to N5's reproduced malformed-DTO diagnostic
finding. The owner adopted it through `OWNER_ADOPTION_V3.md`.

## Exact implementation scope

N4 may modify only:

1. `core/runner/headless/src/bin/openpipestress-runner.rs`
2. `tests/test_headless_runner_contract.py`
3. managed evidence below the R17 run root
4. exactly one new replacement `validation/evidence/sweeps/SWEEP_*.json`
   artifact, after all replacement pre-sweep gates pass

## Exact behavior

- Keep the top-level `export_results` member as raw JSON until the
  `export-results` handler.
- A missing member continues to return
  `HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_MISSING`, exit `1`.
- Deserialize a present member to the existing shared
  `ReportPackageRequest` inside the handler.
- Any DTO deserialization failure returns
  `HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_INVALID`, exit `1`, with a stable
  existing `REPORT-PACKAGE-WIRE-INCOMPLETE` prefix and the concrete serde
  detail in the diagnostic message.
- Every such failure returns no `report_package` payload and writes no output
  file.
- Add focused subprocess coverage for at least a missing required field and
  malformed field type.

## Explicit exclusions

No change is authorized to:

- the shared wire DTO or adapter;
- report-package assembler, members, schema, or encoding;
- redaction behavior or vocabulary;
- cross-binding semantics;
- runner verbs or the other four verb paths;
- benchmark/regression behavior;
- desktop code, picker, or atomic save;
- Cargo manifests or lockfiles;
- witnesses;
- decomposition, dependency, lifecycle, stage, release, issuance, or claims.

## Replacement gates

After the two-file remediation:

1. repeat Rust formatting, full report-package tests, full runner tests, and
   focused Python contract tests;
2. repeat deterministic malformed/success/failure output-file checks;
3. repeat the full registered pre-sweep union and applicable native runner
   proof;
4. validate exact containment and confirm no excluded path changed;
5. invoke exactly one owner-authorized replacement DEC-025 sweep;
6. freeze the tree and dispatch a fresh N5 verifier.

W3 remains blocked until the fresh N5 returns `COMMIT-SAFE`.
