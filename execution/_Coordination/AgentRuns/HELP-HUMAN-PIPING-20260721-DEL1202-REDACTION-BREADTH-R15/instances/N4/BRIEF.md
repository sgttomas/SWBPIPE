# N4 Sealed Brief — DEL-12-02 redaction breadth implementation

AgentRole: TASK  
TaskSkill: `software-bounded-implementation`  
ApplyEdits: true  
RequestedBy: HELP_HUMAN through WORKING_ITEMS  
RunID: HELP-HUMAN-PIPING-20260721-DEL1202-REDACTION-BREADTH-R15  
ParentInstanceID: WORKING_ITEMS-PKG12-DEL1202-R15  
ChildInstanceID: N4  
PackageID: PKG-12  
DeliverableID: DEL-12-02  
AcceptedBasis: `0c066652cd527eb1559f715e914262d2bda42602`  
Branch: `codex/piping-pkg12-redaction`

## Objective

Implement adopted candidate v6 completely: route all 31 frozen export-capable
surfaces through the existing redaction contract using fixed route contexts,
wrapper-owned local-private intent, route-specific leaf projection, safe
materialization, observable controlled results, and fail-closed exposure
gates.

## Binding inputs

- `CANDIDATE_BRIEF_2026-07-21_DEL-12-02_REDACTION_BREADTH.md` v6;
- this run's `OWNER_GATE_3_DECISION.md`, `ROUTE_MATRIX.csv`,
  `ROUTE_VERIFICATION_DISPOSITIONS.csv`, `AFFECTED_OWNER_MAP.csv`,
  `VALIDATION.md`, and N3F return;
- `software-workflow.json` and the `software-bounded-implementation` skill;
- existing Python/TypeScript redaction contract, schema, parity corpus, tests,
  report/runner/handoff contracts, and accepted project doctrine.

## Exact write fence

Candidate v6 §6 is incorporated verbatim as the write fence. It permits only
the named shared redaction contract/schema/fixture/tests; eight named Python
final sinks and focused tests; named desktop report/service/e2e files plus the
frozen download-owning feature folders' existing `*Panel.tsx` and focused
tests; named headless runner/schema/tests; redaction documentation; and N4
evidence under this run root. No other path is writable.

`core/reporting/protected_content_linter/**`, both recorded release tools, all
deliverable `_STATUS.md`/`MEMORY.md`/final run records, loop receipts, Git
metadata, and all paths outside §6 are read-only. Stop and return on any need
outside the exact fence.

## Required behavior and prohibitions

- Preserve the existing action/context/reason vocabulary and parity corpus;
  do not change the underlying contract's item-intent behavior.
- Route projectors strip/override payload-carried intent; only wrapper-owned
  UI/CLI/writer intent authorizes known-private retention.
- Unknown local-private values retain existing `warning_only` behavior and
  remain epistemically unknown.
- `DOTH-CAEPIPE-LOCAL-006` is exclusively fixed `local_private`; it is not in
  downstream-tool `DOTH-FORMAT-003`.
- Preserve source non-mutation, units/dimensions, safe provenance/checksums,
  diagnostics, sandboxing, report controls, and claim-state distinctions.
- Specialized formats withhold the whole member when safe materialization is
  impossible.
- Do not add a runner verb, plugin/bug-report product, transport, cloud/network
  behavior, telemetry behavior, storage root, or professional claim.
- `export-results` remains the existing stub.

## Verification and return

Run focused route/contract tests, then affected registered checks selected by
`software-workflow.json`, including desktop test/build, H4 Playwright evidence
for visible flows, piping pytest, relevant Rust tests, exactly one DEC-025
evidence sweep, practitioner-harness pytest/self-check, path/claims/JSON/scope
and `git diff --check` validation as applicable. Record any environment-limited
native GUI evidence honestly.

Return implemented files, behavioral summary, exact check evidence and exit
codes, write-fence validation, residual risks, and blockers. Do not write
deliverable state or perform commit/push/PR/merge. Do not delegate.

