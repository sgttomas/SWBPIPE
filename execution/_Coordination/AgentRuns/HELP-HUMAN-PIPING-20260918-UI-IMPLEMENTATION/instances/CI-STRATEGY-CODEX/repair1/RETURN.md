# CI review repair1 return — 2026-09-20

TASK `/root/ci_strategy`, parent `/root`, native Codex delegated child; same
inherited model and bounded CI/evidence scope as the original brief. No child
agents, repository Git mutation, UI/browser/native/build/full test execution.
Temporary Git fixtures remain confined to policy tests. Instructions are scope
controls, not host permission enforcement. ROOT's concurrent docs/notice remain
outside this implementer's patch. Earlier return, inputs and failure evidence
are preserved unchanged.

Source basis: `160941d3f9ffcce393583aa73f4dc72b32013117`. Sealed repair brief hash
verified against `3ac96ee4c50efc2474d86fdc61a660802bc986d5ab7d0568f8599d4672547805`.

## Repaired findings

1. Removed the directory-wide instrument reduction. Instrument helpers/fixtures,
   including shared benchmark-harness.ts, now select full coverage. Actual
   source-spec-only changes can still select changed files plus accessibility.
   The retired `instruments` mode cannot pass the aggregate gate.
2. Plans retain `target_base` separately from merge `base`. PR plan validation
   requires a resolvable target base that is an ancestor of the candidate head;
   otherwise it raises an update-base diagnostic. Selection validates before
   downstream setup/execution; runners validate again. Hosted event identity,
   PR number, target SHA and head SHA must match plan metadata. A plan cannot
   relabel a PR as manual or discard its target base. Manual full remains valid.
3. Each runner now uses actual Playwright `--list --reporter=json` before browser
   execution. Every selected file must be nonempty in both profiles; each focused
   exact title must occur once per profile. Actual selection must equal the
   expected collected IDs, with no unexpected/missing/duplicate IDs. Full source
   collection must equal accessibility plus all four shard collections exactly
   once. Missing/empty/error collection fails. Remainder runners recollect the
   full partition locally and bind their own IDs to that checked candidate/plan.

Collection artifacts retain selected/omitted IDs with reasons, execution-stage
IDs, list/execution argv, source head/target/merge base, plan/selector/config/source
file hashes, actual collection configs, pinned browser registry and configured
executable/fallback inputs. `browser_launched=false` and runtime version null
explicitly distinguish this from an observed browser run. Each stage uploads a
unique artifact even on failure; existing browser failure artifacts remain.
Readable job summaries report collection status and counts without a test-pass
claim. `run --evidence-dir DIR` is required; `--list` performs validation only.

ROOT also authorized four exact PR825 reproduction-record paths under
B3-CODEX/ci-tooltip-repair. They are enumerated, not a generic shell/diff allowance.
A regression proves an adjacent unlisted shell script still forces full coverage.

## Validation

- `before-tests.txt`: new routing/base expectations against the previous selector
  produced 4 failures and 1 error across 20 tests (expected pre-repair regression).
- `after-tests.txt`: all 28 tests passed with synthetic outer GitHub event variables
  set, ensuring fixture repositories remain independent of the host event. Covers
  prior contracts plus shared helper/fixture fallback; stale/missing base; event
  metadata tampering; focused absent/duplicate title/profile; empty file/profile;
  missing/duplicate/empty/wrong barrier partitions; reduced omission; empty/error
  JSON; and exact reproduction-script allowance.
- Real candidate collection, never browser execution: full 436 = barrier 30 plus
  shards 147/59/144/56, exactly once. `full-collection/collection.json` binds this
  to final selector bytes. Remainder-stage invocation independently validates
  the same partition and selects its 147 IDs in `shard-1-collection/`.
- PR825 focused collection validates 68 selected and 368 omitted: accessibility 30,
  complete layout/gui 30 and four focused titles × two profiles 8. The CLI run
  used the recorded synthetic hosted event matching actual local candidate and
  target `21175b5d3668f29acd408812d95a33ed4f14bfc8`; it is a local event-binding
  verification, not a hosted workflow run. See `focused-collection/`.
- YAML parser checks preserved DAG/matrix/always gate and required per-stage
  always-uploaded collection artifacts; `yaml-validation.txt` records success.
- `git diff --check` passed. Composite setup and source/dist Playwright configs
  remain byte-identical to input HEAD.

Maintained frozen patch: workflow, selector and its tests only. Exact hashes are
in `hashes.json`; the patch is `working-diff.patch.txt`. Raw collection JSON/stderr
and plans remain under this repair1 directory. ROOT must independently backcheck,
integrate, revalidate live target main and candidate selection, and run required
clean local/hosted checks. No new browser test pass, full hosted qualification,
performance measurement, acceptance or release is claimed.
