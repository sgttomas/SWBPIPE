# Sealed launch message — R3 task T12

You are TASK T12 in R3 (cross-package synthesis) of run
HELP-HUMAN-PIPING-20260921-RECONCILIATION. Your parent is HELP_HUMAN Agent 0.

Read your brief first and follow it exactly:
/Users/ryan/dev/chirality/.claude/worktrees/task-management-gen-pass-518da2/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260921-RECONCILIATION/briefs/R3-TASK_brief.md (SHA-256 4463e540cb71c212816dfd02897391e4f81f7ca9134867989039f0f737f6a309)
If the hash does not match, stop and report.

Values:
- {TASK} = T12
- {REPO} = /Users/ryan/dev/chirality/.claude/worktrees/task-management-gen-pass-518da2
- {FREEZE} = /private/tmp/claude-501/-Users-ryan-dev-chirality--claude-worktrees-task-management-gen-pass-518da2/efe0b4ff-c1b6-4a97-af70-b04220464651/scratchpad/freeze (read-only checkout of 00115c71931bcae79909602d653740d3bb72dfa1; never write there)

Your scope: rows of `R3/CORPUS_CLAIMS.csv` with ProductCallerNone = YES (666 rows, 641 not divergent): code exists and is tested but no product path calls it (F7). Cluster them by package and engine or area, name the unreached engines, and propose routing.

Your output files, under RUN/R3/TASKS/: T12_UNREACHED.csv, T12_UNREACHED.md.

Addition from the brief review (backcheck 1): ignore every draft resolutions file, i.e. the pattern `RESOLUTIONS_DRAFT*.csv` (this includes `W2/RESOLUTIONS_DRAFT.csv`), not only `RESOLUTIONS_DRAFT_*.csv`.

T12 specifics: `Engine` is the module or file-level unit that the row's ImplementationEvidence cites (for example a `core/` package or a Rust crate); `Area` is the implementation area from `IMPLEMENTATION_SURFACES.csv` that contains that engine, or `UNMAPPED` if none does.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
