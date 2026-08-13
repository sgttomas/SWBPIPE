# Launch Brief — N3 Verification-owning WORKING_ITEMS

RequestedBy: `HELP_HUMAN`; RunID and ParentInstanceID:
`HELP-HUMAN-PIPING-20260811-DEC025-MECHANICS-RUNNER-BINDING`;
ChildInstanceID: `N3-WORKING-ITEMS-CROSS-PACKAGE-VERIFY`.

ManagedPackage: `PKG-10`; SelectedDeliverable: `DEL-10-05`.
BoundedDependencyRead: accepted `PKG-09 / DEL-09-01` N1 output only. This
instance does not manage, mutate, or close PKG-09.

Objective: execute only N3 of owner-adopted
`CB-2026-08-11-DEL1005-MECHANICS-RUNNER-BINDING-001` through exactly one fresh,
non-repairing ephemeral Agent 2 and validate its durable return.

AcceptedBasis: branch/base
`codex/piping-dec025-case-runner-binding-20260811` /
`f1e311fb7ab1c2a0800b1d32c59445368428dee9`; candidate SHA-256
`c8d2b63d6a1649102e2b233542975cc8e8062e582914900a2ec75c5acc961626`;
N1 handoff SHA-256
`44138d6aa89f4c6420912ef894efa2ef280771cf2c3415e0e9399fa097fb10de`;
N1 source SHA-256/blob
`18e7c1865dbd5fd07891562b98ea54c794b0227d7bf056c95e567c6e6de3c2b5` /
`a0d8b4a269a5dc0f9d2f868a0a0cec97bacb0197`; N2 handoff SHA-256
`69bc56ab62fb7213286fe415a670bbe92049e143700430d510e190ee331e4562`;
N2 source SHA-256/blob
`4a45a0889391046fe6ab887409c791a2148bc30205478138a142af07fd4f1e6f` /
`455b9e9dee1a6a8154f65a1b5218dd7e09b6444f`; C-B SHA-256/blob
`1829a2a6a608dbbdd479248133f2c2a05406c0eabe7a03be57b998735fbe78b6` /
`468d6dd4a85525b64989ff520a5f4ff10e7c6e6f`.

Dependencies: N1/N2 `ACCEPT`, their fresh verifiers `PASS`, staged zero,
ignored zero. WorkGraph: `TERMINAL_FAN_OUT_IN`, sole node
`A2-N3-CROSS-PACKAGE-VERIFY`; no repair or retry.

ManagerAllowedWrites: this manager launch/status/return/handoff and child
launch/status/return beneath the existing run root only. Child durable writes:
only its `STATUS.json` and `RETURN.md`; all test/build/cache/temp/log/output
writes external beneath `/private/tmp/n3-cross-package-verify-20260812.*`.

Acceptance: exact identities; mechanics 25/25, 206/206, new 14/115, exact
original 11/91 complete case objects; six fail-closed negatives; stress
unchanged; nonlinear 5/5; C-B exact and nonlinear-only; fmt/tests/diff/scope;
staged/ignored zero. Return `COMMIT_SAFE` only if all pass together.

Exclusions: repairs; production/state/memory/run-record/receipt/fixture/page/
policy/TM/lifecycle edits; provisioning/network; repository build outputs;
