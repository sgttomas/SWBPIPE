# Notice — agent run records archived (D-GOV-45)

Root decision D-GOV-45 (`docs/governance_harness/_DECISIONS/D-GOV-45_agent_run_record_archive.md`)
is owner-directed. It treats run records as history. All five execution roots
were archived together on 2026-09-25.

For Piping, `execution/_Coordination/AgentRuns/` no longer contains
54 run folders untouched for 14 days, or 229 binary files
untouched for 7 days in run folders still in use. Their exact bytes are held at
tag `archive/agent-runs-2026-09-25`. `ARCHIVE_INDEX.json` lists every path; `ARCHIVE.md` shows
how to read (`git show <tag>:<path>`) or restore one.

Kept in place because Root governance gates read them as current state:
- none in this project

What this means for the Piping loop:

- References into archived run records resolve as history in Root's
  self-check. Run records are no longer tested or linted.
- New or changed run records in a PR are scanned for credentials (BLOCK)
  and for files over 5 MB (WARN). The repository is public; keep traces and
  screenshots as CI artifacts where possible.
- File new governance or loop state outside run folders.
- Deliverable folders, `_Reconciliation`, `_ScopeChange`, decisions and your
  acceptance evidence obligations are unchanged. A run that needs an archived
  record reads it from the tag and restores it only to reopen that run.

CI note: the Piping selector's step summary now lists the first 200 changed
paths and a count. The archive diff produced a 10.8 MB summary, over GitHub's
1 MiB limit. The uploaded `piping-e2e-selection` artifact keeps the complete
plan, and selection is unchanged.

The loop decides how to record adoption. Future archives run with
`tools/archive_agent_runs.py` under the same policy.
