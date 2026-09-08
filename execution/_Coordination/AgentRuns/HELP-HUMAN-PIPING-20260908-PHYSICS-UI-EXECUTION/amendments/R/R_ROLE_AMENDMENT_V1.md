# R role amendment V1

Status: ACTIVE ADDITIVE AMENDMENT to `WORK_GRAPH_V1.json`; prior frozen artifacts remain unchanged.

Replace node R's role `REVIEW Agent1` with a fresh bounded Agent 2 dispatched directly by `/root` after the authored tranche is frozen:

- If product source changed, dispatch `TASK + software-code-review` over 100% of the frozen source diff and its affected evidence.
- If the tranche contains reference/design evidence only, dispatch a fresh ephemeral evidence reviewer over the exact frozen artifact inventory.

Required configuration remains model `gpt-5.6-sol`, reasoning `high`, with fresh `fork_turns=none`. R is read-only, cannot be an author, writes only `RUN_ROOT/instances/R/**`, and returns a scoped independent verdict/findings for root fan-in. It receives no lifecycle, acceptance, dependency-ruling or formal-hold authority. `AGENT_REVIEW` is not used for this cross-package terminal validation. Historical REVIEW/PDU holds remain separate and route through PS and the Owner's governing instrument.

All other V1 node dependencies, failure isolation, source-write holds, shared ownership and CHANGE gates remain unchanged.
