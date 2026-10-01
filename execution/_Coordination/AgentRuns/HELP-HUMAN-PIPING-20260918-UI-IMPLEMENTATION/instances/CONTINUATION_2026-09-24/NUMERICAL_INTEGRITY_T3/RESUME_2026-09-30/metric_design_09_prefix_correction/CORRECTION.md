# RV28-PREFIX-1 — additive counts union correction

**Correction supplied for the same RV28 backcheck; the finding is not declared
independently closed by its author.** This addendum supersedes only packet08's
HCounts decomposition and narrows its runtime residual #2 using the review's
existing successful-entry warrant. metric_design_08_vr_prefix remains sealed
and unchanged. No maintained source, metric, domain or admission rule changes.

## Fresh free-DOF and layout owners

Keep packet08/K0's operators: R is retained requested bytes; epsilon=0 for
ordinary requested peak and1 for the existing growing-realloc move peak.
G(T,k) is the reviewed fresh filtered/push capacity upper; O_G(T,k) is its
single active growing-old buffer. Define:

```
RFree   = G(usize,f)
HFree   = RFree + epsilon*O_G(usize,f)
RLayout = G(QuantityMeta,q)
HLayout = RLayout + epsilon*O_G(QuantityMeta,q)

HCounts_corrected = max(
    HAdj,
    RAdj + HProfileCount,
    HFree,
    RFree + HPatternCount,
    RFree + HLayout,
    RFree + RLayout + HEncoding(X)
)
```

HEncoding(X) retains packet08's source-encoding construction contract, including
its own active-old term where applicable. SRC0, k4src and the already-retained
caller owners remain **outside** HCounts as before, charged once. Do not sum
these sequential alternatives or add multiple simultaneous realloc-old charges.
The separate HLayout arm now explicitly covers layout growth while RFree lives;
RLayout alone is sufficient only for the subsequent encoding phase.

The immutable40129 lifetime warrant is precise:

1. VR/src/scale.rs136 finishes profile(&free_adjacency(model)) as a separate
   statement. Its adjacency/profile temporaries do not overlap the later free
   receiver merely because both feed Counts.
2. scale.rs145 calls source.free_dofs().len() inside the Counts tail expression.
   FK/src/structural/retained/source.rs629-633 returns a new Vec<usize> from a
   range/filter/collect. This is neither a borrowed cached vector nor a scalar
   accessor; SRC0 does not own the returned buffer.
3. The fresh filtered Vec's construction is HFree. The resulting RFree remains
   through pattern_entries at147, layout at151 and encoding at152. The reviewed
   temporary-scope rule gives no struct-field drop boundary; narrowing the tail
   scope still ends after the whole Counts expression.
4. Layout's receiver remains through the encoding call. All three caller-created
   receivers drop before counts returns, and therefore before V399/400's cut.

This is a reached-owner correction, whether or not another branch dominates the
final maximum. On the named64-bit capacity basis f=60000 gives
RFree=8*65536=524288 bytes. That component amount is **not** an asserted change
in the complete E_max, nor evidence of measured overflow. No numerical final-
total undercount or actual runtime failure was demonstrated by the review.
A later simplification requires an explicit dominance proof; none is invented
or needed for this direct decomposition.

## Cut survivors stay unchanged

Retain packet08's exact cut identity union:

```
C_tV = RuntimeRet_after_start + ArgsRet + separate id clone
       + selected Case children + standalone Model
       + optional external model hash + k4src hash
```

Do not add RFree, RLayout or the second encoding buffer to C_tV. They contribute
to earlier PrefixPeak even though they are dead at the cut. Preserve
`max(PrefixPeak, max_later(surviving cut identities + later owner union))`
for both counters, with the existing prelaunch admission protocol unchanged.

## Narrowed successful direct-entry premise

RV28's sealed review applied the already supplied source12 H-R0 roster and
subsequent h_leaves_09/h_request_bindings_10 facade closure. Under the stated
successful direct single-main, no-foreign-initializer, non-unwinding path, the
registered entry owners are at most one name Box<=4 bytes, one registry mutex
child and one first ThreadInfo leaf. They accumulate and survive into main;
that path has no replacing tree, competing OnceBox loser or growing registered
buffer. The other traced entry work is scalar/static/stack or OS memory outside
this requested-byte metric. Consequently use the reviewed source upper:

```
EntryPeak <= M_stack + L_info + 4       (beta=1 conservatively)
EntryGrowingOld = 0                    (these constructors only)
```

This is a bound on the specified pre-args phase as well as its survivors. Combine
EntryPeak and argv construction by allocation identity, retaining those same
entry children while argv is built; do not add a second free startup allowance.
The separate stdout buffer/mutex joins them on first stdout initialization under
its reviewed source premises. It does not duplicate the registry mutex identity.
No observed612 value or process baseline is used.

Packet08's runtime residual #2 is therefore narrowed: **no missing successful
direct-entry temporary edge is identified by the reviewed source basis**.
Do not commission a generic runtime audit for that already supplied warrant.
The historical64/544 request-site facts are not unknown mathematical constants
again, but their applicability to final VR remains a source/build-transfer gate.

Still required: actual final VR entry/std/cfg/allocator/artifact correspondence
and preservation of the same direct single-main path. Failed startup, foreign
initializers, panic/unwind behavior and additional reached lazy/error paths
retain their explicit qualifications and must have a separately supplied bound
or explicit registered scope. A completed-H assumption is not promoted to all
prelaunch failures. The initialized raw-byte H writer result does not close the
wrapped serde writer/error route; that remains with its separate owner.

## Remaining status

The added pair-set node request, untouched-spring-set reach condition, HHex and
JSON/stream/format/IO/error/panic terms, stable input/actual launch correspondence,
final checked integration, historical replay and required measurements stay as
packet08/K0 and their owning later returns specify. No concurrently assigned
node/format proposal is accepted by this addendum. No new source/library/runtime
proof was attempted, no alternative contract is selected, and no complete
VR-PREFIX/K0/E_max/admission acceptance follows. RV28 must backcheck this exact
corrected union and entry-premise clarification before dependent reuse.
