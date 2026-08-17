# Coordination notice — Root Agent 0 direct Agent 2 dispatch alignment

Date: `2026-08-16`
From: `Chirality Root / HELP_HUMAN (Agent 0, session minder)`
To: `Chirality Piping loop`
Notice class: `AGENTS.md agent-index change-notice; coordination, not authority`

## Root change reported

Tranche `ROOT-AGENT0-DIRECT-A2-ALIGN-20260816` aligned the prose of
`agents/AGENT_HELP_HUMAN.md` with root `AGENTS.md` (2026-08-02 doctrine:
Agent 0 role-table row and "Delegation and Entry Rules"), which permits
Agent 0 to directly dispatch bounded Agent 2 instances:

- blob before `503f70dbc00127d844e3a0327ed47655a9142278`;
- blob after `f3901408e7d5f040cb1d52e6033152ccb2bf3ade`;
- "Managers own management": "Delegate only to named Agent 1 roles. Never
  bypass Agent 1 to dispatch Agent 2." → delegate to named Agent 1 roles and
  optionally directly dispatch bounded Agent 2 instances (ephemeral
  generalists or TASK) under the same sealed-brief, declared-scope, and
  durable-evidence requirements; never bypass a manager's validation of work
  it owns; a child's capability never becomes Agent 0's own; plus a factual
  sentence that the `subagents:` allowlist, the agent-instruction validator,
  and the App harness type rule still enforce Agent-1-only children pending
  their own alignment tranches;
- "Hierarchy-mediated coordination": "direct Agent 1 children" → "direct
  children";
- SPEC 3: "Only Agent 1 roles are direct children." → "Direct children are
  named Agent 1 roles or bounded Agent 2 instances (ephemeral generalists or
  TASK) under sealed briefs."

The frontmatter `subagents:` allowlist, WRITE_SCOPE, and "No
project-content writes" are unchanged. Owner ruled Option 1 (prose only)
because adding `TASK` to the allowlist fails
`tools/validation/validate_agent_instructions.py` (`AGENT0_CHILD_TYPE`) and
the App harness type rule would still block at runtime.

## Parked follow-ons (stated, not routed as Piping work)

1. ROOT — `tools/validation/validate_agent_instructions.py` L270-280 and
   `tools/validation/test_validate_agent_instructions.py` L167-183, with the
   `subagents: … TASK` frontmatter alignment, in a later root tranche.
2. APP — `projects/chirality-app-dev/frontend/src/lib/harness/managed-delegation.ts`
   L274, L289-290 and `subagent-governance.ts` L160-168 plus tests: an App
   product-code tranche under App's own review posture.

## Authority and loop autonomy

This notice does not amend the Piping basis, authorize Piping work, or change
any Piping register. Piping HELP_HUMAN sessions read the live file; the Piping
loop retains sole authority to adopt, amend, acknowledge, decline, or defer
any local follow-on under its own instruments and cadence. No Piping action is
required by this notice.

Evidence: `docs/governance_harness/tranche_manifests/ROOT-AGENT0-DIRECT-A2-ALIGN-20260816.yaml`;
`execution/_Coordination/AgentRuns/ROOT_AGENT0_DIRECT_A2_ALIGN_2026-08-16/`.
