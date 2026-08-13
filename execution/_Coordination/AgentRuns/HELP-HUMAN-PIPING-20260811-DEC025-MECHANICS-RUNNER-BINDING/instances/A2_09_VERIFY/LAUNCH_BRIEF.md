# Sealed Brief — A2-09 Fresh Observation API Verifier

RequestedBy: `N1-WORKING-ITEMS-PKG09-DEL0901`

RunID: `HELP-HUMAN-PIPING-20260811-DEC025-MECHANICS-RUNNER-BINDING`

ParentInstanceID: `N1-WORKING-ITEMS-PKG09-DEL0901`

ChildInstanceID: `A2-09-VERIFY`

Agent form: fresh non-delegating, non-repairing ephemeral Agent 2

PackageID / DeliverableID: `PKG-09` / `DEL-09-01`

## Objective

Independently verify the exact accepted author candidate in
`validation/benchmarks/mechanics/src/lib.rs` at SHA-256
`18e7c1865dbd5fd07891562b98ea54c794b0227d7bf056c95e567c6e6de3c2b5`
and Git blob `a0d8b4a269a5dc0f9d2f868a0a0cec97bacb0197`. Return PASS or BLOCK;
perform no repair.

## Accepted basis and required reads

Resolve active checkout paths. Branch/HEAD is
`codex/piping-dec025-case-runner-binding-20260811` /
`f1e311fb7ab1c2a0800b1d32c59445368428dee9`. Candidate SHA-256 is
`c8d2b63d6a1649102e2b233542975cc8e8062e582914900a2ec75c5acc961626`.
Read root/project AGENTS, adopted candidate, baseline and current `lib.rs`,
mechanics Cargo.toml, frozen runner output, C-B file, N1/A2 author launch and
return/status, DEL-09-01 contract/state/context/dependencies, and software
workflow profile.

## Allowed tools and writes

Read-only shell/Git/hash/search plus mechanics `cargo fmt --check` and full
offline `cargo test`. Externalize all target/cache/temp output beneath a new
`/private/tmp` directory. No network/provisioning. Do not delegate. Write only
this instance's `RETURN.md` and `STATUS.json`; source/project truth is read-only.

## Verification contract

1. Recompute exact 14 case IDs and counts `2,2,6,3,3,2,1,2,21,42,12,6,8,5`
   = `115`; verify exact expected-name order, finite values, no missing/extra/
   duplicate name, and suite-owned comparison PASS for all 115.
2. Audit the observation producers to reject any expected numeric value used
   as an observation. Expected values may supply names/comparison arguments
   only.
3. Verify the two public APIs are additive, deterministic, fail closed for
   unknown/non-finite/malformed sets, and use only existing suite predicates/
   constants with no new tolerance or public policy.
4. Prove original behavior preservation: source delta from baseline is
   addition-only; inventory/expected-values/predicate/provenance definitions
   are unmodified; original frozen 11-case/91-observation runner output bytes
   remain exact; all existing tests plus new tests pass.
5. Verify C-B SHA-256 remains
   `1829a2a6a608dbbdd479248133f2c2a05406c0eabe7a03be57b998735fbe78b6`
   and no mechanics path reads it.
6. Verify format, diff, exact N1 write containment, zero staged/ignored drift,
   and no fixture/page/Cargo/state/evidence/receipt/PKG-10/Git mutation.

Return exact commands/results, source identities, coverage, original-behavior
proof, C-B/containment, findings, blockers, and rerun triggers. Do not repair,
stage, commit, push, PR, merge, fetch, rebase, reset, clean, delete, or edit any
other path.

