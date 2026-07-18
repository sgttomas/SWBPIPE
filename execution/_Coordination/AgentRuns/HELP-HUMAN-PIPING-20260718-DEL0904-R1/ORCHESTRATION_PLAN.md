# Orchestration Plan — HELP-HUMAN-PIPING-20260718-DEL0904-R1

**Plan version:** v1 (frozen by HELP_HUMAN before dispatch)

**Manager:** ORCHESTRATOR

**Instance:** `ORCHESTRATOR-DEL0904-BRIEF-01`

**Parent:** HELP_HUMAN

**Package:** PKG-09

**Deliverable:** DEL-09-04

**Posture:** `TERMINAL_FAN_OUT_IN` (descriptive)

**Selection authority:** Agent 0 applying the owner-adopted piping loop plan

## Objective

Prepare and validate one owner-facing CANDIDATE brief for the selectable
DEL-09-04 actor-neutral clean-checkout reproduction Remaining item. Do not
execute the reproduction and do not cross the owner-adoption gate.

## Work Graph

One node only: ORCHESTRATOR inspects the bounded accepted basis, writes the
candidate brief and managed-run record, validates the outputs, and returns to
HELP_HUMAN. No concurrency and no downstream execution node are authorized.
The frozen machine-readable graph is `WORK_GRAPH.json`.

## Read Scope

Read only the accepted surfaces named in the launch brief: root/project agent
instructions; loop plan and Receipt-55 cursor; current coordination stage and
relevant decision rows; DAG-007 pointer, approval, and DEL-09-04 active
execution-upstream rows; software workflow profile; the selected deliverable
and active dependencies; amended PRD sections; DEC-080/D-47; and the documented
E1 procedure plus directly cited live commands/scripts. Derivative claims are
checked against those live sources.

## Write Scope

Only:

- `execution/_Coordination/CANDIDATE_BRIEF_2026-07-18_DEL-09-04_CLEAN_REPRODUCTION.md`
- `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260718-DEL0904-R1/**`

No deliverable, evidence, code, test, register, decomposition, DAG, receipt,
or other project content may be changed in this planning run.

## Fan-In Gate

The node returns only after:

1. the candidate is explicitly CANDIDATE / awaiting owner adoption;
2. required purpose, basis, tasks, evidence, later write fence, command
   resolution, acceptance, failure disposition, DEC-025 judgment, rerun
   triggers, exclusions, gates, and pending adoption section are present;
3. all required managed-run artifacts exist;
4. JSON, whitespace, containment, path-anchor, claims-language, receipt, and
   other applicable read-only validations pass;
5. no reproduction command has been run and no downstream execution has been
   released.

## Model and Execution Attribution

Execution mechanism: the Codex collaboration child session at canonical task
path `/root/orchestrator_del0904_brief`, launched by HELP_HUMAN. Capability:
harness-assigned current session model. No model override, named-model steer,
capability substitution, or mid-run substitution occurred. No opaque runtime
identifier or unavailable timing/token measurement is asserted.
