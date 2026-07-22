# N4H Sealed Brief — Empty units-manifest remediation attempt 8

AgentRole: TASK  
TaskSkill: `software-bounded-implementation`  
ApplyEdits: true

Authorized writes only:

- `projects/chirality-piping/core/handoff/exporter/workflow.py`
- `projects/chirality-piping/tests/test_handoff_export_workflow.py`
- `instances/N4H/**`
- exactly one new attempt-8 sweep artifact after all gates.

Restore frozen-base falsy behavior for an empty `units_manifest` Mapping.
With `{}`—including when `source_context.units_manifest_ref` is null/matching—
emit the existing missing-field blocker; return blocked, no payload, no
materializer/sink side effect, sanitized evidence and truthful counts. Preserve
non-empty valid mappings and every prior closure. Add exact regressions.

Run focused plus complete piping/desktop/build/H4/Rust/harness/self-check and
claims/path/receipt/JSON/protected-state/diff/containment gates. Before sweep,
report terminal green, inventory, zero test-results, prior seven hashes, count
zero and exact command; wait. Run one sweep only, never rerun, hash artifact,
post-check, finalize SUCCESS/BLOCK. No state/receipt/lifecycle/release/Git.

