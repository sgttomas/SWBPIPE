# N4F Sealed Brief — Exact N5E remediation attempt 6

AgentRole: TASK  
TaskSkill: `software-bounded-implementation`  
ApplyEdits: true  
RequestedBy: HELP_HUMAN through WORKING_ITEMS  
RunID: HELP-HUMAN-PIPING-20260721-DEL1202-REDACTION-BREADTH-R15  
ChildInstanceID: N4F  
PackageID: PKG-12  
DeliverableID: DEL-12-02  
FrozenBase: `0c066652cd527eb1559f715e914262d2bda42602`

## Objective

As the sole active serialized implementation owner, remediate exactly N5E's
two findings without widening candidate v6 or disturbing attempts 1–5.

## Authorized product/test write paths

- `projects/chirality-piping/core/handoff/exporter/workflow.py`
- `projects/chirality-piping/core/security/redaction/route_control.py`
- `projects/chirality-piping/tests/test_handoff_export_workflow.py`
- `projects/chirality-piping/apps/desktop/e2e/r2-smoke.spec.ts`

Evidence writes are limited to `instances/N4F/**` plus the single new
attempt-6 sweep artifact after all gates. Do not edit prior attempts/sweeps,
candidate/maps, state/memory/deliverable records, receipts, policy, schemas,
other product/tests, lifecycle/release, or Git.

## Exact requirements

### A. Additive source-diagnostic gate

- Preserve the distinction between source workflow diagnostics and redaction
  findings, but compose both into the final exposure decision.
- A source diagnostic/finding with blocking severity is blocking regardless of
  whether its code/severity/path/value is later sanitized for outward evidence.
- On any source blocker: return `blocked=true`; expose no payload; perform no
  file, href, handoff, or other sink side effect; retain sanitized decisions,
  findings, and summary sufficient to explain the block.
- Nonblocking source diagnostics retain their governed warning behavior.
- Restore/strengthen exact tests for `EXP-UNIT-METADATA-MISSING`, blocking
  status, payload withholding, evidence visibility, and no side effect.
  Explicitly supersede only A1's weakened expectations; preserve all unrelated
  coverage and N4E safe-key behavior.
- Do not add public authority, structural allowlists, safe-key inference, or
  compatibility exceptions.

### B. Restore pre-A3 H4 flow

- Remove the attempt-5-added
  `page.getByTestId("rendered-report-render").click()` from the controlled
  export assertion sequence.
- Do not add a replacement action or otherwise change the interaction flow.
- Keep only assertion changes necessary to require redaction of unmetadataed
  leaves reachable in the pre-A3 flow. Preserve unrelated H4 coverage.

## Verification and sweep discipline

Run focused workflow and H4 checks first, then the full registered union:
piping pytest, desktop Vitest/build, H4 source+production-dist with external
output, relevant Rust, practitioner harness+self-check, claims/path/receipt/
JSON/protected/state/diff validators, and complete tracked/untracked plus
ignored test-results containment.

Record failed intermediate evidence honestly. Before any sweep, report a
terminal PASS/BLOCK, complete dirty-path count, zero test-results, exact prior
hashes, attempt-6 sweep count zero, and exact intended command. Wait for parent
acknowledgement. Then run exactly one registered attempt-6 evidence sweep and
never rerun it. Hash the sole artifact and perform only post-sweep read-only
checks/cleanup of the known generated ignored `.last-run.json` if present.

## Return

Return `SUCCESS` or `BLOCK` with changed paths, test/validator results,
containment, sweep count/path/hash, residual limitations, and explicit no
state/receipt/Git effect. Fresh N5F remains mandatory.

