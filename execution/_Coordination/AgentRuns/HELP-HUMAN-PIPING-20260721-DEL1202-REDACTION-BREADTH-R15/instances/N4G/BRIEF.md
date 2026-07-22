# N4G Sealed Brief — Exact N5F remediation attempt 7

AgentRole: TASK  
TaskSkill: `software-bounded-implementation`  
ApplyEdits: true  
RequestedBy: HELP_HUMAN through WORKING_ITEMS  
RunID: HELP-HUMAN-PIPING-20260721-DEL1202-REDACTION-BREADTH-R15  
ChildInstanceID: N4G  
PackageID: PKG-12  
DeliverableID: DEL-12-02  
FrozenBase: `0c066652cd527eb1559f715e914262d2bda42602`

## Authorized write paths

- `projects/chirality-piping/core/handoff/exporter/workflow.py`
- `projects/chirality-piping/core/security/redaction/route_control.py`
- `projects/chirality-piping/tests/test_handoff_export_workflow.py`
- `instances/N4G/**` and exactly one new attempt-7 sweep artifact after gates.

No other write is authorized. Preserve attempts 1–6 and all prior evidence.

## Exact requirements

- Validate missing, null, and non-Mapping mapping contract, target fixture, and
  manifest units values before any `.get`, iteration, membership, or other
  dereference.
- Emit exactly the existing declared diagnostic code for each invalid source:
  `EXP-TARGET-MAPPING-MISSING`, `EXP-TARGET-FIXTURE-MISSING`, or
  `EXP-HANDOFF-MANIFEST-FIELD-MISSING`; preserve the governed blocking class.
- Feed every emitted blocker through the existing additive source-finding gate.
  Result must be blocked, payload absent, materializer/sink uncalled, and
  sanitized decision/finding/summary evidence present.
- For lossless-only withholding, expose/count the real lossless exposure
  blocker so total exposure blockers are nonzero and consistent with blocked.
  Preserve existing lossless behavior and do not add unrelated findings.
- Add exact regressions for missing key, explicit null, and representative
  non-Mapping values for all three inputs, plus lossless-only count/evidence.
  Preserve all prior workflow coverage.

## Verification and sweep discipline

Run focused tests then full piping, desktop test/build, H4 source/dist with
external outputs, relevant Rust, harness+self-check, claims/path/receipt/JSON/
protected-state/diff validators, and complete tracked/untracked plus ignored
test-results containment. Record intermediate failures.

Before any sweep, report terminal green, exact inventory, zero test-results,
unchanged prior six hashes, attempt-7 count zero, and exact command; wait for
acknowledgement. Invoke exactly one registered attempt-7 sweep, never rerun,
hash its sole artifact, and perform only post-sweep read-only checks/generated
`.last-run.json` cleanup if needed.

Return `SUCCESS` or `BLOCK`; no state/receipt/lifecycle/release/Git effect.
Fresh N5G remains mandatory.

