# Sealed CI review repair — three concrete findings

ROOT to existing TASK /root/ci_strategy, Astra/low, no delegation. Same bounded
CI file/test/evidence scopes in swbpipe-wt2; do not edit product or ROOT docs.
No Git mutation, browser/native/build/full tests. Current integration head160941d3
includes your frozen097decd and reviewed tooltip3861 plus ROOT records. Preserve
all earlier review/return evidence; write new evidence under CI-STRATEGY-CODEX/repair1/.

Independent code review reports three actionable findings (final return follows):
1. e2e/ui-foundation/benchmark-harness.ts is imported by ui-foundation-workflows.ts
   and feeds workspace-layout.spec.ts and ui-foundation.spec.ts. Directory-wide
   instrument-only routing omits actual consumers.
2. New jobs check exact PR head instead of synthetic merge but do not establish
   that target base is integrated. The old merge base can mask a stale branch.
3. One absent focused title in the OR filter silently loses its two profile cases;
   argument nonemptiness and a prior manual listing do not enforce live coverage.

ROOT dispositions:
- Remove directory-wide instrument reduction. Non-spec instrument helpers/fixtures
  use full fallback. Keep source-spec-only reduction and the explicitly reviewed
  PR825 exception. No guessed import-dependency closure or hermetic classification.
- Retain exact PR-head execution, but record event target base separately from
  merge base and enforce that valid event target base is an ancestor of head before
  any test run can succeed. Missing/unresolvable or unintegrated target base must
  block execution with a clear update-base diagnostic, not become a green head-only
  result. Manual full remains possible. ROOT revalidates live main again before
  merge. Do not widen PR825 allowlist to bypass new upstream executable changes.
- Before execution, collect candidate test IDs through real Playwright --list in
  the actual projects. Require each requested focused title exactly once in each
  requested profile and every selected file nonempty in both profiles. Full mode
  must compare unfiltered source inventory to barrier + all four shard inventories,
  exactly once/no gaps. A cancelled/missing/empty collection never passes. Emit
  source head/base, actual selected/omitted test IDs with reasons, command and
  browser/config identity as structured retained CI evidence. Do not start browser
  tests merely to validate collection. Record validation as collection, not a pass.
- Persist per-stage collection/evidence with unique names, preserve job-level
  failure artifacts and readable summary. Collect/validate before costly remainder;
  avoid duplicating browser execution. Remainder should bind its actual collected
  tests to candidate/plan and full partition, not trust arbitrary downloaded args.

Add meaningful regressions: shared benchmark helper/fixture forces full; stale
base and unavailable target fail execution; focused missing/duplicate title or
profile, empty selected file and missing/duplicate full partition are refused;
valid manual/full/reduced collections accepted. Preserve both source viewports,
all tests/oracles/timeouts/skips/workers and pinned setup. Rerun18previous tests
plus new tests and candidate listing (now436full,68focused after6new regressions).
Notify ROOT if syntax or evidence contract needs additional path scope.

Return frozen repaired working diff, exact source/evidence hashes, failing then
passing tests, and any limitations. ROOT independently backchecks before sweep/CI.
