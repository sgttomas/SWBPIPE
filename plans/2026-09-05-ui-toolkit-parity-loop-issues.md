# UI toolkit parity — development loop issue

Date: 2026-09-05
Run: HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY
Status: proposed owning-workflow follow-up; no governance/tooling authority change.

## Runtime telemetry cannot represent a recovered prelaunch failure

During PKG05 design intake, native delegation rejected A2_REFUTE before creating a session because the agent thread limit was reached. Its NOTICE used the explicit placeholder session ID `no-session-spawn-rejected`. Parent HELP_HUMAN subsequently dispatched actual independent design reviews and accepted their PASS_DESIGN_ONLY returns. The implementation and fresh source-review sessions also completed with paired START/FINISH events and PASS.

The generic runtime summary nevertheless reports INCOMPLETE and lists the non-existent session as unmatched. This is false as an execution-graph incompleteness claim; it accurately reflects the current summarizer's pairing rule. The record command requires session_id, and summarize groups every event by that field, including a prelaunch NOTICE, then requires exactly one START and one FINISH per group. It has no distinct failed-attempt record or accepted replacement binding. Adding fake session events would corrupt the evidence and was deliberately avoided.

Evidence:
- [Recovery snapshot and exact hashes](../execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N5_WI_PKG05/FAILED_LAUNCH_RECOVERY_SNAPSHOT_V1.json).
- [Recovery explanation](../execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N5_WI_PKG05/FAILED_LAUNCH_RECOVERY_V1.md).
- [Unmodified raw runtime summary](../execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N5_WI_PKG05/RUNTIME_SUMMARY.json) and [event ledger](../execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N5_WI_PKG05/RUNTIME_EVENTS.jsonl).
- [Actual Phase A reviewer return](../execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N_CROSS_CONTRACT_REFUTE/RETURN.md) and [actual adapter V2 recheck](../execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N_SELFWEIGHT_ADAPTER_REFUTE/RECHECK_V2.md).
- [Root telemetry tool](../../../tools/workflow_runtime/runtime_telemetry.py), record parser required session-id and summarize all-event grouping/pairing logic.

Proposed routing: the owning governance/tooling workflow, through HELPS_HUMANS under the root runtime doctrine, should assess a bounded telemetry contract and validator change. Distinguish dispatch attempts from instantiated sessions; permit an explicit prelaunch failure without inventing a session; retain immutable failed-attempt evidence; and bind recovery to an actual replacement execution, its parentage, accepted return, and source/brief hashes. Report raw telemetry completeness separately from graph-obligation recovery rather than collapsing both into one status. Preserve backward compatibility or specify a migration strategy for historical ledgers. This proposal does not authorize root files or tooling edits.

Acceptance tests for that future bounded work:
1. A failed prelaunch with no instantiated session is reported as a failed attempt, not a missing session START/FINISH pair.
2. An instantiated session missing START or FINISH still reports incomplete; an invented recovery label cannot conceal it.
3. Recovery requires an actual replacement execution with matching parentage, objective/brief scope, accepted return and hash binding; missing, unaccepted or mismatched evidence cannot satisfy the graph obligation.
4. Failed attempts and actual session counts/durations remain distinct, preserving the original failure and replacement evidence without synthetic start/finish events.
5. The present PKG05 fixture reports two completed actual sessions and one recovered failed attempt; the original raw ledger and historical summary remain immutable evidence.
6. Existing ordinary one-START/one-FINISH ledgers retain their prior results; duplicate event IDs and invalid chronology remain rejected.

No product behavior, runtime tool, instruction, accepted authority, lifecycle, or dependency pointer changes accompany this issue plan. This is the sole new issue-plan file for this session, verified against Git changes before creation.
