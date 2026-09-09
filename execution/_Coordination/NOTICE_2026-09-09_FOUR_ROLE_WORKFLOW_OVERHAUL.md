# Four-role agent and workflow overhaul — adoption notice

Root is implementing the human-approved four-role design under D-GOV-41. The
retained roles are HELP_HUMAN (Type 0), HELPS_HUMANS and WORKING_ITEMS (Type 1),
and TASK (Type 2). Thirty specialized agent packages become workflows or tools;
the 45 existing method packages move from skills/ to workflows/. The initial
library has 71 WORKFLOW.md entrypoints.

Role prose now contains only PROTOCOL, SPEC, STRUCTURE, and RATIONALE. Machine
configuration moves to agents/registry.json. Assignments select Workflow, with
an explicit TaskSkill compatibility adapter. workflows/legacy-agents.json maps
retired identifiers to roles and workflows or tools. Direct human entry through
Type 0 or either Type 1 remains available.

## Adoption hold

This notice is coordination, not an adopting act. The receiving loop adopts,
amends, or declines under its own instruments and cadence. Consumers of the old
frontmatter, Agent Type table, retired agent paths, or skills/SKILL.md layout must
retain their accepted instruction basis until they adopt the replacement registry,
loader, packaged resources, and brief handling together. Root export staging is
held for incompatible clients and does not constitute a public release.

Follow-on verification covers role discovery and direct entry, capability and
command restriction composition, explicit resource loading, caller/brief migration,
child execution boundaries, and compatibility handling. Update owned corpus
references and SHA-pinned mirrors only through their owning adoption instruments.
Preserve historical snapshots and evidence bytes.

## Affected surfaces

The project entry instructions, loop launchers, workflow references, and historical
agent-run evidence refer to the former roster. The project owns their live caller
migration and any App/Runtime basis it uses. Frozen instruction-basis manifests and
accepted execution evidence remain unchanged. Repoint live specialized role
invocations through WORKING_ITEMS and the appropriate workflow when adopting the
replacement basis.

## Root evidence

The replacement contract is docs/AGENT_WORKFLOW_RUNTIME.md. Source dispositions,
original hashes, downstream reference census, implementation returns, and validation
are recorded in execution/_Coordination/AgentRuns/FOUR_ROLE_OVERHAUL_20260909/.
The new Root decision and instruction tranche manifest identify this authorized
change; the Root handoff records final validation and remaining adoption work.
