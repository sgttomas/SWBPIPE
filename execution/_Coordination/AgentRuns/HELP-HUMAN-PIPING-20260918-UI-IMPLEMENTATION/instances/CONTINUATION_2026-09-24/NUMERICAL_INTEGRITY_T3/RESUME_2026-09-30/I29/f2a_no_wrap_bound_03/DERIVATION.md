# No-wrap arithmetic components — proposed, not a whole-run proof

Source: main 49034a940f3f8cd3f3da4d4cbc839943b808063d.
FK below denotes core/solver/frame_kernel/src/structural/retained under
projects/chirality-piping. All bounds are mathematical nonnegative integers,
evaluated with checked arithmetic. M64=2^64−1; require strict bound<M64,
excluding both wrap and exact-MAX/saturation ambiguity. Neither work thresholds,
the memory target nor returned-counter equality appears as a proof premise.

## Typed conceptual count interface

Bind to the actual immutable PrimitiveSource, CasePrep and prepared group:
N=6n global DOFs; F free DOFs; Z pattern entries (both triangles);
H=sum_i(i−first[i]+1); E=H−F; W=max_i(i−first[i]); B free blocks.
Require H≥F, W≤max(F−1,0), B≤F and actual block partition size sum=F.
Also carry member/spring/station/layout/body counts, load terms and ledger input
limb bounds, total prescribed term pairs, support-group counts G and contributor
upper C, and ordered combination operand multiplicities. K bounds all actual
case/combination runs and separately authorized retries; cache origins carry
their own proven source/count bound. No wire/native field shape is assumed.

Before exact counts exist, protect raw count construction with checked6n,
contribution/pattern uppers, F(F+1)/2 profile upper, encoding/index conversions,
and scalar products. Existing order_free computes a usize profile sum; its
returned value alone is not proof that preprocessing did not overflow.
Raw lengths and bounds must precede that calculation. The named proof remains
independent of unfinished geometry/source mapping, but those facts must be bound
by the admission owner before relying on it.

## A. Atomic cumulative-counter bounds

wide_sum.rs:40–103,188–210,216–545: clear and reset keep work; clones copy it.
Each owner therefore needs its creation/ancestor bound plus every increment over
its complete lifetime, not a new zero allowance after each clear.

Premise: used≤128 and the existing span/carry invariant. A live magnitude has
fewer than 2^64 nonzero raw additions between value resets; prospective operation
bounds, not returned counters, must establish that carry-headroom condition.
The 8128-bit span check permits at most 127 significant term limbs. Allowing at
most128 carry steps gives a deliberately loose256 term increment; shifting both
magnitudes moves at most 256 limbs.

| Call, including a failed prefix | term≤ | shift≤ | net≤ | rounded≤ | Wide charge |
|---|---:|---:|---:|---:|---|
| raw / wide / binary64 / integer add |256|256|0|0|none|
| wide_scaled, L≤16 |L+256|256|0|0|none|
| Wide product add |512|512|0|0|one TwoProduct|
| scaled accumulator add |768|512|0|0|none|
| accumulator product add |98,304|65,536|256 on mutable operand b|0|none|
| signum/is_zero/make_absolute |0|0|128|0|none|
| round |0|0|256|128|one Round|

Scaled-accumulator addition has two passes, each at most 128 scaling increments
plus one raw add. Accumulator-product addition nets b once then makes at most 128
scaled additions; the destination does not own b's netting increment.
Errors shorten this nonnegative event prefix. Caller counts, clone ancestry and
full merge multiplicity remain required; this table alone is insufficient.

wide/multi.rs:984–997,1012–1111,1153–1226:
A_L=2L (add/sub/round), P_L=L², D_L=(L+1)(64L+2),
S_L=(L+2)(64L+2), T_L=L²+4L (TwoProduct), TwoSum=12L.
L is4/8/16, so max primitive price is S_16=18,468.
A context increments before returning success/error. max_span_bits is max-only
and bounded by the accepted8128-bit span. SumWork LME is the sum of
its four component bounds. WidthWork/AttemptWork weighted sums and merges require
their own upper despite native saturation. Conversion/widening/mul_pow2 get no
new price from this analysis. Local unmetered accumulators still need safe counters.

## B. Factor and failure scan

factor.rs:447–496,504–601. Let
Q=Σ_i Σ_(j=first[i]..i−1) max(0,j−max(first[i],first[j])).
Q≤floor(E·max(W−1,0)/2).
The loop has at most Q multiply/subtract pairs, E divisions, E pivot
multiply/subtract/add triples, and F pivot predicates (three scaled adds+sign).
A failed pivot invokes negative_pair at most once before return: at most Z
tested pairs, each six scaled adds+sign.

J_L=3(L+256)+3·256+128; V_L=6(L+256)+6·256+128.
A sufficient local increment upper is:
F_fac=Q(P_L+A_L)+E(D_L+P_L+2A_L)+F·J_L+Z·V_L.
It conservatively includes mutually exclusive complete-success and failure work.
An unguarded row plus its possible failure scan is bounded by:
U_row=floor(W(W−1)/2)(P_L+A_L)+W(D_L+P_L+2A_L)+J_L+Z·V_L.
The incoming context/SumWork bound must be added. The ordinary 64·operations
multiplier also requires128F<M64 because operations≤2F.

## C. Unguarded condition screen and post-check pivot tracker

factor.rs:695–809; bound.rs:185–214; adaptive.rs:1363–1409.
There are at most five condition iterations and one alternating solve:
11 solve_scaled calls. One solve_scaled costs
Tsolve=2E(P_L+A_L)+F·D_L (:633–655).
Norm uses at most Z scaled adds; six absolute sums use6F plain adds;
five dots use5F TwoProducts/two-term expansions. Eleven observer calls use at
most22F plain adds,22B rounds and11B divisions.
For F>0 let R=F+11+22B. Component increment bounds:
term≤(L+256)Z+256·38F; shift≤256Z+256·38F;
net≤256R; rounded≤128R.
Wide LME≤11Tsolve+A_L·R+5F·T_L+(11B+F+3)D_L+P_L.
C_cond is their sum. F=0 returns without counted work. This bounds the whole
unguarded screen, including the observer and cumulative clears.

The following pivot-margin loop has≤F offers. Every offered row incurs at most
one width4 approximate ratio and one width16 exact evaluation; collapsed rows
are not reevaluated by finish (adaptive.rs:688–791).
C_margin≤F[(2A_4+D_4)+(2A_16+D_16)]=18,812F charged LME.
Tracker.offered+=1 additionally requires its lifetime offer count<M64.

Legacy directed_ratio correction comparisons create fresh scratch accumulators
(:470–550); their local counters do not accumulate into the charged context.
Each scratch is atom-bounded, but this is not a wall-time bound. Cloned input
counters need ancestor bound plus sign/round increments. A1 round_up is different:
CertificateMeter collects comparison scratch repeatedly. Its cumulative loop
bound remains a named residual; no presumed “one correction” is used here.

## D. Unguarded support tails and scalar hazards

Let G=support groups and
C=Σ_groups(true restrained components + axis child count +3·directional child count),
an upper on actual added component contributions after source validation.

recover.rs:438–475 runs after its last guard. It has C plain adds,6G component
rounds, and2G magnitudes; each magnitude has3 products+round+sqrt (:221–234).
Thus its SumWork increment≤512C+9,216G and Wide increment
≤8G·A_L+6G·T_L+2G·S_L. Incoming histories still apply.

verify.rs:244–283 formation_scale support tail has C plain adds and2G stage
roundings. Nearest stage costs≤384 SumWork+A_L. For upward stage,
directed.rs:36–90 gives at most a nearest round, add/sign and one exact step:
SumWork≤2,432+L and Wide≤2A_L. The conservative both-mode tail upper is
512C+2G(2,432+L)+4G·A_L. Counts alone do not close the preceding formation_scale.

residual_rows/bounded_fallback (adaptive.rs:1486–1500,1643–1653) use count+=1,
m=2count+2 and64m. A sufficient scalar guard is128(Z+1)<M64 where row count≤Z.
Also protect all usize/index expressions before conversion.
bound.rs:946–955 uses ordinary r*r in ceil_sqrt. A conservative sufficient guard
is F≤2^32−2 with each n_c≤F: its exact integer input is representable in binary64,
initial integer r≤n_c for n_c≥1, and every increment stays≤n_c+1≤F+1, whose
square is below2^64. This is a sufficient count restriction, not an observed failure.
F=0 gives no nonempty block. A broader proof is possible but not needed here.

## E. Required global composition (NOT instantiated)

Each raw owner needs an independently derived lifetime event bound, including
clones. Every aggregate/weighted/stage/record transfer then propagates parent
bounds with actual multiplicity; subtraction also needs monotone chronological
snapshots. Define complete bounds S_p/O_p for shared/own solves, V_p for
verification shared+own, D_p for decision and A_p for A1 certification.
The schedule has at most four physical solves128/256/512/1024, three verification
passes and three decisions/certificates (adaptive.rs:3937–4165). Reused verification
is the same physical record, not a second solve.

B_run=Σ_four_p(S_p+O_p)+Σ_256,512,1024 V_p+Σ_128,256,512(D_p+A_p)
is a conservative candidate upper only after those functions/internal transfers
are closed. Failed/stopped paths are prefixes of proven nonnegative bounds.
The present factor/condition/margin bounds are components of S_p, NOT S_p itself.

For cached success/non-budget failure, retain the originating build's bound.
Each case pays full reused shared work; the invocation pays actual new builds.
A conservative invocation sum may sum complete standalone run bounds for all K
runs, carrying cache-origin bounds and forgoing reuse credit. Budget failures
are not cached; new independent calls/combination slots are counted again.
Every raw/component/weighted/stage/case/invocation upper must be<M64 and every
scalar/index expression representable. Exact-JSON projection stays a separate
later check. The missing complete functions/transfer audit prevent using this
composition as production admission today.
