# U7 Repair Preparation V2

Status: `HELD_FOR_RU_TERMINAL_BINDINGS_AND_ROOT_RELEASE`; no product edit or Agent 2 dispatch is authorized by this draft.

Frozen input remains immutable evidence:

- final nine-path manifest V1 SHA-256 `9f71f16fe91112a42ed01fc69aee040665c4c1663d3e9d5a02d8df00747f4446`;
- exact V1 diff SHA-256 `df93c21cb945f6ed5d3e42ecbfd8f4f25ef462ba5c453d76757b36d2b2fa2056`;
- U7-R1 return SHA-256 `0307c1d9dff3e044176a0a1ece582aa85e08d9e38905cbc46a22bf79c3e0f962`;
- U7 manager return SHA-256 `e5b4e7ab6ad169a456b8ad0b0c2d02a233931f351770d74c7bd19254843c9471`.

Nonterminal RU findings supplied by root:

1. successful existing-end Apply with Continue selected loses the intended continuation/from state because the `PipeViewport` model-change effect runs after `finishAppliedRoute` and resets it;
2. delayed Apply currently leaves Cancel available; child-only generation invalidation cannot stop the already-started App batch from later publishing both records and a checkpoint;
3. a possible missing single-operation receipt is still under RU reproduction and is not yet admitted into repair scope.

Root clarification governs finding 2: the accepted contract requires implicated controls to be disabled while Add/Apply is in flight and requires canceled/inactive callbacks to fail closed. It does not require a new enabled in-flight cancellation feature. The final repair must test the actual permitted cancellation/invalidation paths and may not invent a new interaction contract.

The minimal successor remains inside the same nine source/test paths. It must add genuine App-level regressions for successful existing-end continuation, busy-control behavior plus no stale publication/checkpoint on permitted invalidation paths, and exact single-operation receipt behavior only if RU confirms that third finding. No threshold, default, schema, service, Rust/Tauri, global window, lifecycle, or dependency change is permitted.

Final RU packet path/hash/finding IDs and root source-release act remain `TBD_PENDING_RU_TERMINAL`. They must be sealed in a successor brief version before dispatch; this draft may not be used as launch authority.
