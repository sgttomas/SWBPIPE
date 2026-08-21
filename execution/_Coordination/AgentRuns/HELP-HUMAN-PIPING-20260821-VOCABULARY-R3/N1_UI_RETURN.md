# N1 UI repair-cycle-1 manager return — vocabulary completion round 3

- RunID: `HELP-HUMAN-PIPING-20260821-VOCABULARY-R3`
- InstanceID: `WORKING-ITEMS-VOCAB-R3-N1-UI`
- Role: `WORKING_ITEMS`
- Package: `PKG-07`
- Selected deliverables: `DEL-07-01`, `DEL-07-02`
- Accepted main basis: `66efaf6b95605ef69f3e405b505f48506d3cbada`
- Pre-repair lineage: `2bee267300e571e4e8686f73aba6ad4ba8be4c54`
- Verdict: `REPAIR_CYCLE_1_READY_FOR_FRESH_REVIEW`

## Failure lineage and bounded repair

Round 2's immutable integrated review failed P1 because tee, reducer, valve,
and flange could inherit pipe roles from incident-span collection order. The
initial round-3 repair cleared those roles on kind and node transitions, but
the immutable round-3 `N1_REVIEW_V1.md` found one remaining P1 path: after a
successful Inspector or viewport queue, `defaultComponentDraft` retained the
same kind while reinitializing its pipe roles from the incident array.

Repair cycle 1 closes only that enumerated residual. The shared
`defaultComponentDraft` policy now initializes incident-pipe references only
for `bend`. A new `tee`, `reducer`, `valve`, or `flange` draft therefore keeps
its same kind after queue but begins with empty required pipe-role selectors.
Both UI surfaces remain disabled after all non-pipe fields are complete until
fresh deliberate selections are made. Bend behavior and all frozen operation
payload keys are unchanged.

Direct post-queue regressions cover tee and valve as the rigid-family
representative in both Inspector and viewport. The tee reset case realizes a
third span, queues a first tee, verifies the next same-kind draft has empty
roles, then deliberately selects header `pipe:P-110` and branch `pipe:P-150`.
Its preview and applied `C-2` model retain exactly those roles. Existing
reducer/valve/flange exact-span cases continue to select `pipe:P-120`.

No resolver, shared surface, status, coverage, receipt, Git state, or
engineering default was changed by this instance.

## Preserved failure and hash evidence

The complete failed review-v1 inventory remains frozen in
`N1_UI_CHECKS.json`, including:

- review-v1 aggregate base-to-current binary-diff SHA-256
  `5441ee541bfe1b1b18ec080724aabae758e3bf78c18fb170b076a6f7857cbf10`;
- all five review-v1 content and per-file binary-diff hashes;
- all five R2/pre-repair content hashes; and
- both the R2 P1 failure and the round-3 post-queue P1 residual.

Repair-cycle-1 owned-file transitions are:

| Path | Review-v1 SHA-256 | Post-cycle-1 SHA-256 | Cycle 1 +/- |
|---|---|---|---:|
| `apps/desktop/src/features/component-creation/componentIntent.ts` | `5c191db66a7c6d4e61bbdddbb549d3d4ebc2c3a2927a7d75ea83f52468025859` | `19ebce4104c8b7900a004526c71769bc4ce8cfec0036342f5cdd643bb47e98d6` | `+1/-1` |
| `apps/desktop/src/App.test.tsx` | `a8938b4a96c9a746b1057c41a005f457aaad406436d18de7798a284a14d98c14` | `19d1aa176a447a3d6c491164cba007471a8e7cbc819b8141a485028e89a396cb` | `+85/-0` |

The unchanged owned UI blobs remain:

- `PropertyInspector.tsx`:
  `07162a3ba9d2a43b9a3a95f9b1af9b891559149d7fd408044250caa9a394b14f`
- `PipeViewport.tsx`:
  `ae5e770d512508859bf340401b49b13d322bbdf53071e0c940a58a217002bb4a`

The refreshed complete five-file base-to-current diff is `+1399/-333`, with
aggregate binary-diff SHA-256
`5445faa5739064714e98ce11d8cb8c20d68d9ab55dd88e06cab4231b9b6505d3`.
The machine record binds every current content and per-file binary-diff hash.

## Validation

- `npm run build:wasm:desktop`: `PASS`, rebuilt from the current resolver.
- Focused affected matrix: `23/23 PASS`, `67` skipped, `19.27s`. This includes
  eight kind/node empty-role negatives, four direct post-queue negatives, ten
  Inspector/viewport positive kind cases, and the enhanced three-incident
  reset/exact-role application case.
- Exact-final `npm run test:desktop`: `29` files, `562` tests, all passed in
  `104.01s`.
- `npm run build:desktop`: `PASS`; only the standing Vite chunk-size advisory.
- Five-file basis-scoped `git diff --check`: `PASS`.
- Practitioner harness self-check: exit `0`; standing unrelated inventory is
  `INFO=14`, `NOT_APPLICABLE=1`, `REVIEW=4`, `WARN=40`.

## Containment and handoff

- Blockers: none.
- P1 residuals known to this instance: none.
- Waivers: none.
- Product writes remained within the four N1 UI/test targets. The resolver
  blob is unchanged by this instance.
- No commit, push, merge, PR, status/coverage update, or receipt act occurred.
- Fresh integrated review remains mandatory over 100% of the refreshed
  five-file product diff and hash inventory.
- Next owner: `HELP_HUMAN` for fresh repair-cycle-1 integrated review.
