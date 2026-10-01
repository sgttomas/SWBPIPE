# Addendum 02 — C17 independent BLOCKING confirmation

**BLOCKING: C17 publishes false absolute claims for D:0 and M:0.** The unchanged
validated comparator reports exact-claim and source-binary64-claim failures for
both rows. This independently confirms the reported kernel-source observation;
it does not establish literal product/native reachability.

## Exact derivation and observed claim

The independently frozen C17 primitive definition has a spring at axial DOF 0
with stiffness k=2^-33. Its two ground-node axial loads are 2^-900 and 9h;
the other node carries -2^-900, with h=2^-1074. Adding the two axial equilibrium
equations cancels the internal member force exactly:

    k*u_0 = (2^-900 + 9h) - 2^-900 = 9h
    u_0 = 9*2^-1041
    magnitude(node 0) = |u_0| = 9*2^-1041

The source geometry and constraints allow no other translation component at
node 0. These equations match D:0 and M:0 in TRUTH.json, which was frozen before
any preserved numerical output was opened. No I22 expected value, observed
solver value, or comparator denominator was used to derive this truth.

C17 selects 128, verified at 256. Both rows publish +0.0, class AbsoluteVerified,
and bound bits 0000001000000000, meaning b=2^-1038. Their allowed publication
error is b*(1+2^-22), including the accepted publication factor. Thus:

    error = 9*2^-1041
    allowance = 4194305*2^-1060
    error / allowance = 4718592/4194305 > 1

The allowance is exactly representable in binary64 here; both the exact and
operation-by-operation source predicate fail. This is not a nearest-rounding
argument or a benchmark-scale substitution. The failure is against each row's
own verified bound and the accepted factor. The ordinary public 1e-9 relative
predicate is not the claim on these absolute rows.

The comparator checks all 36 source-layout rows. Its only claim failures are
D:0 and M:0. Direct rounding also differs on spring row S:1:0; that row remains
within its own published claim. A direct-bit mismatch alone was not counted as
a false claim. CONFIRMATION.json carries the exact values and metadata;
COMPARISON.json carries every row's result.

## Input and execution identity

Raw output: `<WT>/scratch/i22/c_01/C17/stdout.tsv`, copied without modification
into this addendum, SHA256
841167ed887a71b0e2a904647e1f217758fad304baaaf0d669f5c47035dc797a.

ROOT-released I22 packet `R/I22/c_01/SHA256SUMS` was hash-checked as
98523b1cd5d55549c872b983095037dd48676d8e049c1c973909bb448d1e2b00.
Its C17_INPUT.json is identical to the input-only C17 handoff previously bound
independently to cases.rs/main.rs before the truth freeze. Its recorded source
and probe hashes match the original pinned definitions. The sealed preflight
records the binary hash
bcbe897204ec702b99529d25e6d0213d0132af5e6086e0397fae3a8f8ef8a08f.

The log reports source 3bddc2b05f6106e969c7cf43373b230845c7cc66, case C17,
100000000/100000000 limits, Selected 128/256, the expected method/policy identity,
and matching source encodings before and after selection. Addendum 01's
independent Git tree query already establishes FK byte-identity with current
d01ad98a754698631f927709d08284c272de85e8. No extra Rust or source execution was done.

The sealed execution command clears FK_SEEDED_FAULT and compiler-wrapper/flag
variables and invokes the same binary exactly once with C17 and the fixed
limits. Recorded workload exit is 0; the strict comparator exit is 1. The packet
records the existing guard before and after, and records no continuing process
session. These are preserved execution records, not a new oracle-task guard
qualification or independent repeat. METADATA.json captures their paths/hashes
and selected execution metadata; the binary itself was not rerun or rehashed by
this TASK.

## Read order and preservation

This follow-up was explicitly released by ROOT to `/root/a1_oracle_fresh`.
No new agent or delegation, role/skill/workflow change, or expectation change
occurred. All prior seals and comparator bytes remain unchanged.

1. cb9046: read only the released stdout; require its exact supplied SHA256;
   copy raw bytes; invoke the unchanged addendum-01 wrapper and frozen comparator.
   Exit 1 and all four predicate failure labels were retained. Immediately
   report BLOCKING to ROOT before additional metadata work.
2. ba8335: independently recompute force balance from the already-frozen input
   bits and match frozen truth; inspect only the new comparison and raw headers;
   compute the exact ratio; enumerate current I22 packet filenames.
3. 48d482: hash-check I22's SHA256SUMS; read COMMANDS metadata (display partially
   truncated due to its size) and inspect C17_INPUT keys. No historical response
   oracle or comparator was imported.
4. ab16af/60665d: select preflight/execution fields, hash consulted metadata,
   verify current C17_INPUT equality with the prior independently bound handoff.
   Preserve metadata without using I22's numerical conclusion as truth.
5. Addendum seal: verify prior manifests' listed hashes, write this return and
   inventory the new files. No affected-path experiment continued.

RUN.json contains the actual comparison argv, exit and raw stdout/stderr.
The environment used was standard-library Python; GIT_OPTIONAL_LOCKS=0 was
retained. No Rust, solver rerun, host tools/change, maintained source edit,
Git/index write, parser modification or truth re-derivation occurred.

The bounded confirmation is complete. Source/native reachability, general
scale-transfer design proof, remedy selection, receipt/unit consequences,
practitioner qualification and release remain outside this return. C17 is a
kernel PrimitiveSource witness with invented extreme binary64 inputs.
