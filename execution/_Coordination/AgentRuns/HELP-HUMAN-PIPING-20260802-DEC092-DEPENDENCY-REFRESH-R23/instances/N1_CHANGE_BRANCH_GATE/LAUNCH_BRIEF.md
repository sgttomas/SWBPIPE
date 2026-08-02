# N1 CHANGE branch gate — launch brief

- Parent: HELP_HUMAN / PROJECT_SETUP R23.
- Role: CHANGE.
- Objective: verify the synchronized Git basis, create and switch to the exact
  owner-approved branch, and return the branch identity without editing project
  semantics.
- Required basis: `23d15899fd0acf5d1d0513f3fe396438375c9e25`.
- Required branch: `codex/piping-dec092-dependency-refresh`.
- Evidence required: HEAD, local `main`, and `origin/main` all resolve to the
  required basis; worktree is clean before branch creation; branch did not
  already exist; branch is current after switch.
- Exclusions: no commit, push, PR, merge, reset, dependency semantic write, or
  control-plane reinterpretation.

