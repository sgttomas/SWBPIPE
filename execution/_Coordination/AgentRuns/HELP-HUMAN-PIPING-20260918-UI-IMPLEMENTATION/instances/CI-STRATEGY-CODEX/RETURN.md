# CI strategy implementation return — 2026-09-20

TASK `/root/ci_strategy`, parent `/root` HELP_HUMAN, executed through the native
Codex collaboration harness. No descendants. Model/settings inherited from
parent; the sealed brief specifies gpt-6-astra/low. Scope restrictions were
instructions, not filesystem enforcement. No repository Git mutation, browser,
app, native, build, or full test execution occurred. Temporary fixture Git
repositories were created and committed solely for the required policy tests.

Working basis: `002dff0f244976f98b36517d920b3761f6f88704` in `swbpipe-wt2`.
`hashes.json` records exact instruction, skill, brief and output origins/hashes;
the two sealed brief hashes were verified against the launch values.
ROOT supplied additional exact allowlist decisions through coordination messages:
DisabledReason.tsx; the CI tranche manifest; named repair evidence directories;
and summary JSON under validation/evidence/sweeps for PR825 only. No broader
product paths were added. ROOT concurrently authored its own docs/manifest/notice;
those files are outside this implementer's frozen patch and output hashes.

## Result

Four maintained files implement conservative routing, the existing pinned setup
as a composite action, early accessibility execution, four isolated full-mode
remainder runners, and the stable `Desktop E2E (source mode)` gate. The gate
requires success from selection and barrier, and full remainder success unless
a recognized reduced mode explicitly omits it. Cancellation/failure/invalid or
missing mode cannot pass. PR event head is checked out consistently, full Git
history is fetched, ordinary routing compares against the complete merge base,
and the PR825 exception separately checks ancestry and the entire repair delta.

Reduced ordinary routing covers instrument-directory-only changes or changed
source specs with optional prose/data records. Product/config/core/schema/fixture,
unknown inputs, deleted/renamed test inputs, unavailable diff, empty ordinary diff,
and dispatch default full. The exception requires PR825 and the exact tested
baseline, exact repair paths, narrow CI/manifest paths and allowed records.
A recomputed plan must match checkout before execution. Commands use argument
arrays, escaped file regexes, explicit projects, one worker and the existing
180000 ms hosted timeout. Empty selections fail; assertions/config/skips remain
unchanged. Focused ui-foundation cases use a separate exact-title-suffix invocation.

`coverage_full` describes planned source coverage, contingent on successful jobs;
reduced artifacts explicitly forbid a DEC093 full surface4 claim. No measured
speedup or hosted full-mode qualification is claimed.

## Validation

- `python3 -m unittest discover -s projects/chirality-piping/tests -p test_ci_e2e_plan.py -v`: 18 passed; raw output in `unit-tests.txt`.
- Temporary Git tests cover whole-PR diff and merge-base behavior, source/instrument routing, unknown executable fallback, missing/nonancestor baseline, wrong PR, deleted/renamed inputs, stale/malformed plans, exact exception evidence scope, safe argument construction, partition contract and aggregate success/failure/cancel/skip states.
- `python3 <run>/validate_inventory.py.txt`: all actual Playwright calls use `--list --reporter=json`, without browser launch. Original source inventory 430 entries equals accessibility24 plus disjoint shards147/59/144/56, each test/project once. Old-source focused union62 = accessibility24 + layout/gui30 + eight focused ui-foundation project entries. Commands and counts are in `inventory-validation.json`; raw list JSON and stderr are retained. This inventory predates the manager's added tooltip regressions; ROOT must repeat after fan-in. Existing20 runtime skips are not reclassified by listing; no source tests were edited.
- Workflow/action YAML parsed using the already-installed Playwright-core bundled YAML parser. Dependency DAG, four-shard matrix, always-running aggregate and composite shell declarations checked. All13 setup steps compared structurally with HEAD; only required explicit bash shells added. `yaml-validation.txt` records success.
- CLI `plan --event workflow_dispatch` and `run --stage barrier --list` passed using `manual-plan.json`; summary and raw listing retained.
- `git diff --check`: passed. Source/dist Playwright configs byte-equal to HEAD, checked while recording hashes.

Initial diagnostics: one temporary-Git test initially used `checkout -` after an
orphan checkout; corrected to the saved candidate SHA. Python's system runtime
has no PyYAML, so YAML validation used the installed Playwright YAML parser.
An initial setup equivalence probe sliced14 rather than13 original setup steps;
corrected the probe, with no product/setup change. These were validation fixture/
tooling issues; the final checks above pass.

## Handoff

`working-diff.patch.txt` and `hashes.json` freeze only the four maintained files.
No independent review or hosted run is represented by these self-checks. ROOT
owns fresh-context review, integration, complete-delta selection verification,
clean local registered sweep, actual candidate CI, commits/push/merge and notices.
The next broad product slice must exercise hosted full mode. File-level shard
sizes are uneven by design; global fullyParallel remains unchanged. The local
DEC025 evidence sweep and its full-source obligations remain untouched.

Syntax references consulted: https://playwright.dev/docs/test-sharding and
https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax.
