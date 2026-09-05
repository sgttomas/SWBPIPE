# Bounded self-weight requirement mapping

Accepted upstream parent basis revision0.12/SCA009 row22/DAG010; this derivative implementation mapping does not replace DEL-05-01 ScopeOfWork.md or broaden DEL-05-02. No lifecycle transition.

| Bound requirement | Source implementation evidence | Check / outstanding acceptance |
|---|---|---|
| DEL-05-01 SOW-013 OBJ-003 OUT-001; SCA009 row22 | product_physics/src/self_weight.rs emits case shell + standard distributed_force primitive changes | Module tests; PKG16 batch/PKG07 interface integration pending |
| REQ-05-01-004 explicit units/dimensions | selected-pipe normalization with existing units catalog; gravity Acceleration, output N/m force_per_length | Alternate length/density units and signed axes tests |
| REQ-05-01-005 explicit missing/invalid findings | whole plan rejects missing/invalid mass/gravity/source and ambiguous refs, stale section cache, invalid geometry, collisions, overflow | Table-driven negative tests, no partial plan |
| REQ-05-01-009 deterministic verification | injective IDs, sorted selected pipes, immutable inputs, structured operation payloads | Module focused tests and full physics118 pass; fresh combined review pending |
| REQ-05-01-010 provenance | supported payload provenance and source_note strings preserve original request, pipe/section source and optional absence summary | Evidence serialization tests; wrapper full original hash and native/Wasm tests pending |
| AC-001 / VER-001 preserved mechanical/user-source boundary | explicit gravity and existing supplied densities; no factors/standards, component/point weights or auto-combinations | Design independent PASS, full source independent PASS required before manager acceptance |

DEL-05-02 algebra has no source changes. Parent owns operation transactionality, UI generation, dispatch gates and overall sweep. Generator pure output neither mutates nor approves a model. Optional absent contents/insulation is visible limited selected-pipe mass scope, not complete equipment weight.
