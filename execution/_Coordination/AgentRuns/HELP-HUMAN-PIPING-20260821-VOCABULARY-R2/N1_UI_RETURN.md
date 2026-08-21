# N1 UI manager return — vocabulary completion round 2

- RunID: `HELP-HUMAN-PIPING-20260821-VOCABULARY-R2`
- InstanceID: `WORKING-ITEMS-VOCAB-R2-N1-UI`
- Role: `WORKING_ITEMS`
- Package: `PKG-07`
- Selected deliverables: `DEL-07-01`, `DEL-07-02`
- Accepted basis: plan v1 at `7d5a3f558dfa2e8e902df25fc9a3e813a9ab7048`; proven bend seam `b988d9d0e4a7048ac28a73bbe53ce045c631dff8`
- Verdict: `ACCEPTED_FOR_N1_FAN_IN`

## Coverage and outputs

The Inspector and viewport component-creation surfaces now expose separately
selected `tee`, `reducer`, `valve`, and `flange` creation in the owner-required
order while retaining the proven bend path. Every kind queues an
`insert_component_symbol` structured operation; no UI code mutates accepted
model state directly.

- `tee`: explicit component/node identity; distinct incident header and branch
  pipe selections; positive run/header sizes; positive connection angle;
  explicit length/angle units; nonempty connection type, reinforcement
  reference, geometry-source reference, and provenance.
- `reducer`, `valve`, `flange`: explicit component/node identity; incident
  realized-pipe selection; positive body length, end sizes, and weight; finite
  three-axis center of gravity; explicit length/force units; nonempty end,
  stiffness-behavior, geometry-source, and provenance references.
- The Inspector and viewport emit the resolver's frozen key names and nested
  quantity shapes exactly. No modifiers, protected tables, engineering values,
  dimensions, weights, COG values, or stiffness behavior are inferred.
- Cross-surface queued target-ID reservation remains enforced.
- Bend Inspector and viewport regression coverage was restored during review.

Changed product/test paths:

- `projects/chirality-piping/apps/desktop/src/features/component-creation/componentIntent.ts`
- `projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport.tsx`
- `projects/chirality-piping/apps/desktop/src/features/model-tree/PropertyInspector.tsx`
- `projects/chirality-piping/apps/desktop/src/App.test.tsx`

No resolver, deliverable status, vocabulary coverage, receipt, shared handoff,
export, or Git-state surface was written by this instance.

## Validation

- Local WASM operation engine rebuilt from the live resolver source: `PASS`.
- New Inspector/viewport tee, reducer, valve, and flange cases: `8/8 PASS`.
- Restored bend Inspector/viewport cases: `2/2 PASS`.
- Exact-final-diff registered desktop suite: `29` files, `549` tests, all
  passed in `98.98s`.
- `npm run build:desktop`: `PASS`; only the existing Vite chunk-size advisory.
- `git diff --check` over all four owned files: `PASS`.
- `python3 tools/practitioner_harness/harness.py self-check`: exit `0`; its
  standing unrelated cross-project REVIEW/WARN inventory remains outside this
  package activation and was not changed.
- Fresh read-only `TASK + software-code-review` covered 100% of the final
  `607`-insertion / `280`-deletion owned diff: `PASS`, no remaining actionable
  findings. The reviewer identified loss of bend UI regression tests during
  the first refactor; both tests were restored and passed before the terminal
  verdict.

Detailed machine-readable results and exact hashes are in `N1_UI_CHECKS.json`.

## Failure and remediation record

The first focused viewport-apply run passed all four Inspector cases but
blocked all four apply cases because the generated, intentionally uncommitted
browser WASM still represented the pre-change bend-only resolver. Detection
layer: focused App Vitest; failure class: `STALE_GENERATED_TEST_RUNTIME`;
disposition: rebuild the WASM operation engine from current resolver bytes and
rerun. The rerun passed `8/8`; this was not a product-contract failure.

The fresh reviewer then found that the refactor had removed the two existing
bend-specific UI tests. Detection layer: independent semantic review; failure
class: `TEST_REGRESSION`; disposition: add bend to both parameterized suites,
run the focused `2/2` check, and rerun the full registered suite. The reviewer
accepted the remediation and returned `PASS`.

## Blockers, residuals, and handoff

- Blockers: none.
- Residual tee/reducer/valve/flange UI kinds: none.
- Waivers: none.
- Derivative disposition: local generated WASM was validation-only and is
  intentionally uncommitted; no governed derivative package was created here.
- Next owner: `HELP_HUMAN` for N1 engine/UI fan-in, integrated review, ordered
  commit, and row-15/shared-surface disposition.
- Lifecycle/Git: no acceptance-state, commit, push, merge, PR, or receipt act
  was performed by this instance.
