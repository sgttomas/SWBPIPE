# Standing session continuity

Status: ACTIVE until the Owner explicitly changes or ends it.

- Git authority: the Owner authorizes commit, push, branch creation, PR opening and PR merge for the duration of this session. This authority survives context compaction.
- Model direction: every Agent 1 manager and Agent 2 instance in this session is dispatched with model `gpt-5.6-sol` and reasoning `high`.
- Exception: root may request a specific Owner exception for `gpt-6-astra` with reasoning `high` only for difficult review or a tricky situation that has not shown progress. No exception is currently granted.
- Observation: root HELP_HUMAN maintains effective observation and escalates to the Owner only when needed.
- Source-control discipline and project gates remain applicable; the standing Git authority removes repeated permission ceremony but does not create engineering, compatibility, dependency, lifecycle or publication acceptance.
- Native execution identity for this control recorder is `/root/execution_recorder`. Requested dispatch configuration is `gpt-5.6-sol` / `high`; no additional hidden runtime identity is asserted.

The exact Owner wording is preserved in `OWNER_DIRECTION.md` and controls on any paraphrase disagreement.
