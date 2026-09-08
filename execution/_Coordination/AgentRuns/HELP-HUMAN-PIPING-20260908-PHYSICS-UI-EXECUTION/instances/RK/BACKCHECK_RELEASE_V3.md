# RK K8 V3 backcheck release

Review release: final bounded K8 V3 successor backcheck.

- Source checkout: `chirality-physics-ui-execution-20260908`
- Source SHA: `779dedb8670625b36af07b89fc5557470e47c50e`
- Subject manifest: `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-EXECUTION/instances/K8/MANIFEST_V3.json`
- Required manifest SHA-256: `24d368585f1b1a7f01d79dd6201ae44fa0bfb20bc0aea72d41e5ca2c5ffcdf4c`
- Root precheck: 16/16 V3 outputs rehashed; writer freeze active.
- Prior inputs: K8 V1/V2 and RK first-pass/V2 records remain frozen.

Scope is limited to the V3 successors and closure of `RK-V2-001` and `RK-V2-002`, with regression confirmation that `RK-003` remains closed. Review includes isolated focused builder/adversarial reproduction and concrete malformed-shape, ingress, version, refusal, and mapping probes. It excludes source or author writes, Cargo, the full harness, Git mutation, delegation, sibling records, public adoption, and physics adoption.

RK writes are restricted to `instances/RK/**`. Active record paths remain repository-relative or use `{TASK_TEMP}`.
