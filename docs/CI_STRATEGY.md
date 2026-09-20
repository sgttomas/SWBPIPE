# Desktop CI selection and integration cadence

The Piping desktop workflow runs a cheap selector when a pull request opens,
receives a new head, or reopens. Its
stable check remains **Desktop E2E (source mode)**. An unrelated App/Runtime PR or
Piping evidence-only change receives an explicit validated **not-applicable**
result; no Piping browser setup or tests run, and no test-pass claim is made.
Runtime is not a Piping dependency before its owning adoption decision.

The owner authorized reduced routine execution frequency and exact scenario
deduplication on 2026-09-20. Selected coverage is not equivalent to running the
full suite. Omitted identities and reasons are retained in the collection
artifact, and partial CI is not DEC093 full surface4 evidence.

| Complete PR diff | Source coverage |
|---|---|
| Reviewed reason-tooltip component only | Lean ordinary journeys |
| Shell/layout/appearance CSS and named layout helpers | Lean plus all distinct layout cases, twelve appearance combinations, Escape ownership and overlay interactions, plus complete C3 viewport-visibility coverage |
| Viewport, model-tree/properties, geometry/rich authoring and selection state | Lean plus full UI-foundation, linear authoring and C3 viewport-visibility coverage |
| Results/report/export/storage display, redaction and persistence-integrity helpers | Lean plus complete result compatibility, GUI, session-status, R2 and B3B project-persistence files |
| Reviewed instrumentation module | Lean plus its transitive literal-import source-spec consumers; shared benchmark harness reaches UI-foundation and workspace-layout as well as instrument contracts |
| Only added/modified source specs | Complete changed files plus accessibility |
| Shared application/session state, dependencies/configuration, native/build/WASM/solver, schemas, CI policy/duration hints, unknown inputs or deleted/renamed test inputs | Full source coverage |
| No relevant Piping/shared inputs | Not applicable |

The maintained router enumerates actual ownership paths. Unknown instrument
helpers, dynamic resource inputs, fixtures and configurations fall back to full;
no directory-wide hermetic or instrument-only classification exists. Source
selection does not replace applicable dist, native or protected benchmark checks.
The closed PR825 exception has been removed; its authorization and results remain
historical records.

The lean set includes the entire B3 accessibility file and the existing canonical
save/history/reopen, first workspace editing, straight authoring, R2
solve/results/report/overlay, strict Current export, both shared drawer/menu Close,
and typed selection/keyboard measurement journeys. Their original project
assignments remain intact. No assertion, geometry/oracle tolerance, default test
timeout or permanent skip was changed to obtain a reduced run.

## Exact coverage and duration assignment

The explicit appearance matrix sets its viewport before navigation and retains
all twelve theme/density/window combinations once. Explicit-start workspace cases
also run once. They carry `@explicit-viewport`; the compact project excludes that
tag. The desktop project, first workspace journey's two distinct starts, and
selection-cardinality test's two initial states are preserved. CLI `--grep` and
exact lists retain the project filter; `--grep-invert` overrides are rejected to
prevent reintroducing repetitions. Collection also rejects a compact tagged test
or an incomplete appearance matrix. Removed repetitions map to their retained
same-title desktop counterparts in the implementation evidence.

Full CI starts with accessibility. Four isolated runners then execute exact
Playwright 1.60 `--test-list` partitions with one worker each. Duration hints come
from completed PR826 source logs and are checked into `tools/ci/e2e_duration_hints.json`;
new or untimed tests use a conservative 30-second scheduling weight. These values
are scheduling hints, not acceptance limits or a speed guarantee. Assignment is
deterministic, using the lightest estimated bin first. Files/projects remain
atomic except the independently set-up UI-foundation and workspace-layout cases;
named describe groups and explicitly serial files remain together. No global
`fullyParallel` change is made.

Every runner collects the actual source inventory and checks each exact list before
browser execution. The selected union must contain every intended identity exactly
once, with no empty, missing, duplicate or unexpected matches. Required lean titles
must exist exactly once per original profile. Collection records source/head/target
base, config/source/policy hashes, browser configuration identity, commands, IDs,
selection reasons and scheduling estimates. Collection is not a browser-test pass.
Cancellation, collection failure, selector failure or a missing required job fails
the stable aggregate; only explicitly unneeded jobs may skip.

## Full checkpoints and integration

Manual `workflow_dispatch` is the deliberate full integration-milestone mechanism
and always selects full source coverage. Broad-risk and CI-policy changes also
select full automatically. There is no label-triggered or unconditional nightly run.

Body edits, readiness changes and label changes launch no workflow, participate in
no concurrency group and emit no competing successful/skipped stable check. They
cannot cancel or replace a meaningful pending validation. Base retargeting is also
an ignored metadata edit: it does **not** automatically invalidate or rerun GitHub
checks. A retarget without a new head therefore requires deliberate full dispatch
on the current candidate with the **target_base** input set to the current target
commit SHA. Alternatively, updating/reopening the PR validates its current event
base. The supplied manual target must be an ancestor of the exact dispatched head;
unresolved or unintegrated targets fail. Manual head, target and merge-base evidence
remain distinct, and ROOT revalidates the live target before relying on the proof.

Execution checks the exact PR head and records the event target base separately
from its Git merge base. The target must be integrated into head; stale or
unavailable target data blocks the check with an update-base diagnostic. Hosted
event identity and PR number must match the plan; a manual dispatch also binds its
requested target and exact head. ROOT rechecks
live main before merge. Workflow and collection artifacts are bound to the actual
candidate rather than reused across changed inputs.

Develop with focused checks, then combine related independently reviewed work into
one coherent integration candidate and one clean local DEC-025 sweep before product
integration. Full local source/dist, native witness and governed qualification
requirements remain distinct from this hosted source check. Fewer internal-worker
PRs do not weaken review or merge checks, and an early owner merge is not a waiver.
Branch protections, other project workflows and repository permissions are unchanged.
