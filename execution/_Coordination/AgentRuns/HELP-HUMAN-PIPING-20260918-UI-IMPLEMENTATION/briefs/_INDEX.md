# Sealed briefs — index

Run `HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION`. Every brief is written, hashed and listed here before its child is launched. Each row records the mechanism, the model requested, the role and the return. The model that actually ran is recorded with the return. Returns are retained with SHA-256. Type 2 children do not delegate.

| Brief | SHA-256 | Sealed | Model requested | Role | Return |
|---|---|---|---|---|---|
| `A1-LINT_registry_and_lint.md` | `3db45afb67197ce57b7dc52c6e871a4c15edf8f3f670b7fbe19d785789491cad` | 2026-09-18T19:41Z | `fable` (Claude Fable 5.1) | TASK implementer; the registry and the root claims lint | `../instances/A1-LINT/RETURN.md` |
| `A1-APP_desktop_visible_rename.md` | `19fe7ca236cd9681e799e138637cdc442e9274707dd313c3f9d54312ce1f6480` | 2026-09-18T19:41Z | `fable` (Claude Fable 5.1) | TASK implementer; `apps/desktop/**` | `../instances/A1-APP/RETURN.md` |
| `A1-DOCS_sca010_core_docs.md` | `2a45a9731a1953c8907a76a8fce15b2adf26e114da73c2af3a7e4758c392c17d` | 2026-09-18T19:41Z | `sonnet` (Claude Sonnet 5) | TASK implementer; SCA-010, core text, documents, packaging tools | `../instances/A1-DOCS/RETURN.md` |

Tranche A1, mechanism for all three: Claude Code `Agent` tool, general-purpose type, background, in ROOT's worktree on branch `codex/swbpipe-a1-visible-rename-20260918`, at the same time, with disjoint write scopes by path. ROOT integrates and commits; the children run no state-changing git command.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
