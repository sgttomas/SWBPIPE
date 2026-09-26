# Archived run records

Closed run records were moved out of the working tree under D-GOV-45
(`docs/governance_harness/_DECISIONS/D-GOV-45_agent_run_record_archive.md`).
Nothing was deleted: each archive tag holds the exact bytes, and
`ARCHIVE_INDEX.json` lists every archived run folder and file.

Read an archived file without restoring it:

```bash
git show <tag>:<path>
```

Bring a run folder back temporarily for context (do not commit it unless the run is reopened):

```bash
git restore --source=<tag> -- <path>
```

| Tag | Archived on | Run folders | Binary files |
|---|---|---|---|
| `archive/agent-runs-2026-09-25` | 2026-09-25 | 54 | 229 |
