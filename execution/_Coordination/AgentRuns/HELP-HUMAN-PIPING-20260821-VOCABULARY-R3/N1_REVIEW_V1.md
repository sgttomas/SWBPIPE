# N1 integrated independent code review — round 3, review v1

- RunID: `HELP-HUMAN-PIPING-20260821-VOCABULARY-R3`
- Role: fresh bounded read-only ephemeral Agent 2 software-code reviewer
- Accepted main basis: `66efaf6b95605ef69f3e405b505f48506d3cbada`
- Pre-repair lineage: `2bee267300e571e4e8686f73aba6ad4ba8be4c54`
- Reviewed state: complete five-file base-to-current-worktree product diff
- Verdict: **FAIL — one actionable P1 residual; repair cycle 1 required**
- Mutation boundary: no product or Git mutation; this review record is the only authorized write.

## Finding

### P1 — a queued non-bend component silently initializes the next draft's pipe roles from collection order

The kind-change and node-change paths now correctly clear tee and rigid-family
pipe selectors: `componentDraftForKind` clears both non-bend refs
(`componentIntent.ts:82-94`), and `componentDraftForNode` does the same after a
node change (`componentIntent.ts:97-104`). The Inspector and viewport selectors
are controlled by those fields (`PropertyInspector.tsx:690-745`;
`PipeViewport.tsx:1075-1133`).

However, `defaultComponentDraft` still unconditionally assigns the first and
second incident spans to `primaryPipeRef` and `secondaryPipeRef`
(`componentIntent.ts:38-55`), even when its explicit `kind` argument is `tee`,
`reducer`, `valve`, or `flange`. Both successful queue handlers preserve the
current kind and create the next draft through exactly that function:

- Inspector: `PropertyInspector.tsx:204-209`
- viewport: `PipeViewport.tsx:501-513`

The result is a second same-kind draft whose visible selectors already carry
array-order-derived connectivity. Its other fields reset empty, so the queue is
initially disabled; once the user fills those non-pipe fields, the current
validity checks accept the inferred incident refs (`componentIntent.ts:139-167`)
without requiring any selector interaction for the new draft. For tee, the
first/second incident entries become header/branch roles. For reducer, valve,
and flange, the first entry becomes `rigid_pipe_ref`.

The new negative matrix covers clearing immediately after kind selection and
node change (`App.test.tsx:15106-15170`), but stops before queueing and therefore
does not exercise the reset path. The positive queue matrix clicks once and
only checks queue count (`App.test.tsx:15172-15205`); it likewise does not assert
the next draft's selector state. This leaves the original no-inferred-role
requirement open for ordinary repeated creation in both product surfaces.

Impact: a user can create the second tee/reducer/valve/flange at a node without
deliberately selecting its connectivity role(s), and at a three-or-more-incident
node the serialized topology again depends on `model.pipe_segments` order.

## Minimum repair matrix — repair cycle 1

1. Make `defaultComponentDraft` initialize `primaryPipeRef` and
   `secondaryPipeRef` empty for every non-bend kind. Preserve the accepted bend
   initialization behavior. Equivalently, route both post-queue resets through
   a helper with that exact kind-sensitive contract; do not duplicate divergent
   policy in the two UI surfaces.
2. Add Inspector and viewport regressions that queue a tee and a rigid-family
   component, observe the next same-kind draft, fill all non-pipe fields, and
   prove the selectors remain empty and queueing remains disabled until the user
   deliberately selects the required role(s). Include a three-or-more-incident
   node in the tee reset case and prove the subsequent preview/application uses
   the exact selected roles rather than collection order.
3. Rerun the focused UI matrix, browser WASM rebuild, exact-final desktop suite,
   desktop build, and scoped `git diff --check`; then obtain a fresh 100%-diff
   read-only review with a new content/diff inventory. P2 needs no semantic
   redesign, but its regression remains in the rerun slice.

## Confirmed review points

- **P2 repaired.** The operation applier compares trimmed tee header and branch
  refs after required-field validation and before lookup
  (`operation_applier/src/lib.rs:2088-2118`). Equality sets reference validation
  to blocked, emits `OP-COMPONENT-CONNECTIVITY-INVALID`, and returns no canonical
  component. The negative regression verifies the blocking diagnostic,
  `reference_validation=blocked`, `application_status=blocked`, and no
  `applied_model` (`operation_applier/src/lib.rs:11336-11356`).
- **Exact selections work when deliberately made.** The three-incident test
  creates `pipe:P-150`, selects header `pipe:P-110` and branch `pipe:P-150`,
  checks both serialized keys, applies the operation, and observes the same
  mapping (`App.test.tsx:15239-15294`). Rigid-family positive cases select
  `pipe:P-120`, not the fixture's first incident pipe
  (`App.test.tsx:15067-15078`, `15197-15199`, `15207-15236`).
- **Bend behavior preserved.** Bend still initializes its incident span on
  default/kind/node transitions (`componentIntent.ts:48-55`, `82-104`), uses
  the unchanged bend payload keys (`componentIntent.ts:241-249`), and passes
  the Inspector/viewport positive regressions.
- **Payload parity otherwise holds.** Intent construction emits tee fields at
  `componentIntent.ts:251-261` and rigid-family fields at `263-279`; the applier
  consumes and canonically reconstructs the same shapes at
  `operation_applier/src/lib.rs:2047-2287`. Required quantities, units,
  references, and source/provenance strings are explicit and no geometry or
  connectivity value is synthesized by the resolver.
- **Claimed-model-hash gate preserved.** Hash checking remains at
  `operation_applier/src/lib.rs:1008-1014`, before component resolution at
  `1074-1086`. A mismatch contributes a blocking diagnostic, so apply cannot
  enter the `!blocking` branch.
- **Structured-operation-only mutation preserved.** Apply clones the model and
  inserts the resolved canonical component only in the apply branch
  (`operation_applier/src/lib.rs:1316-1378`). The outcome continues to report
  `structured_operations_only`, no direct mutation, unchanged input, and a new
  applied document (`1538-1566`).
- **Scope contained.** The product diff is exactly the five authorized paths:
  `5 files`, `+1314/-333`; no other N1-coupled App test path differs from the
  accepted basis. Aggregate base-to-current binary-diff SHA-256:
  `5441ee541bfe1b1b18ec080724aabae758e3bf78c18fb170b076a6f7857cbf10`.

## Pre-repair lineage comparison

The product blobs at pre-repair commit
`2bee267300e571e4e8686f73aba6ad4ba8be4c54` exactly match all five content
hashes frozen by the immutable R2 review. The accepted R3 main basis has the
same five base blobs as R2 basis `7d5a3f558dfa2e8e902df25fc9a3e813a9ab7048`,
so each base-to-pre binary diff also exactly reproduces the R2 inventory.

| Path | R2/pre-repair content SHA-256 | Lineage result | R2/base-to-pre binary diff SHA-256 | Diff result |
|---|---|---|---|---|
| `core/model_operations/operation_applier/src/lib.rs` | `847bca37ad991b24a2e7dbd6315c9294a0ac305f427b78628a36b3b1e177b51d` | MATCH | `800cef40d5b9ef2e70310f7296226c562c05749455dcc93fc1a49656255c2480` | MATCH |
| `apps/desktop/src/features/component-creation/componentIntent.ts` | `43dd7c07e807c5028e24878098b127a3bca7d947a982c44b9af3d96789d529d3` | MATCH | `60d63ba2d211d1ba7e6e3b48a219fb95bec69d8116c571743a28f2c156763c63` | MATCH |
| `apps/desktop/src/features/model-tree/PropertyInspector.tsx` | `ff2977446b090b46a4c3131fe252df750669a185a7e9139b6a6971d596fcea32` | MATCH | `0ae123b305b6fe00ea33a56ee74021dac13f7f04cda76a74aa877895238e167d` | MATCH |
| `apps/desktop/src/features/viewport/PipeViewport.tsx` | `4c208376cc98248aa7f8842f2ba2d07bd024898393814a4a1e9c73c032955474` | MATCH | `3a671bc32a70bd6d5805d27f8844cc2bf0b878505f13fd86f8b08a7a9f700cb6` | MATCH |
| `apps/desktop/src/App.test.tsx` | `5ba8d88f5f03f517ef3af571113208c95f5376404321ac0a0f5b4512144e6846` | MATCH | `792ec590e011fcd6956c8f448c7e966a512c9828c9eb40ee082e54753a5010f4` | MATCH |

## Exact current reviewed inventory

Binary-diff hashes are SHA-256 over
`git diff --binary 66efaf6b95605ef69f3e405b505f48506d3cbada -- <path>`.

| Path | Base diff (+/-) | Current content SHA-256 | Base-to-current binary diff SHA-256 |
|---|---:|---|---|
| `core/model_operations/operation_applier/src/lib.rs` | `+553/-61` | `f6ecdc476482d8798591689770c4a87a318ec4765e0362b886ae46922b58be7f` | `d88d79619f5f6e6f0caf5cfc0ef0a7fb22bf075b05e350c948bbf53f3b15bef1` |
| `apps/desktop/src/features/component-creation/componentIntent.ts` | `+204/-46` | `5c191db66a7c6d4e61bbdddbb549d3d4ebc2c3a2927a7d75ea83f52468025859` | `d2b68dd53a96ad68a70b133c8fba5764a814f08464b10d119a999c14fe95ba71` |
| `apps/desktop/src/features/model-tree/PropertyInspector.tsx` | `+162/-82` | `07162a3ba9d2a43b9a3a95f9b1af9b891559149d7fd408044250caa9a394b14f` | `e30ac162e120f15c6dd3fc6c4a4f0406e8e288d8d5960434de98d9e0a8f28f6b` |
| `apps/desktop/src/features/viewport/PipeViewport.tsx` | `+164/-92` | `ae5e770d512508859bf340401b49b13d322bbdf53071e0c940a58a217002bb4a` | `50782a01714c595f3d40c7a81eaac472e76b80589ec420203e5c6daa92f29903` |
| `apps/desktop/src/App.test.tsx` | `+231/-52` | `a8938b4a96c9a746b1057c41a005f457aaad406436d18de7798a284a14d98c14` | `d5fb30d4839ea693f25733f17156a3f53ce47c35267abbbb6bf874eedca14dcd` |

## Checks independently reproduced

| Check | Result |
|---|---|
| `cargo test --offline --manifest-path projects/chirality-piping/core/model_operations/operation_applier/Cargo.toml --lib creation` | PASS — `7 passed`, `0 failed`, including equal-ref/no-applied-model regression |
| `npm test --workspace apps/desktop -- App.test.tsx -t 'pipe roles empty\|queues explicit (tee\|reducer\|valve\|flange\|bend)\|creates a (tee\|reducer\|valve\|flange\|bend)\|three incident spans'` | PASS — `19 passed`, `0 failed`, `67 skipped` |
| `npm run build:desktop` | PASS — TypeScript and Vite build; only standing chunk-size advisory |
| `git diff --check 66efaf6b95605ef69f3e405b505f48506d3cbada -- <five reviewed paths>` | PASS — no whitespace errors |

The passing focused UI matrix does not cover the post-queue reset path described
in the finding. The recorded manager evidence for browser-WASM rebuild, full
desktop suite (`558` tests), full operation-applier suite (`85` tests), and
formatting was inspected but not redundantly rerun by this reviewer.

## Fan-in disposition

**FAIL.** P2 and the deliberate-selection paths pass, but N1 is not acceptable
for commit/fan-in while repeated non-bend creation can reintroduce inferred
roles. Apply the minimum repair matrix, preserve this failed review and hash
inventory, and send the complete refreshed five-file diff to a fresh reviewer
for repair-cycle-1 review. N2 remains held.
