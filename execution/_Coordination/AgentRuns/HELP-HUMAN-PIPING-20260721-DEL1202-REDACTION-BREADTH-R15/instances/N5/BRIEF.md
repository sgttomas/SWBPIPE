# N5 Sealed Brief — Fresh DEL-12-02 implementation verification

AgentRole: TASK  
TaskSkill: `software-code-review`  
ApplyEdits: false  
RequestedBy: HELP_HUMAN through WORKING_ITEMS  
RunID: HELP-HUMAN-PIPING-20260721-DEL1202-REDACTION-BREADTH-R15  
ParentInstanceID: WORKING_ITEMS-PKG12-DEL1202-R15  
ChildInstanceID: N5  
PackageID: PKG-12  
DeliverableID: DEL-12-02  
AcceptedBasis: `0c066652cd527eb1559f715e914262d2bda42602`  
Branch: `codex/piping-pkg12-redaction`

## Objective

Independently review the complete N4 diff and evidence. Return only
`COMMIT-SAFE` or `BLOCK`. Review 100% of the 31-route matrix, exact write-fence
containment, fixed contexts, wrapper-owned intent, source-intent stripping,
materialization/withholding, no-exposure side effects, report/runner behavior,
contract parity, preservation constraints, tests, evidence, and absence of
premature deliverable state.

## Binding inputs

- adopted candidate v6 and `OWNER_GATE_3_DECISION.md`;
- `ROUTE_MATRIX.csv`, `ROUTE_VERIFICATION_DISPOSITIONS.csv`,
  `AFFECTED_OWNER_MAP.csv`, N3F return, N4 brief/return/run record/check
  evidence, and `N4_FAN_IN.md`;
- full working-tree diff from frozen `HEAD`;
- existing redaction, report, runner, handoff, and software-workflow contracts.

## Required review

- Validate every changed path against candidate §6 before judging code.
- Trace all 31 routes through projector → unchanged contract/mirror →
  materializer → exposure gate. Check each disposition exactly.
- Prove known-private retention depends only on wrapper-owned intent; payload
  true/false intent cannot affect route behavior; unknown local values retain
  unchanged `warning_only` semantics.
- Check `DOTH-CAEPIPE-LOCAL-006` remains exclusively fixed `local_private`.
- Check specialized formats withhold safely and blocked routes create no
  file/href/IPC/stdout payload side effect.
- Check report raw DOM/pre-IPC/iframe/save/print suppression, hashes, and
  renderer/linter composition.
- Check both runner binaries, exits 0/1/2, all existing verbs, and unchanged
  `export-results` stub; no new verb/transport.
- Check protected/release tools unchanged, parity vocabulary/corpus preserved,
  source non-mutation, units/provenance/diagnostics/sandbox/claim states, and
  no plugin/bug-report feature.
- Validate N4's evidence and exactly-one sweep. Re-run only read-only focused
  checks needed to resolve findings; do not execute another evidence sweep.
- Confirm N4/N5 made no deliverable state, final run-record, receipt, or Git
  closeout changes.

## Boundaries and return

Read-only. No repair, edits, lifecycle acceptance, state closeout, receipt, or
Git mutation. Do not delegate. A material unknown or defect is `BLOCK` with
file/location, impact, evidence, and remediation direction. `COMMIT-SAFE`
means valid for WORKING_ITEMS fan-in and releases only DEL-12-02 W3 closeout;
it does not authorize merge or lifecycle/release acceptance.

