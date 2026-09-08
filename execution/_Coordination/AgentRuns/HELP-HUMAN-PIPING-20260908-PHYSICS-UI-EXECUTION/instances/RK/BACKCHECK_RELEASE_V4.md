# RK K8 V4 backcheck release

Review release: bounded K8 V4 closure check for `RK-V3-001`.

- Source checkout: `chirality-physics-ui-execution-20260908`
- Source SHA: `779dedb8670625b36af07b89fc5557470e47c50e`
- Subject manifest: `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-EXECUTION/instances/K8/MANIFEST_V4.json`
- Required manifest SHA-256: `a285a7aee0f276919c16db28fb74926257735b9b41dd9c7cc66b554dba1a0941`
- Root precheck: 18/18 V4 outputs rehashed successfully; writer freeze active.
- Released validator SHA-256: `1ef2e0f7730f3f2590a7e72167f1ce1f893e550e99478a0702eb62cf572f93f9`
- Released tests SHA-256: `6ced8f265facdee383700cbaafde3da744e0857a2fae0cc80a203a7fa1ee3334`

Scope is limited to the V4 successors and closure of `RK-V3-001`: prior unhashable kind/unit and non-string-key cases, the consumed-field JSON type-class matrix, validation priority, preservation of well-typed stable errors, and clean 830-row round trip. Previously closed API/version ingress, legacy 0.1 handler, and pressure sequencing are checked for unchanged retention only.

RK writes are restricted to `instances/RK/**`. The review excludes author/source writes, Cargo, the full harness, Git mutation, delegation, sibling records, optional robustness axes, public adoption, and physics adoption. Active record paths remain repository-relative or use `{TASK_TEMP}`.
