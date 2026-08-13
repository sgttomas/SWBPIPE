# Sealed Launch Brief — A2-N3 Fresh Cross-package Verifier

RequestedBy: `N3-WORKING-ITEMS-CROSS-PACKAGE-VERIFY`; RunID:
`HELP-HUMAN-PIPING-20260811-DEC025-MECHANICS-RUNNER-BINDING`;
ParentInstanceID: `N3-WORKING-ITEMS-CROSS-PACKAGE-VERIFY`;
ChildInstanceID: `A2-N3-CROSS-PACKAGE-VERIFY`.

AgentType: fresh ephemeral Agent 2; delegation forbidden; repair authority
false. PackageID: `PKG-10`; DeliverableID: `DEL-10-05`.

BoundedIntegrationScope: read-only verification of the accepted PKG-10 runner
consumer against the exact accepted PKG-09 mechanics-suite dependency.

Objective: independently verify N3 and return `COMMIT_SAFE` or `BLOCK` without
repairing any finding.

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
`468d6dd4a85525b64989ff520a5f4ff10e7c6e6f`; frozen output SHA-256
`e41f8545cf6fc0603cb41a0bdc08d90726f59ce57e13ad6809036589db3ded8f`.

DeclaredReads: governing instructions/profile, candidate/N1/N2 records,
candidate source delta, frozen input/output, mechanics/runner/stress/nonlinear
manifests/source/tests, and C-B.

AllowedTools: read-only Git/filesystem inspection, `rg`, `shasum`,
`git hash-object`, `git diff`, external copy, offline Cargo fmt/test/run, and
Python read-only JSON analysis. No network or provisioning.

AllowedWriteTargets: durable only this instance's `STATUS.json` and
`RETURN.md`; ephemeral one fresh external root matching
`/private/tmp/n3-cross-package-verify-20260812.*`. Retain it; do not delete.

ExecutionRequirements:

1. Resolve `REPO_ROOT`/`WORKING_ROOT`; validate branch/HEAD, all hashes and
   handoffs, staged/ignored zero, and exact two-path production delta.
2. Create the fresh external root; copy complete Piping tree to
   `shadow/chirality-piping`; copy existing N2 verifier Cargo cache from
   `/private/tmp/a2-10-verify-20260812-77uFjL/cargo-home` to own `cargo-home`.
   Set own `CARGO_HOME`, `CARGO_TARGET_DIR`, `TMPDIR`, and offline mode; do not
   repurpose `HOME`.
3. Prove shadow/source identity. Run mechanics and runner fmt checks, then
   complete offline tests separately. Require mechanics 41/41 and runner
   library 38/38, preview 1/1, final binary 15/15, zero failures.
4. Execute the registered whole-suite mechanics input through the shadow
   runner with explicit local/private intent. Independently analyze 25/25,
   206/206 finite complete comparisons, exact 14/115, exact complete case-
   object equality for frozen 11/91, zero mismatch/block/diagnostic, and
   unique/exact names.
5. Require direct PASS for six negatives: unknown, incomplete, duplicate,
   non-finite, suite failure, and name mismatch. Verify unchanged stress and
   nonlinear 5/5 from full runner tests and source identity to HEAD.
6. Prove C-B exact and nonlinear-only by inspecting the candidate diff.
7. Run `git diff --check`; prove exact two-file production containment; prove
   final staged/ignored zero and source identities unchanged.
8. Write structured return and terminal status; `BLOCK` on any failure.
