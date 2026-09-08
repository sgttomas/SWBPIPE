# K8 route-A V2 repair run record

Source: `779dedb8670625b36af07b89fc5557470e47c50e`. Original K8 manifest SHA-256: `a6ad4f8ab2881801a2ebabb483e5c22e4cafaa029e07cc398bf420013b576a65`. RK first-pass return SHA-256: `1e6deabb8b0ec0971b342ba0db47e5b80aad970ec2d36c408060d9a068e90e57`.

This additive successor resolves RK-001 through RK-003 without editing the frozen V1 packet or production code. It adds a schema revision that binds owner semantics, an executable semantic validator, writer/reader adversarial probes, an exact recommended K-A+K-U1 API contract, row-status interpretation rules, and explicit permission for lossless pending-pressure transport while retaining the PKG04/05 standardization gate.

Focused commands:

```text
uv run --with jsonschema python3 instances/K8/evidence/build_candidate_v2.py
uv run --with jsonschema python3 instances/K8/evidence/adversarial_tests_v2.py
```

Both passed. The complete nonlinear witness validates 830/830 with exact native source-record equality. Thirteen adversarial invariants pass at writer and reader boundaries, including cross-set duplicate ID, mirrored identity/value/unit/metadata/reference disagreement, ordered source-reference disagreement, NaN/infinity, unknown unit, schema/runtime owner mismatch, and row status taking precedence over `set_type`.

Public compatibility remains undecided. Recommendation remains K-A+K-U1. No production source, public schema, register, DAG, lifecycle pointer, or accepted pressure meaning changed.
