# Common terms for T3 TASKs

Every T3 TASK brief is read with this file. Where a brief differs, the brief wins.

## Roles and returns

- You are a Type 2 TASK. Read Root `AGENTS.md` and `agents/AGENT_TASK.md` first. You do not delegate.
- ROOT (HELP_HUMAN) spawns you at the T3 WORKING_ITEMS manager's request. The manager owns T3's integration and is your return path: send your return to the manager by SendMessage, using the name ROOT gives you at spawn. If you need a decision that blocks you, ask the manager the same way; do not wait silently.
- Your return is the file your brief names, plus a short SendMessage summary (verdict or status, files written, what remains).

## Locations

- Worktree: `<worktree>` is the T3 worktree on branch `codex/piping-numerical-integrity-20260926`. ROOT gives the actual path at spawn.
- `P/` is `projects/chirality-piping/`. `T3/` is `P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/`.
- Product basis: main `c61a540ea` (contains PR905 `23aad15d6` and T0R/PR952 `82b43f9bd`).
- T1 basis, read-only: branch `origin/codex/piping-load-states-20260925` at candidate `f3270ea79`. Read it with `git show f3270ea79:<path>` or `git diff origin/main...f3270ea79`. **Never read from, write to, build in or run anything in T1's worktree, and never touch its branch.**
- Start with `T3/STAGE0_MAP.md` and `T3/STAGE1_PLAN.md`.

## Writes

- Write only inside the write set your brief names. Everything else in the repository is read-only, including product source, tests, fixtures, schemas, other records and the work graph.
- Run no Git write: no commit, branch, stash, reset, checkout or push. The manager commits.
- Temporary files, probe crates and build output go under `<scratch>` (your own scratch directory, outside the repository). Delete build output when done.

## Host resources

- The T1 manager is running a quiet-host DEC-025 sweep. **Run no cargo build, cargo test, npm build or full test suite until the manager tells you ROOT has released the host.** Reading, grep, design and standard-library Python are always fine.
- When released: Rust uses `RUSTUP_TOOLCHAIN=1.97.1` (matches CI), `CARGO_INCREMENTAL=0`, and T3's own target directory `<t3-target>` (ROOT gives the path). Never use T1's target directory. Python uses the DEC-025 venv (path given at spawn). Node 24 is on the host (path given at spawn).
- Keep free disk above about 8 GB (`df -h` on the home filesystem). Prune only your own build output.

## Rules

- Invented inputs only. No material, component, catalogue or code-rule data is added by any agent. State every property you use.
- Frozen references, fixtures, hashes and protected criteria are never edited: N01–N09, R01–R07 and NP-A–NP-D in `P/validation/benchmarks/numerical_integrity/`, the T0R references, T1's references, the analytical 1e-9 relative criterion and the DEC-050/053 observations. A disagreement is reported, never resolved by changing one of them.
- No skipped tests, raised timeouts, stripped loads or dropped features to make anything pass.
- Committed records use placeholders (`<worktree>`, `<scratch>`, `<t3-target>`, `{REPO_ROOT}`) instead of machine-specific absolute paths.
- Record what you actually read and ran (paths, commits, hashes where they matter) in your return, and state what you did not do.
- Accepted and historical design records are basis, never evidence that current code is correct. Cite source lines at `c61a540ea`.
