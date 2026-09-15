# Production UI foundation readiness

Accepted by HELP_HUMAN on 2026-09-15 under the approved foundation plan. The complete registered sweep and source/native evidence reviews have passed. This document prepares the next tranche; production UI implementation is outside the current foundation tranche.

## Decision and basis

HELP_HUMAN's decision is READY_FOR_FIRST_PRODUCTION_UI_FOUNDATION. Its technical prerequisites are satisfied; activate the prepared next tranche after Git closeout and fresh live-state validation. The four gates are stable typed operation/result interfaces, truthful Current/Historical behavior, a verified native modelling lifecycle, and an accepted current interface handoff.

The implementation basis is source 8ad37207cf088025623aa1e777a97a6fcb802f48, unchanged in clean sweep candidate a843e16090b58abef857c9eeacc407217b6c946d, which also includes the independently reviewed F12 dispatcher test repair. Paths below resolve within this AgentRuns directory unless stated otherwise.

- `instances/ROOT/_run_records/FINAL_SOURCE_RELEASE_V6.json` and `instances/ROOT/_run_records/FINAL_REVIEW_V6/REVIEW_RETURN_V6.json` bind the accepted source and independent review.
- `instances/ROOT/_run_records/FINAL_NATIVE_V4_FANIN_V1.json` binds actual author/solve/inspect/edit/history/save/normal quit/reopen/re-solve evidence, Current/Historical designation, malformed carriers and both legacy profiles.
- `instances/ROOT/_run_records/DEC025_V2/summaries/SWEEP_20260915T083057Z_a843e16090b5.json` (SHA-256 e002a678e07d87446a10c1494d0d3d890bab45763d76c61face615e13c49dfbc) binds the complete registered verification.
- `instances/ROOT/_run_records/UI_INTERFACE_SOURCE_BINDING_V1.json` (SHA-256 d6e40b7e497b4123e8fd64258a3535f01d7277ffa55c4b7e907aa57313b012aa) binds all eight current interface groups and supersedes stale source observations in the earlier preparation.
- `PRODUCTION_UI_FOUNDATION_CANDIDATE.md` defines the proposed next bounded implementation, delegation and file fence. The prior interaction prototype is simulated design evidence; it does not establish production behavior or personal visual approval.

## Accepted interface handoff

| Surface | Required shared behavior |
| --- | --- |
| Viewport and selection | The accepted model supplies geometry. Camera and selection are transient view state. Orbit and routing retain their existing movement guard, explicit working plane/axes/units and shared entity identity. |
| Model tree | Derive entities from the accepted model and use the same selection identity as the viewport and inspector. Do not create a second model authority. |
| Property inspector | Read current model values and send edits through typed intents/batches, preview/review and the Rust applier. Retain unit/target validation and synchronous busy/stale guards. |
| Tool palette and routing | Reuse the existing catalog and operation vocabulary. Node-plus-pipe creation stays atomic with one history checkpoint. Unavailable runtime functions remain unavailable until their owning engineering contracts exist. |
| History and persistence | Human controls and equivalent agent-authored intents produce the same exact model hash. Preserve attachments, project-replacement invalidation, undo/redo guards and save/reopen behavior. |
| Results and queries | Current results belong to their exact successful input basis. Reopened records remain Historical without the original input-manifest payload. Preserve raw rows, interpreted semantics, profile-specific findings, and separate mechanics-run and record-revision identity. Historical records cannot drive current overlays, comparisons, rules or readiness. |

The first production tranche should establish the dominant 3D workspace, coordinated model tree and inspector, accessible palette/search/keyboard controls and reviewed operation feedback described by the candidate. Its verification must carry the native cantilever lifecycle and exact human/agent model equivalence forward.

## Boundaries and parallel work

Use one WORKING_ITEMS implementation manager and one product writer, both Sol/high, with Agent 0 Astra/high for scope and integration, independent Astra/high review, Sol/high native verification and Sol/medium Git closeout. Keep no more than six active instances, Type 2 non-delegation and serialized builds, GUI witnesses and Git integration. Actual execution configuration and limitations must be recorded at dispatch.

Pressure runtime, objective connector mechanics and numerical/sparse work may proceed through separately owned briefs and write scopes. The first UI foundation need not wait for the entire physics backlog, vendor export readiness or harness integration. Changes that need the same product-physics files remain serialized. CAEPIPE and harness integration remain deferred; semantic equivalence remains required for every touched control.

This is a bounded interface/readiness decision. DEL-07-06 usability holds and whole-project lifecycle/dependency states remain unchanged. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
