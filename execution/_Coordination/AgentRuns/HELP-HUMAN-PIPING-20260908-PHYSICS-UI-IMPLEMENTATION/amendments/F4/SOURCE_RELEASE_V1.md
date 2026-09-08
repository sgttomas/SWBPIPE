# F4 source release amendment V1

**Status:** `SEALED_PENDING_ROOT_RELEASE`
**Manager:** F4 `WORKING_ITEMS`
**Model:** `gpt-5.6-sol`, high reasoning
**Basis:** source `55df51ac3201456e0f181823e3aefefef47a73bb`; adopted Owner act `../../OWNER_ACT.md`; shared contract `../../SHARED_SOURCE_LAUNCH_CONTRACT_V1.md`; frozen mechanics brief `../../../HELP-HUMAN-PIPING-20260908-PHYSICS-UI-EXECUTION/instances/F4/IMPLEMENTATION_BRIEF_V2.md` at SHA-256 `63c0361979c20474cddaada74076705a495a60da1c2d5cb0548a10ac6fd7974c`.

Implement exactly the frozen current-iterate derived-normal `+/- mu*N` correction. Preserve canonical SI, the public result shape, diagnostics vocabulary, sliding direction, history, conversion constants, convergence threshold, and outer cap semantics.

Allowed production/test source:

- `{WORKING_ROOT}/core/solver/nonlinear_integration/src/lib.rs`
- `{WORKING_ROOT}/core/product_physics/src/lib.rs`, test-only and only for the optional normalized product-boundary regression; no production code change

Allowed evidence/control writes:

- `{WORKING_ROOT}/execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/_run_records/PHYSICS_UI_IMPLEMENTATION_20260908/**`
- `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/F4/**`

No other source or deliverable is writable. The manager may dispatch bounded `gpt-5.6-sol` high Agent 2 work inside these fences and must persist each sealed child brief first. F4 owns the Rust compile slot until its source and focused checks freeze. Run the exact cases and mapped checks in the frozen V2 brief, freeze the complete diff and hashes, then return for a fresh independent 100% product-diff review. Stop on singular coupling, unresolved branch cycling at the existing cap, public-contract need, new tolerance/reference/history/direction choice, or fence expansion.

Source writes remain held until CHANGE Step 0 passes and root explicitly releases this sealed amendment after reading its hash.
