# RV28 — prefix correction backcheck

**VERIFIED. RV28-PREFIX-1 is closed at source-design level.**
The corrected counts phase union and successful-entry clarification are
suitable for conditional K0 integration. No residual blocking or SHOULD-FIX
finding in this additive correction.

This is not full VR-PREFIX, checkpoint0, E_max, implementation, final-build
correspondence, admission replay or measurement acceptance.

Reviewed packet: R/metric_design_09_prefix_correction.
CORRECTION.md SHA256:
`3225e178ca399beb25089a800c3fb4c102a72ccd0ecf5971f86909f932ac276f`.
Packet seal:
`72582e7a4e58721e32290dfd61d7d226f4d9be681007c34763869095423f7b82`.
Original packet08 and RV28 vr_prefix_02 seals were independently checked
unchanged, including every listed payload.

TASK /root/rv28_a1_design; direct parent /root; native followup_task.
Actual start **2026-10-01 19:50:27 UTC**, after ROOT announced the sealed packet.
Deadline 20:00:27 UTC. No unsealed author output was inspected.
The same instruction/skill basis remains active; origins/hashes are inherited
from the sealed vr_prefix_02 evidence. No new role, workflow, library programme
or delegation was activated. Raw provenance is under _run_records.

Aliases: P=projects/chirality-piping; FK=P/core/solver/frame_kernel;
VR=P/validation/benchmarks/numerical_robustness; R=this resumed run.

## Receiver/growth/overlap backcheck

The immutable source remains
`40129a225d73860ac2a53da9a2fa73869df668f3`.
Fresh bounded reads confirmed:

- VR/src/scale.rs:136 ends the adjacency/profile call as a separate statement.
  Those helpers have ended before the Counts initializer.
- scale.rs:145 calls source.free_dofs().len(). FK retained/source.rs:629–633
  returns an owned range/filter/collect Vec<usize>.
- The same initializer next evaluates pattern_entries at :147,
  layout(source).len() at :151, and source.encoding().len() at :152.
- FK retained/recover.rs:101 onward begins layout with Vec::new and pushes
  QuantityMeta entries.

The already reviewed temporary-scope rule leaves the returned free-DOF buffer
alive through this full initializer. The layout buffer similarly survives its
own field through encoding. No struct-field boundary supplies an earlier drop.
All three receivers are dead on return from counts, before the named cut.

The correction's retained and construction bounds use the proper existing
operators:

    RFree   = G(usize,f)
    HFree   = RFree + epsilon*O_G(usize,f)
    RLayout = G(QuantityMeta,q)
    HLayout = RLayout + epsilon*O_G(QuantityMeta,q)

Here epsilon selects the requested or growing-move metric. Fresh filtered
collection does not inherit a cached/source allocation; layout uses pushed
capacity. The construction phase includes only its current active-old request.

The six corrected alternatives account for each sequential phase:

| Phase | Owners above unchanged outer caller/SRC0/k4src prefix |
|---|---|
| Construct adjacency | HAdj |
| Profile while adjacency is borrowed | RAdj + HProfileCount |
| Construct free receiver | HFree |
| Pattern helper while free receiver lives | RFree + HPatternCount |
| Construct layout while free receiver lives | RFree + HLayout |
| Encode while free and layout receivers live | RFree + RLayout + HEncoding(X) |

Taking their maximum is valid for the ordered lifetime structure. It neither
sums already-freed phases nor double-counts multiple active realloc-old terms.
HEncoding retains its previously reviewed own construction/move accounting.
The separate HLayout arm removes the need to assert that some different phase
must dominate layout growth.

The example f=60,000 gives 8*65,536=524,288 requested bytes on the named 64-bit
basis. The correction accurately labels that as a component amount, not a
measured allocation, runtime overflow or net change in E_max. No additional
dominance proof or numerical-margin assumption is used.

## Cut and global window remain correct

C_tV is unchanged:

    RuntimeRet_after_start + ArgsRet + separate id clone
      + selected Case children + standalone Model
      + optional external model hash + k4src hash.

RFree, RLayout and the second encoding buffer are absent from this survivor
union but remain represented in the earlier counts prefix peak. The corrected
text retains

    max(PrefixPeak, max_later(surviving cut identities + later owner union))

for both counters. It neither substitutes the heap at the cut nor an observed
prefix/no-op/baseline. Admission remains before launch under the existing rule.

## Successful-entry clarification is accurate

The correction uses the prior RV28/source12/H-R0 and closed public-facade
warrants; it does not claim a new runtime audit.

On the specified successful direct single-main, no-foreign-initializer,
non-unwinding path, the registered name Box (at most four bytes), registry
mutex child and first ThreadInfo leaf accumulate and survive into main.
There is no competing OnceBox loser, replacement tree or growing registered
buffer on that construction. Other traced entry activity is
scalar/static/stack or OS storage outside the metric. Therefore the existing
source result supports

    EntryPeak <= M_stack + L_info + 4
    EntryGrowingOld = 0

for those constructors. The corrected proposal combines those retained entry
identities with argv and later adds the separate stdout buffer/mutex once.

It accurately keeps final VR entry/std/cfg/allocator/artifact applicability
open. Historical 64/544 request facts remain qualified evidence, not unknown
constants to re-probe or automatically adopted final-VR values. Failed startup,
foreign initialization, panic/unwind and additional reached lazy/error paths
retain their explicit qualifications or separate required bound. H's
initialized raw-byte dropped-error writer result is not widened to the wrapped
serde writer/error path. No completed-H-only scope has silently become a
universal global-prefix claim.

## Unchanged residuals

The correction preserves the added pair-set binding, roster-limited empty
spring-set premise, HHex, JSON/stream/format/IO/error/panic owners, stable
input/actual launch correspondence, final checked implementation, source/build
transfer, historical replay and required measurements. Concurrent node/format
proposals are not accepted by this backcheck.

The original successful fixed-input parser/read/typed-construction conclusions
remain conditional and unchanged. No input, source algorithm, private request
or arbitrary failure-path question was reopened. Later final integration must
use the corrected formula rather than packet08's superseded HCounts equation.

## Evidence and execution

One bounded evidence command returned exit 0 and **80 passing mechanical
checks**, including the three full seals, three immutable source blobs,
formula identities and the stated integer example. Those checks support this
independent lifetime reasoning; text presence alone is not a proof.

The command was:

    <VENV>/bin/python -B <OUT>/_run_records/check.py <K6C> <NUM> <OUT>

Raw stdout/stderr, Git argv/status/source hashes and extracts are in
_run_records. Git reads set GIT_OPTIONAL_LOCKS=0.
The brief is immutable NUM
`38360301a1a2074d0c206f1d02f9ed016d96a7d6`,
R/BRIEFS/K6C_PREFIX_CORRECTION_09.md.

No Rust/build/probe/parser/model/solver/runtime execution, new library
research, maintained edit, Git/index change, allocator/guard change or
delegation occurred. Only this additive review subtree was written.
Prior sealed bytes remain unchanged. TIMING.json records completion within
the granted ten-minute leg.

