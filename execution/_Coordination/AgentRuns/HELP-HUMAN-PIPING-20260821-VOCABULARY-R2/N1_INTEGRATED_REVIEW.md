# N1 integrated independent code review

- RunID: `HELP-HUMAN-PIPING-20260821-VOCABULARY-R2`
- Role: fresh bounded read-only ephemeral Agent 2 software-code reviewer
- Base: `7d5a3f558dfa2e8e902df25fc9a3e813a9ab7048`
- Verdict: **FAIL — two actionable findings**
- Scope: 100% of the base-to-worktree product diff in the five authorized N1 paths; no product or Git-state mutation performed.

## Findings

### P1 — the new component kinds silently infer connectivity roles from model array order

`defaultComponentDraft` takes the first and second incident spans and assigns them directly to `primaryPipeRef` and `secondaryPipeRef` (`componentIntent.ts:38-55`). `componentDraftForNode` repeats the same assignment whenever the node changes (`componentIntent.ts:90-97`), while `componentDraftForKind` carries those values unchanged when bend is switched to tee/reducer/valve/flange (`componentIntent.ts:82-87`). For tee, those generic positions become the engineering-significant `branch_header_pipe_ref` and `branch_branch_pipe_ref`; for reducer/valve/flange the first span becomes `rigid_pipe_ref`.

Both visible forms expose selectors, but their selected values already come from this inference (`PropertyInspector.tsx:705-745`; `PipeViewport.tsx:1090-1133`). The new test helper confirms the behavior: it changes kind and tee node but never changes either pipe selector (`App.test.tsx:15048-15075`), then asserts that the inferred `pipe:P-100` / `pipe:P-110` mapping was emitted (`App.test.tsx:15083-15103`). Thus a user can queue and apply a payload described as explicit/user-entered without selecting which incident span is the tee header, which is the branch, or which is the rigid component span. At a node with three or more incident pipes, the result depends solely on `model.pipe_segments` ordering and can encode the wrong topology.

This violates the sealed N1 requirement for explicit geometry/connectivity and the no-inferred-engineering-default boundary. Preserve the already accepted bend behavior if required, but initialize/clear the new kinds' pipe-role fields until the user selects them. Add negative tests proving tee and rigid queueing stays disabled until the required pipe role(s) are explicitly selected, including a node with more than two incident spans.

### P2 — tee validity differs between the UI and authoritative operation applier

The UI requires the header and branch refs to differ (`componentIntent.ts:132-136`). The operation applier checks only that both strings are nonempty and that each resolves to a pipe incident to the node (`operation_applier/src/lib.rs:2088-2108`), then persists both refs (`operation_applier/src/lib.rs:2135-2137`). A structured-operation caller can therefore apply a tee whose header and branch fields name the same pipe, while both UI surfaces reject that identical payload.

This is an acceptance-contract and payload-parity gap at the authoritative seam. Enforce the distinct-reference invariant in the resolver with a negative Rust regression, or—if equality is intentionally valid under accepted domain authority—record that ruling and align the UI/tests. Until those layers agree, the claimed end-to-end tee contract is not closed.

## Confirmed review points

- The claimed-model-hash gate remains before component resolution (`operation_applier/src/lib.rs:1008-1014` before `1074-1086`).
- Application remains copy-on-write through the validated structured-operation path (`operation_applier/src/lib.rs:1316-1318`, `1370-1378`), and the outcome preserves `structured_operations_only` / no direct mutation (`1560-1565`).
- Canonical reconstruction trims and persists validated identity, reference, source, and provenance strings rather than returning the raw payload.
- Quantity payload keys and nested shapes otherwise match across intent construction and the resolver for tee, reducer, valve, and flange. Length, angle, force, and COG-length dimensions are checked in the resolver; no numeric geometry, weight, COG, stiffness behavior, or modifier value is synthesized there.
- The focused claimed checks reproduced: Rust creation slice `6 passed`; App Inspector/viewport matrix `10 passed`; scoped `git diff --check` passed. Passing tests do not cover either finding above.

## Exact reviewed inventory

All content SHA-256 values match `N1_ENGINE_CHECKS.json` and `N1_UI_CHECKS.json`.

| Path | Base diff (+/-) | Content SHA-256 | Binary diff SHA-256 |
|---|---:|---|---|
| `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs` | `+518/-61` | `847bca37ad991b24a2e7dbd6315c9294a0ac305f427b78628a36b3b1e177b51d` | `800cef40d5b9ef2e70310f7296226c562c05749455dcc93fc1a49656255c2480` |
| `projects/chirality-piping/apps/desktop/src/features/component-creation/componentIntent.ts` | `+197/-46` | `43dd7c07e807c5028e24878098b127a3bca7d947a982c44b9af3d96789d529d3` | `60d63ba2d211d1ba7e6e3b48a219fb95bec69d8116c571743a28f2c156763c63` |
| `projects/chirality-piping/apps/desktop/src/features/model-tree/PropertyInspector.tsx` | `+162/-82` | `ff2977446b090b46a4c3131fe252df750669a185a7e9139b6a6971d596fcea32` | `0ae123b305b6fe00ea33a56ee74021dac13f7f04cda76a74aa877895238e167d` |
| `projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport.tsx` | `+164/-92` | `4c208376cc98248aa7f8842f2ba2d07bd024898393814a4a1e9c73c032955474` | `3a671bc32a70bd6d5805d27f8844cc2bf0b878505f13fd86f8b08a7a9f700cb6` |
| `projects/chirality-piping/apps/desktop/src/App.test.tsx` | `+84/-60` | `5ba8d88f5f03f517ef3af571113208c95f5376404321ac0a0f5b4512144e6846` | `792ec590e011fcd6956c8f448c7e966a512c9828c9eb40ee082e54753a5010f4` |

Aggregate reviewed diff: `5 files`, `+1125/-341`. No other N1-coupled App test path differs from the base.

## Minimum repair acceptance checks

1. Inspector and viewport tests show tee/reducer/valve/flange pipe selectors start empty after selecting the kind and after changing the node; queueing remains disabled until the user changes the required selector(s).
2. A three-or-more-incident-pipe fixture proves the exact user-selected tee header/branch roles—not collection order—are serialized and applied. A rigid-kind case proves the exact selected `rigid_pipe_ref` is applied.
3. A Rust negative test sends equal `branch_header_pipe_ref` / `branch_branch_pipe_ref` and observes a blocking diagnostic with no `applied_model` (or cites an owner/domain ruling and aligns the UI if equality is intentionally accepted).
4. Rerun the focused Rust creation slice, rebuild the browser WASM, rerun the directly affected Inspector/viewport matrix plus the new negative cases, run the desktop build and scoped `git diff --check`, then obtain a new fresh 100%-diff read-only review.
5. Refresh the content/diff hash inventory in the new review; do not rely on the hashes frozen above after remediation.

## Fan-in disposition

N1 is not acceptable for commit/fan-in yet. Remediate both findings and send the resulting complete five-path diff to a new fresh read-only reviewer. N2 remains held because its shared-surface dependency requires accepted-and-committed N1.
