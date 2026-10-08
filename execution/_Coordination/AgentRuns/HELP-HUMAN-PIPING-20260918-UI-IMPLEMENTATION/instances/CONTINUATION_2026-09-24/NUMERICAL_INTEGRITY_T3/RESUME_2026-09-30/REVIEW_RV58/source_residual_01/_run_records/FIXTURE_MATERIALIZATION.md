# Fixture links replaced by their source bytes (2026-10-07)

This is a post-seal correction to the repository copy of this packet. The RV58
review, its verdict, and the sealed `SEAL.json`, `INVENTORY.json`,
`FIXTURE.json` and `FIXTURE_README.md` bytes are unchanged. Those files still
describe the packet as it was returned on 2026-10-03.

## What changed

The packet as returned and as committed in `f506f3e2de` (#1084) held 104 of the
reviewer fixture's 110 entries as symlinks (Git mode 120000) to absolute paths
in the reviewer's local worktree,
`/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a-arithmetic/projects/chirality-piping/core/solver/frame_kernel/<path>`.
Off that machine they dangled, and checkout reported "Too many levels of
symbolic links" for `fixture/.gitignore`.

Each link is now replaced by the bytes at the same path under
`projects/chirality-piping/core/solver/frame_kernel/` in the recorded SOURCE
candidate `6ba653451f9fd27cdb852b7974753a3921f1c483` (`EXECUTION.json`,
`SOURCE_AFTER.json`). 94 file links became regular files and 10 directory links
became directories, which gives 130 regular files, all mode 100644. The six
copied entries and the appended reviewer test were already regular files and
are untouched. The candidate was not on `main` on 2026-10-07. It was reachable
from branch `codex/piping-f2a-arithmetic-20261002`, among others.

## Why these are the bytes the run used

- Per `FIXTURE_README.md`, the links referenced SOURCE at that candidate.
  `SOURCE_AFTER.json` records HEAD `6ba6534…` with empty `git status` after
  the run, so every tracked file the links reached held its candidate bytes.
- The candidate reproduces all recorded hashes: the 6 `copy` entries'
  `source_sha256` in `FIXTURE.json`, the 8 paths in `SOURCE_BEFORE.json`, and
  the 8 `unchanged_baseline` paths in `SOURCE_AFTER.json`. Fourteen of those
  16 source paths (6 of `SOURCE_BEFORE.json`, all 8 baseline paths) lie under
  linked entries.
- For every one of the 104 replaced paths, the Git object id in this commit
  equals the candidate's object id (blob or tree), so the bytes and file modes
  are identical:

  ```sh
  C=6ba653451f9fd27cdb852b7974753a3921f1c483
  K=projects/chirality-piping/core/solver/frame_kernel
  F=<this _run_records dir>/fixture
  # for each symlink entry path P in FIXTURE.json:
  test "$(git rev-parse $C:$K/$P)" = "$(git rev-parse HEAD:$F/$P)"
  ```

- Replay: the recorded reviewer command (`reviewer_fixture.command.json`:
  `cargo test --locked --offline --lib source_residual_rv58 -- --nocapture`,
  isolated target dir), run on the materialized fixture with cargo 1.97.0 on
  Linux, passed 1 test. Its output matched `reviewer_fixture.log` line for
  line, except for the elapsed time.

## Limits

- The links recorded no hash of the files they pointed to (`INVENTORY.json`
  hashes the link text only). The check above therefore rests on the clean
  candidate state recorded in `SOURCE_AFTER.json` and the hashes listed, not
  on a hash of each linked file taken during the run.
- An untracked or ignored file under a linked directory at run time, such as
  build output, is not recoverable and is not reproduced. Nothing in the
  records says the run read one.
- `INVENTORY.json` and `SEAL.json` still list 104 symlinks and 55 regular
  files. They are the sealed historical description and do not describe the
  current tree.
