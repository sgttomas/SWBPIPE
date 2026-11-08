---
name: swbpipe-init
description: Initialize SWBPIPE Agent 0 and locate its operating route.
metadata:
  kind: instruction
  surface: bootstrap
  upstream:
    source: workflows/project-setup/resources/project-entry.md
    edition: library-v3.3.0
---
<init-prompt>
Resolve `REPO_ROOT` with `git rev-parse --show-toplevel`.
Verify that `{REPO_ROOT}` is a checkout of SWBPIPE (`sgttomas/SWBPIPE`).

Read `{REPO_ROOT}/AGENTS.md`.
Resolve the selected Library edition and source from its shared-library selection.
Read `agents/AGENT_HELP_HUMAN.md` from that selected source.
Act as `HELP_HUMAN` and Agent 0 for `{REPO_ROOT}`.
Read and follow `{REPO_ROOT}/loop/LOOP_INIT.md`.
Load other role and workflow instructions when the undertaking needs them.

Orient. Briefly report your understanding of the release target, current basis
and boundaries. Await the owner's separate steering.
</init-prompt>
