# Piping CI cache preparation and scheduling data

Piping HELP_HUMAN owns a bounded follow-up under the owner's effective-CI
direction. The new internal `Piping E2E dependency cache` workflow reuses the
existing setup action on canonical-repository `main` only, for changed bootstrap
inputs or explicit manual dispatch. It prepares caches without another test
suite, coverage check, scheduled run or repository-permission change.

The owner direction, peer writer acknowledgement, observed cache misses and
current implementation/check state are retained in the UI implementation run's
`instances/ROOT/CONTINUATION_2026-09-19_CODEX/CI_CACHE_FOLLOWUP.md`, its
`_run_records/CI_CACHE_DISCOVERY/` and the work graph. PR832 remains separate.

The shared setup action, existing Root/App/Runtime workflows and exporter remain
unchanged. The peer retains the separate existing public-export compatibility
finding. Independent review and integration are pending; actual cache creation
on main and later consumer reuse must be observed afterwards. This notice does
not establish qualification, receiving-loop acceptance or product release.

The same owner-directed CI maintenance refreshes Piping's duration hints from
successful PR832 logs and its canonical collected identities. It preserves
coverage, grouping, fallback and skip weights, timeouts and all test oracles.
Twenty newer table identities now have observed durations instead of fallback
weights. A historical replay verifies identical coverage/barrier and a more
balanced sum of recorded durations; it is not a claim about future runner time.
Exact inputs, the one-off reproduction method and the observed-versus-replayed
limits are in the continuation's CI_DURATION_REFRESH_ADDENDUM.md and
_run_records/CI_HINT_REFRESH/. No shared selector or other project is modified.
