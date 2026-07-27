# NOTICE — D-APP-48 / D-30 Metadata Mismatch

**Date:** 2026-07-27

**From:** Chirality App Dev / D-APP-76 route

**To:** Chirality Piping coordination loop

**Basis:** `c487b7dd57a378e2f74417118e78e7f61a161629`

**Status:** factual coordination notice; non-authoritative

**Related instruments:** D-APP-48, D-APP-76, D-T0-07, D-T0-09, D-30, D-31,
DEC-063

## Observed mismatch

At the stated basis:

- D-APP-48's ruling names source commit
  `ee290e22a8c19d46fb8004114d2ede55b805fba4`.
- The current D-APP-48 live JSON names descendant commit
  `55a066fdff6877d8aa2a49ce08a545ac98872848`.
- Piping's D-30 consumption record remains pinned to
  `ee290e22a8c19d46fb8004114d2ede55b805fba4`.
- The recorded combined validator command exits `1` with:
  `ERROR: consumption source.commitSha mismatch`.

Both commits resolve. This is not an unreachable-object defect.

## Current limits

No repin or Root-owned successor has been authorized. D-APP-76 selects only a
future Root-owned successor preparation route and authorizes no deliverable
activation, preparation write, identity, version, commit, implementation,
repin, or lifecycle transition.

Piping's present D-30 posture remains metadata/validator-only. Piping is not a
current Root-runtime source client, and this notice does not make it one.
Claims that D-30 is synchronized to the current D-APP-48 live JSON must not be
relied upon while the combined validator fails.

## Routed follow-on

After an exact successor disposition is separately accepted:

1. the accepted identity and compatibility effect will be routed to Piping;
2. the Piping owner may retire the App-era D-30/D-31 current-mechanism claim or
   adopt a separately accepted successor;
3. D-30 and D-31 remain historical acts; and
4. any forward DEC-063/decomposition amendment belongs to Piping
   `SCOPE_CHANGE`.

This notice is coordination, not authority. It assigns no work, changes no
Piping decision or decomposition, and requires no immediate repin.
Acknowledgement is tracked but does not gate App or Root closure unless a
future named dependency expressly requires it.
