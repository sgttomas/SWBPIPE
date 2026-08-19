# Sealed Launch Brief — WI-PKG10-DEL1004-DEC093

- RequestedBy: `HELP_HUMAN`
- RunID: `HELP-HUMAN-PIPING-20260819-DEC093-CI-SURFACE4`
- ParentInstanceID: `HELP-HUMAN-PIPING-20260819-DEC093-CI-SURFACE4`
- ChildInstanceID: `WI-PKG10-DEL1004-DEC093`
- Role: `WORKING_ITEMS` (Agent 1)
- PackageID: `PKG-10`
- DeliverableID: `DEL-10-04`
- Objective: implement the separately authorized DEC-093 CI-bound surface-4
  path in the evidence-sweep tooling and its summary schema/validator as
  applicable, including rejection when the bound CI head SHA differs from the
  sweep `commit_hash`, while preserving the host path unchanged.
- AcceptedBasis: D-65 ruling; DEC-093; DAG-009; Receipt 114; live DEL-10-04
  `## Remaining`; current R5 stage; standing development-pressure steer.
- Dependencies: DAG-009 active EXECUTION UPSTREAM edges for DEL-10-04 are
  satisfied; no predecessor node in this tranche.
- ScopePath: `projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-04_Build, packaging, and CI-CD pipeline`
- DeclaredReads: root/project `AGENTS.md`; `agents/AGENT_WORKING_ITEMS.md`;
  `agents/AGENT_TASK.md`; software workflow profile and project registration;
  D-65/DEC-093; DEL-10-04 kit, dependencies, and recent run records; current
  sweep tool/schema/validator/tests and workflow contract.
- AllowedWriteTargets: `projects/chirality-piping/tools/release/**`;
  `projects/chirality-piping/validation/**` only where source-controlled
  schemas/tests/fixtures require it; the DEL-10-04 folder; this run root; and,
  per brief amendment V2, exactly
  `projects/chirality-piping/tests/test_evidence_sweep.py` and
  `projects/chirality-piping/tests/test_release_gate_records_script.py`; and,
  per brief amendment V3, exactly
  `projects/chirality-piping/tests/test_release_packaging_script.py`.
  Do not write the receipt ledger; it is fan-in-only.
- AllowedTools: read, apply_patch, shell test/build commands, governed TASK
  Agent 2 dispatch under a frozen child brief.
- ExpectedOutputs: working implementation; regression tests for accepted and
  rejected CI bindings plus unchanged host behavior; DEL-10-04 `_STATUS.md`,
  `MEMORY.md`, and one concise run record; Agent 1 `RETURN.md`, `STATUS.json`,
  and run handoff evidence.
- AcceptanceCriteria: exact D-65 binding fields are recorded and validated;
  `ci` is a supported execution capability alongside `host`; head SHA must
  equal `commit_hash`; successful registered dual-viewport e2e execution is
  represented and validated; same-SHA reruns are bindable; spec failure is
  not accepted; host semantics remain unchanged; focused tests pass; changes
  stay inside allowed targets.
- EXCLUSIONS: no workflow change; no exploratory/agent-as-user evidence; no
  macOS-specific CI substitution; no DEC-025 historical rewrite; no lifecycle,
  stage, release, publication, or professional-reliance act; no receipt, commit,
  push, PR, or merge.
- Escalation: stop and report any required scope outside allowed writes, new
  evidence semantics not fixed by D-65, contradiction in live authority, or
  inability to identify an executable focused proof surface.
- ModelAttribution: inherited OpenAI Codex runtime capability; exact model is
  exposed by the parent runtime as GPT-5-based Codex; no substitution planned.

The manager may use the single-manager path or dispatch bounded Agent 2 work.
If it dispatches a child, freeze the intra-package graph and child brief under
this instance before dispatch, and record terminal return/status.
