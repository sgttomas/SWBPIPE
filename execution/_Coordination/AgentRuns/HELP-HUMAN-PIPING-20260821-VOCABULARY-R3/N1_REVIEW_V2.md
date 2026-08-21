# N1 integrated independent code review — round 3, review v2

- RunID: `HELP-HUMAN-PIPING-20260821-VOCABULARY-R3`
- Role: fresh bounded read-only ephemeral Agent 2 software-code reviewer
- Accepted main basis: `66efaf6b95605ef69f3e405b505f48506d3cbada`
- Pre-repair lineage: `2bee267300e571e4e8686f73aba6ad4ba8be4c54`
- Reviewed state: 100% of the complete five-file basis-to-current-working-blob product diff after repair cycle 1
- Prior review: immutable `N1_REVIEW_V1.md`, SHA-256 `582ad72b35e4a95772e581df838765b5801b8cc8fc19018adabfc422c5231bd3`
- Verdict: **PASS — no actionable findings; N1 accepted for commit/fan-in**
- Mutation boundary: no product, Git, shared-state, status, coverage, receipt, or prior-review mutation; this review record is the only write.

## Findings

No actionable findings were found in the complete five-file diff. The repair-cycle-1
change closes review v1's residual post-queue inference path in both product
surfaces, and the P2 resolver repair continues to satisfy the owner-directed
distinct-role contract.

## Owner five-point repair matrix

1. **Empty roles after kind selection — PASS.** `componentDraftForKind` assigns an
   incident pipe only for bend and clears both non-bend role fields
   (`componentIntent.ts:82-94`). Inspector and viewport route their kind selectors
   through that helper (`PropertyInspector.tsx:690-703`;
   `PipeViewport.tsx:1075-1088`). The negative matrix covers tee, reducer, valve,
   and flange in both surfaces.
2. **Empty roles after node change — PASS.** `componentDraftForNode` assigns an
   incident pipe only for bend and clears both non-bend fields
   (`componentIntent.ts:97-104`). Both forms and viewport node picking use that
   helper. The negative matrix fills every non-pipe field and proves queueing stays
   disabled until the required role selection(s) are deliberately changed.
3. **Empty roles after successful queue — PASS; review-v1 residual closed.** The
   shared `defaultComponentDraft` now initializes `primaryPipeRef` from collection
   order only when `kind === "bend"`; every non-bend draft starts with empty primary
   and secondary roles (`componentIntent.ts:38-55`). The Inspector and viewport
   preserve the selected kind while resetting through that same helper
   (`PropertyInspector.tsx:204-209`; `PipeViewport.tsx:501-513`). Direct tee and
   valve post-queue regressions in both surfaces prove the next same-kind draft has
   empty selectors and remains disabled after all non-pipe fields are filled.
4. **Exact selected roles, including three incident spans — PASS.** Intent
   construction serializes the draft's selected tee header/branch fields verbatim
   (`componentIntent.ts:251-260`) and the selected rigid span verbatim
   (`componentIntent.ts:263-279`). The three-incident regression realizes
   `pipe:P-150`, then on the post-queue draft selects header `pipe:P-110` and branch
   `pipe:P-150` rather than the collection's first/second entries, verifies both
   serialized keys, applies the structured operations, and observes the same two
   roles on applied `component:C-2` (`App.test.tsx:15303-15378`). Rigid-family
   positives select and apply `pipe:P-120`, not the first incident span. Bend keeps
   its accepted incident-span initialization and payload behavior.
5. **Authoritative distinct tee-role invariant — PASS.** The operation applier trims
   both required refs, rejects equality before lookup with blocking diagnostic
   `OP-COMPONENT-CONNECTIVITY-INVALID`, and returns no canonical component
   (`operation_applier/src/lib.rs:2047-2123`). The Rust regression asserts the
   diagnostic, `reference_validation=blocked`, `application_status=blocked`, and no
   `applied_model` (`operation_applier/src/lib.rs:11336-11356`).

## Complete semantic review

- Tee, reducer, valve, and flange remain allowlisted only through their explicit
  kind-specific resolver contracts. Required geometry, quantities, units,
  connectivity refs, source refs, and provenance are validated and canonically
  reconstructed; no pipe role or engineering value is synthesized by the applier.
- Payload parity holds between `componentIntent.ts` and the operation-applier
  resolver for bend, tee, and the rigid family. All five kinds have Inspector intent
  coverage and viewport apply coverage.
- The claimed-model-hash check remains ahead of component resolution and contributes
  a blocking diagnostic on mismatch. Apply remains copy-on-write and only inserts a
  resolved canonical component inside the non-blocking structured-operation apply
  branch. The outcome continues to report `structured_operations_only`, no direct
  mutation, unchanged input, and a new applied document.
- Scope is exactly the five authorized product paths: `5 files`, `+1399/-333`.
  No other product path differs from the accepted basis.

## Pre-repair lineage verification

Commit `2bee267300e571e4e8686f73aba6ad4ba8be4c54` has parent
`66efaf6b95605ef69f3e405b505f48506d3cbada` and the required WIP lineage message.
Every lineage blob and every basis-to-lineage binary-diff hash exactly matches the
immutable R2 review inventory.

| Path | R2/pre-repair content SHA-256 | Basis-to-pre binary diff SHA-256 | Result |
|---|---|---|---|
| `core/model_operations/operation_applier/src/lib.rs` | `847bca37ad991b24a2e7dbd6315c9294a0ac305f427b78628a36b3b1e177b51d` | `800cef40d5b9ef2e70310f7296226c562c05749455dcc93fc1a49656255c2480` | MATCH |
| `apps/desktop/src/features/component-creation/componentIntent.ts` | `43dd7c07e807c5028e24878098b127a3bca7d947a982c44b9af3d96789d529d3` | `60d63ba2d211d1ba7e6e3b48a219fb95bec69d8116c571743a28f2c156763c63` | MATCH |
| `apps/desktop/src/features/model-tree/PropertyInspector.tsx` | `ff2977446b090b46a4c3131fe252df750669a185a7e9139b6a6971d596fcea32` | `0ae123b305b6fe00ea33a56ee74021dac13f7f04cda76a74aa877895238e167d` | MATCH |
| `apps/desktop/src/features/viewport/PipeViewport.tsx` | `4c208376cc98248aa7f8842f2ba2d07bd024898393814a4a1e9c73c032955474` | `3a671bc32a70bd6d5805d27f8844cc2bf0b878505f13fd86f8b08a7a9f700cb6` | MATCH |
| `apps/desktop/src/App.test.tsx` | `5ba8d88f5f03f517ef3af571113208c95f5376404321ac0a0f5b4512144e6846` | `792ec590e011fcd6956c8f448c7e966a512c9828c9eb40ee082e54753a5010f4` | MATCH |

## Refreshed current inventory

Binary-diff hashes are SHA-256 over
`git diff --binary 66efaf6b95605ef69f3e405b505f48506d3cbada -- <path>`.

| Path | Basis diff (+/-) | Current content SHA-256 | Basis-to-current binary diff SHA-256 |
|---|---:|---|---|
| `core/model_operations/operation_applier/src/lib.rs` | `+553/-61` | `f6ecdc476482d8798591689770c4a87a318ec4765e0362b886ae46922b58be7f` | `d88d79619f5f6e6f0caf5cfc0ef0a7fb22bf075b05e350c948bbf53f3b15bef1` |
| `apps/desktop/src/features/component-creation/componentIntent.ts` | `+204/-46` | `19ebce4104c8b7900a004526c71769bc4ce8cfec0036342f5cdd643bb47e98d6` | `235ae9341329752ee572a7e042d8da3ec6139e709bdb28352624083da2011617` |
| `apps/desktop/src/features/model-tree/PropertyInspector.tsx` | `+162/-82` | `07162a3ba9d2a43b9a3a95f9b1af9b891559149d7fd408044250caa9a394b14f` | `e30ac162e120f15c6dd3fc6c4a4f0406e8e288d8d5960434de98d9e0a8f28f6b` |
| `apps/desktop/src/features/viewport/PipeViewport.tsx` | `+164/-92` | `ae5e770d512508859bf340401b49b13d322bbdf53071e0c940a58a217002bb4a` | `50782a01714c595f3d40c7a81eaac472e76b80589ec420203e5c6daa92f29903` |
| `apps/desktop/src/App.test.tsx` | `+316/-52` | `19d1aa176a447a3d6c491164cba007471a8e7cbc819b8141a485028e89a396cb` | `b89cad63145d2cf3c4b22adf00165cc40e74c6309049b233147175ad61a6f64f` |

Aggregate basis-to-current binary-diff SHA-256:
`5445faa5739064714e98ce11d8cb8c20d68d9ab55dd88e06cab4231b9b6505d3`.

## Independently reproduced checks

| Check | Result |
|---|---|
| `cargo test --offline --manifest-path projects/chirality-piping/core/model_operations/operation_applier/Cargo.toml --lib creation` | PASS — `7 passed`, `0 failed`, including equal-role/no-applied-model regression |
| `npm test --workspace apps/desktop -- App.test.tsx -t '(pipe roles empty\|pipe roles after queue\|queues explicit (tee\|reducer\|valve\|flange\|bend)\|creates a (tee\|reducer\|valve\|flange\|bend)\|three-incident)'` | PASS — `23 passed`, `0 failed`, `67 skipped` |
| `npm run build:desktop` | PASS — TypeScript and Vite build; only standing chunk-size advisory |
| `cargo fmt --manifest-path projects/chirality-piping/core/model_operations/operation_applier/Cargo.toml -- --check` | PASS — no formatting diff |
| `git diff --check 66efaf6b95605ef69f3e405b505f48506d3cbada -- <five reviewed paths>` | PASS — no whitespace errors |

The manager's current-blob evidence was also inspected for the browser-WASM rebuild
(PASS), exact-final desktop suite (`29` files / `562` tests, PASS), full
operation-applier suite (`85` tests, PASS), and practitioner self-check (exit `0`).
Those broader checks were not redundantly rerun by this reviewer.

## Fan-in disposition

**PASS.** Repair cycle 1 closes the only review-v1 residual. N1 now satisfies the
owner's complete minimum repair matrix for tee, reducer, valve, and flange while
preserving bend behavior. Accept the five current product blobs for the N1 commit;
row 15 may close only after the normal landed-evidence and final shared-state gates.
N2 may be released only after HELP_HUMAN confirms this review and N1 is committed.
