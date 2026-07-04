# D-30 - PROPOSAL: Harness-contract consumption execution

**Status:** PROPOSAL / `AWAITING_RULING` with prerequisite-dependent execution held.
**Date prepared:** 2026-07-04
**Decision ID:** D-30
**Prepared by:** bridge work loop agent, at owner direction to prepare D-30 now and hold prerequisite-dependent execution until D-APP-48 and D-T0-09 are resolved.
**Structural precedent:** Follows the D-28/D-21 packet convention for preparing DEC-041 follow-on work while preserving immutable ruled history and explicit execution gates.

This packet prepares the piping-side consumption lane. It does not add a
dependency, pull app-dev package files, bind a live agent, open F3, modify
protected paths, advance lifecycle state, or create release, professional,
certification, sealing, authentication, or code-compliance claims.

## Decision to rule

Decide how piping will consume the app-dev `@chirality/harness-contract`
package once the publishability mechanism is ruled and the concrete Flow-A
contract version is settled.

## Verified basis

| Fact | Source |
|---|---|
| DEC-041 chose harness-as-versioned-packages, rejected a Tauri refactor and Rust reimplementation, and gated execution behind D-21 plus the no-manual-toil automation condition. | `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md` DEC-041 |
| D-21 is ruled by DEC-056, but its SCOPE_CHANGE propagation residue is tracked separately as D-29. | `projects/chirality-piping/execution/_Coordination/_DECISIONS/_REGISTER.md`; `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md` DEC-056 |
| D-APP-48 owns the app-dev publishability mechanism that can satisfy the no-manual-toil condition. | `projects/chirality-app-dev/execution/_Coordination/_DECISIONS/_REGISTER.md` |
| D-T0-09 owns the concrete Flow-A contract version value replacing `TBD_BY_TIER_0`. | `_DomainEngines/_DECISIONS/_REGISTER.md`; `_DomainEngines/_DECISIONS/D-T0-09_flow_a_contract_version_value.md` |
| DEC-055 says event-vocabulary pins must use the live app-dev source at the commit being pinned, not stale DEC-041 prose. | `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md` DEC-055 |

## Prerequisite hold

Per owner direction, prepare now but hold prerequisite-dependent execution until:

1. D-APP-48 is ruled and supplies the automated pull mechanism.
2. D-T0-09 is ruled and supplies the concrete Flow-A contract version value.

D-29 remains a companion traceability lane: D-30 must not claim full v0.2/R7 or
live-binding closure until D-29 propagation is also closed or explicitly held
with a recorded handoff state.

## Options

| ID | Option | Consequence |
|---|---|---|
| **O-A** | **Consume via D-APP-48 O-A intra-repo SHA-pinned pull contract once prerequisites rule.** Add a piping-side consumption manifest/validator that names repo commit, package path, package name, Flow-A contract version, package export manifest, and validation commands. | Satisfies DEC-041 without package publication. Keeps live binding separate and auditable. |
| **O-B** | **Consume via a private or public registry after D-APP-48 selects publication.** | Aligns with registry package semantics, but waits on F2 publication/distribution authority. |
| **O-C** | **Prepare design only; no dependency consumption.** | Avoids implementation risk but leaves DEC-041 execution residue open. |
| **O-D** | **Defer.** | Keeps D-30 parked. |

## Recommendation

Recommend **O-A**, conditional on D-APP-48 O-A and a D-T0-09 concrete value.

This is the lowest-governance execution path because it uses SHA-pinned source
truth and validation instead of publication. It also preserves DEC-041's
separation between pure contract package consumption, later React UI package
consumption, and eventual Node Agent-SDK sidecar live-binding.

## Scope constraints if ruled

- No protected-path writes under `projects/chirality-piping/core/**`,
  `projects/chirality-piping/schemas/**`, or `projects/chirality-piping/core/handoff/**`
  without a separate lawful vehicle.
- No live model/provider binding or agent runtime execution.
- No `DomainEngineProfile` / `OperationProposal` source-type import into
  app-dev or piping runtime.
- No lifecycle transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim.

## Human ruling

**Ruling:** OPEN.

The owner may select O-A, O-B, O-C, O-D, or give a custom ruling. Even after a
selection, implementation remains held until D-APP-48 and D-T0-09 are resolved.
