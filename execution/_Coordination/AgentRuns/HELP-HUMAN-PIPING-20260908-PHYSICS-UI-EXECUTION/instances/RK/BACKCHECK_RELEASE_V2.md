# RK backcheck release V2

Status: SEALED. Existing RK pre-review scope and permissions remain unchanged. This is a versioned successor backcheck release from `/root` HELP_HUMAN.

- Successor manifest: `{RUN_ROOT}/instances/K8/MANIFEST_V2.json`
- Required SHA-256: `ee4fcf91cf7eb818b2f85403c4819322df24d6070c94f62b7aeaee70e875725a`
- Expected successor inventory: 14/14 outputs
- V1 manifest remains frozen at SHA-256 `a6ad4f8ab2881801a2ebabb483e5c22e4cafaa029e07cc398bf420013b576a65`; V1 outputs remain unchanged.
- Review objective: read all 14 V2 outputs, compare the successor conceptually with V1, reproduce `build_candidate_v2.py` and `adversarial_tests_v2.py` in isolation, and independently determine closure of RK-001 through RK-003.
- Required closure checks: mandatory validator invocation at writer and reader boundaries; row `semantic_status` dominance; exact API/DTO/refusal/default/version-registry/desktop-consumption candidate design; pressure sequencing.
- Root design clarification: lossless pending native pressure transport may remain a `preserve_only` candidate; P5 gates standardized meaning or status change, not exact-preservation transport.
- Public/API/schema choices remain candidates. This release authorizes no source, production schema, public adoption, physics acceptance, lifecycle, Git, or publishing action.
- Durable writes remain limited to `{RUN_ROOT}/instances/RK/**`. No delegation. No direct K8 messaging; root mediates.
- Validation is focused only: no full harness, product tests, Cargo/native build, or source writes.
