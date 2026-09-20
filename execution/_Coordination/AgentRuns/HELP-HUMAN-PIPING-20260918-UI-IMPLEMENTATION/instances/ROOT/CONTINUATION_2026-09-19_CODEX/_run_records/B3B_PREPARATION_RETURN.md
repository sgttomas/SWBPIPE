Read-only preparation complete. wt3 is clean at `dada348294d21b8619b4644a5a690aedb039957d`. No writes, children, tests, builds, or UI use.

**Proposed B3B implementation**

| Work | Exact paths relative to `apps/desktop/` | Boundary |
|---|---|---|
| Synchronous project-operation ownership | `src/features/workspace/projectSessionState.ts`, `workspaceSession.ts` | One synchronous owner acquired before request counters or awaits; shared by New Local, New Blank, Open, Save, List. Only its owner releases busy. |
| Native parity | `src/services/nativeMenu.ts`, `src-tauri/src/lib.rs` | Add project-busy presentation state and disable those five File items. Controller guard remains authoritative against queued or same-turn dispatch. No Cmd-S addition. |
| Integrity observation | `src/types.ts`, `src/features/workspace/projectPersistenceIntegrity.ts`, `workspaceSession.ts` | Session-only source/time bound to the returned persisted snapshot and verified hashes. No persisted-envelope/schema change. |
| Display and downloads | `src/features/project-storage/ProjectStorageAuditPanel.tsx`, `src/features/project-validation/ProjectValidationPanel.tsx`, `src/App.tsx` | Carry both integrity observations consistently through panels and exported JSON. |
| Regression coverage | `src/App.projectHandlers.test.tsx`, narrowly identified cases in `src/App.test.tsx`, Rust native-menu tests in `lib.rs`; new focused helper/panel tests and `e2e/b3b-project-persistence.spec.ts` | Preserve unrelated integrity, normalization, history, and solved-only boundaries. |

Native parity is presently missing in both layers: `NativeShellState` lacks project-busy state; New/Open/Save handlers have no busy guard; List checks only rendered React state. A synchronous ownership ref closes the pre-render duplicate-dispatch window.

Save verification should reuse B3A’s canonical returned-model/envelope verification, same-project generation and fulfillment ordering. A newer local edit must retain its model/history while the observation describes the older snapshot that actually landed. Record verification observation time explicitly—not a backend commit timestamp. Failures preserve the previous observation; successful replacement clears/replaces it only at its commit boundary.

Historical model-hash carriers require separate truthful treatment: an accepted retained Historical claim is not canonical equality with the current model. Missing claims, recomputation failure, mismatches, invalid returned identity and unsupported normalization must never produce a verified-match label.

Two packet defects require explicit scope:
- Storage Audit currently displays model integrity but **omits it from its JSON packet**, and has no envelope-integrity prop.
- Project Validation serializes both records but labels every match `*_verified_on_open`; its summaries, diagnostics and visible lines must use actual source/time.

**Named test amendments**

In `App.projectHandlers.test.tsx`, the old “clears open-time verification” expectations at approximately lines 553, 586 and 613 become truthful landed-write observations, retaining newer-model protection. Busy-overlap setups around 308, 528, 740, 790, 821, 991 and 1024 must be explicitly reconsidered: native parity prevents those second project commands from starting.

Preserve their ownership guarantees through focused helper/controller regressions where the original race becomes unreachable—especially delayed verification, later valid versus invalid observations, same-ID replacement, and Undo/Redo against persisted contents. Do not merely remove them. The mixed batch/save/replacement test in `App.test.tsx` around 16601 also needs a named setup amendment while retaining stale batch/publication assertions.

Add same-turn double dispatch, all five blocked commands, List ownership, failed/missing completion, successful release, pending-hash busy duration, and native-state synchronization tests. Retain failure, normalization and Historical regressions.

**Delegation after activation**

Use one fresh Astra/low implementation TASK for the cohesive B3B path set above, with manager as sole integrator and no concurrent shared-file edits. Then use a separate fresh Astra/low native witness TASK against a frozen build. ROOT retains independent review and complete sweep.

**Native observability**

No existing safe deterministic project-operation gate was found. `projectService.ts` invokes normal native operations directly; the explicit five-second Rust timeout belongs to legacy-store copying and is unsuitable for this witness.

First attempt the uninstrumented synthetic-row native path and capture actual menu enablement if observable. If its busy interval cannot be captured reliably, return that limit and seek ROOT’s bounded approval for an isolated probe-only completion gate with explicit release, source/binary identity and synthetic project custody. No production delay, timeout inflation, debug command, shared-store locking or private-row mutation. Probe evidence must remain distinct from final uninstrumented save/source/time evidence.

**C3 handoff**

C3’s proposed separation is compatible, but these shared files must remain shell-owned: `selectionSessionState.ts`, `workspaceSession.ts`, `App.tsx`, `styles.css`, and their shell tests.

The interface should carry:
- Explicit hidden keys separately from nullable isolation-selected keys.
- Derived true `hiddenKeys`, separate `dimmedKeys`, isolation-active state and total hidden count.
- Isolate snapshot and Show All callbacks; project replacement clears both states, stage/view changes retain them.

Canvas owns `viewportVisibility.ts` and its tests, `PipeViewport.tsx`, and resource/material changes. Picking implementation/tests remain unchanged. Hidden wins; dimmed entities remain pickable; selection does not reveal hidden entities.

Recommended ordering: freeze B3B’s shared-controller checkpoint first, then apply the exact C3 shell-interface patch through this manager against that checkpoint. Canvas may prepare disjoint code against the agreed typed interface, but cannot edit the shared files concurrently. No performance benchmark is part of either assignment.

