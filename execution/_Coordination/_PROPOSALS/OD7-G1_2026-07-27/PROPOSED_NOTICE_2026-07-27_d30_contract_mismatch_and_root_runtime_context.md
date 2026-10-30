# Coordination Notice — D-30 contract mismatch and Root runtime context

Routed by: Root loop, `HELP_HUMAN` / `HELPS_HUMANS`, 2026-07-27, to the
OpenPipeStress/Piping loop.

This notice is coordination, not authority. The Piping loop records, adopts,
amends, declines, or defers any local response under its own instruments and
cadence. Delivery is required; acknowledgement is tracked but does not gate
the issuing loop's closure.

## Current observed fact

At `main@fb16e32ed60bb4f384cf1e07a83c4a14ff63bbae`, this command:

```sh
python3 tools/coordination/validate_harness_contract_pull.py \
  --pull-contract projects/chirality-app-dev/execution/_Coordination/_DECISIONS/D-APP-48_FLOW_A_PULL_CONTRACT_2026-07-04.json \
  --consumption-record projects/chirality-piping/execution/_Coordination/_DECISIONS/D-30_HARNESS_CONTRACT_CONSUMPTION_2026-07-04.json
```

exits `1` with:

```text
ERROR: consumption source.commitSha mismatch
```

The Piping D-30 record names source commit
`ee290e22a8c19d46fb8004114d2ede55b805fba4` and tool registry
`harness-tools.v6.mutating-mcp`. The current D-APP-48 record names source
commit `55a066fdff6877d8aa2a49ce08a545ac98872848` and tool registry
`harness-tools.v14.headless-preview-live`.

This establishes record drift and failure of the named synchronized-
consumption check. It does not by itself establish semantic incompatibility.

## Reliance and history

D-30 and D-31 remain valid historical owner acts. Until a later Piping owner
disposition, no workflow may rely on D-30's named claim of current
synchronized App-contract consumption.

No D-30 repin, replacement, successor contract, compatibility finding, or
retire-versus-replace choice is authorized by this notice. That substantive
choice remains OD7-G5 after the App/Root D-APP-48/49 contract disposition.

## Runtime boundary

D-GOV-20 and D-GOV-28 place generic runtime semantics under Root stewardship.
D-GOV-20 excluded Piping from the initial runtime pilot, and no later accepted
act gives Piping a runtime dependency or client scope. Piping therefore
remains a non-client. This notice does not change that status.

## Scope limits and follow-on

- no contract repin or successor;
- no D-APP-48/49 disposition;
- no runtime dependency or client scope;
- no Piping PRD, decomposition, package, deliverable, dependency, lifecycle,
  implementation, release, or professional-reliance change; and
- no SCOPE_CHANGE gate.

Record delivery and track acknowledgement. Rerun the combined check after an
accepted App/Root contract disposition or Piping successor act; do not infer a
green successor from this notice.
