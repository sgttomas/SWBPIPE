# Notice — hosted CI burden follow-ups

Tranche `ROOT-CI-BURDEN-FOLLOWUPS-20260925` follows
`ROOT-CI-BURDEN-REDUCTION-20260925` (see
`NOTICE_2026-09-25_CI_BURDEN_REDUCTION.md`). What changed for Piping:

- **Full runs start all four shards at once.** In full mode the B3
  accessibility file is balanced into the four exact partitions as an ordinary
  atomic group. The separate barrier job, which repeated the whole browser
  setup and delayed every shard by about four minutes, now runs only in partial
  modes. The aggregate requires `barrier == skipped` and `remainder == success`
  for full mode. A failing shard still cancels the others (fail-fast matrix).
  Coverage is unchanged: the union of partitions still contains every selected
  identity exactly once.
- **Duration hints refreshed** from successful full dual-profile run
  36112629958 (PR 901): 475 measured durations and 20 skips. This removes all 40
  fallback weights for the new B4 table and Sections identities. On the same
  collection, replayed measured durations put the slowest shard at 341s rather
  than 415s for pull-request full runs, and 417s rather than 510s for manual
  full dispatch.
- **`tools/ci/refresh_duration_hints.py`** is the maintained, reproducible
  refresh: `--run <id> [--apply]` reads the run's collection artifact and
  barrier/shard logs through `gh`, and launches no browser or test.

The Piping loop decides whether to adopt, amend or revert these changes. This
notice does not change local check profiles, native witnesses, release gates
or DEC-025/DEC093 evidence obligations.
