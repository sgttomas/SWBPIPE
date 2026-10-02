# I21 source_03 — frozen A1 owner delta

**Provisional source accounting only.** No E_max, admission, compiled-layout or
measurement acceptance. This compact delta applies to A1
`3cf296e36645d97e4c657c8ad1a6322bc4163f16`; adaptive.rs SHA256
`4e5618a8d809e6ffa4ddd73024c206cc45eac6d5404a2f2034674552e1a95db9`.
All product references below are frozen Git blobs. A=K/adaptive.rs,
K=P/core/solver/frame_kernel/src/structural/retained. H/VR and allocation
operators are as in source_01/source_02. Later I22 repairs must be reconciled
against this exact source, even when described as compiler-only.

Let q=validated publication rows, B=bodies; E(k,T)=k*actual sizeof(T).
G(k,T)=P(k,T)*sizeof(T), with source_02's nonempty P=max(mu,next_pow2(k)),
and MG<=1.5G for fresh pushes/lower-zero collect. A(4B,S) and MA are the
source_02 conservative generic-collection retained/move envelopes for the
body-scale tuple S=(u32,Kind,u64). No private size is inferred.

“Reuse old term” below means the corresponding named source_01 owner can be
carried forward with its **new phase/lifetime/capacity**. It does not mean
unchanged H/counts.rs already proves coverage. Its coarse end term at :525-528
still uses q*(sizeof(PublishedRow)+40), not the complete owner ledger.

## Owner, lifetime and phase maximum

| Owner / source | Heap, lifetime and maximum | Old term versus explicit delta |
|---|---|---|
| Canonical layout, A:3119-3121; K/recover.rs:101-198 | Fresh push-only Vec<QuantityMeta>, G(q,Meta) retained / MG(q,Meta) construction. The existing prep.layout stays live. Temporary ends at comparison condition; no draft exists yet. Called at A:4027 before R7 and again at :3314 only when R7 accepts. | **New phase term** above kept+report, and above kept+report+decision for second call. Existing CasePrep layout covers only the original owner. Two sequential calls are not two simultaneous temporaries. Initial CasePrep layout at :957 is unchanged. |
| BoundVerification, A:2359-2365,3892-3898 | Stack wrapper containing prep Arc, PrecisionState Arc variant and report Arc. prep/state clones are shallow; the report Arc allocation is the existing verify-result allocation. No Arc::new for wrapper/prep/state. | Existing prep/state/report heap owners absorb payload; **no duplicate Arc payload term**. Report lifetime extends through canonical checks, R7, certificate and selected finalization, then ends. It is not retained in RetainedSolve. |
| Provisional values, A:2326-2331,3323-3327 | Borrowed exact-size map produces E(q,Binary64Outcome). publish_prescribed mutates this buffer using an inline ExactAccumulator. Explicit drop at :3342, before certify_rows/H/radius allocation. | Reuse old K20 publication-values owner, **move it to every reached certificate attempt**. Remove it from the later accepted finish owner union; do not silently trade its former slack for radii. |
| Raw/coupled scales, A:2680-2702 | Two separate E(B,[f64;4]) buffers, alive through row and body-scale construction. Both end when publication_with returns. | Reuse old K20 classification scale owners in the earlier draft phase. They do **not** overlap radius/H phase. |
| Publication rows, A:2703-2722 | Result<Vec<PublishedRow>> from borrowed zip/map uses GenericShunt lower0: **G(q,PublishedRow)** retained, MG(q,PublishedRow) growth. A classification failure drops the successful prefix and all local scales. PublishedRow fields themselves have no child heap. | **Capacity correction**, replacing exact q-row storage and construction with P(q)/move model. It affects the shared public classification helper too, even with its infallible closure at :2667. |
| Publication body scales, A:2723-2732 | 4B tuple output, finite flat-map collection. Use A(4B,S) retained and MA(4B,S) during construction unless a tighter exact iterator capacity is bound. Rows and provisional values/scales remain during this construction. | Reuse old body-scales owner; do not count a second array merely because Publication moves to CertifiedPublication. Exact-length deep clone in evidence remains a separate old K20 term. |
| Radius storage/conversion, A:3223 | vec![sentinel;q] requests **E(q,u64)=8q** on the pinned implementation. Source_02 Vec exact-request contract gives cap=len; into_boxed_slice therefore keeps this allocation (vec/mod.rs:1733-1739). Box slice pointer/length is inline in its owner; no extra heap header. Allocated for all q rows before the loop, including absent-radius rows. | **Explicit new radius owner**, construction/certification/finish/selected result. This exact array request is only one owner, **not a universal certificate or process bound**. No radius growth per row; no two full arrays for this conversion. |
| H, comparison, RU scratch, A:2819-2917,2942-3067,3270-3275 | h, difference, numerator/denominator clones, reaches scratch and arithmetic intermediates are inline arrays/scalars. ExactWideSum has two [u64;128] magnitudes (wide_sum.rs:108-124); Wide arithmetic uses fixed stack arrays. The dyn FnMut is borrowed, not boxed. | **Zero new heap term** for these arithmetic locals. At most five named high-level ExactWideSum objects coexist in RU (caller num/h,n,den,d,reaches scratch), plus callee fixed arrays. This is not a compiled stack-frame bound. Never add a tracker-row heap budget for each H comparison. |
| Meter / cloned-work counters, A:2755-2817,3371-3377,4082-4088 | CertificateMeter, SumWork, before/delta counters and AttemptWork are inline fixed fields/arrays. clone_delta changes charged counters, not allocation behavior. Static policy/sentinel/Kind::ALL data is not a heap String/Vec. | Existing work fields in AttemptRecord are updated; **no growing work-log container** is introduced. New enum alternatives can still affect actual containing type strides; witness dependencies below. |
| Rejected or partial certificate, A:3219-3296,3313-3370,4089-4135 | On predicate rejection, Publication and radii are dropped inside certify_rows before Rejected(index,predicate) returns. Err likewise unwinds partial rows/scales/values/radii. Budget failure at meter.checked after constructing Accepted also drops the complete temporary. Only scalar rejection/error fields escape. | Retain construction peak in new certificate phase, but **do not retain failed drafts/radii into the next precision**. Three candidate certificates are sequential, not three accumulated boxes. Earlier states/shared caches and attempt records still remain under existing terms. |
| Selected finish, A:4184-4306 | Certified publication/radius are moved at :4208-4209. No re-publication/reclassification. Existing report summary, selected-record deep clone, evidence encodings/filter outputs and body/decision clones coexist with publication+radius. Box<RetainedSolve> at :4297 contains a new slice-pointer field; use actual new sizeof. Certified prep Arc and caller BoundVerification add refs, not payload copies. | Reuse old K20 finish/evidence terms **without provisional values**, replace publication row capacity, add radius and updated Box layout. Existing report owner persists through Box creation, then drops before the selected result returns to consumer. Early finish rejection :4197/:4210/:4214 drops publication/radius and leaves none in terminal outcome. |
| RetainedSolve / selected outcome Clone, A:3422-3435,3554-3566 | Derived Clone deep-copies publication rows/body scales, evidence and states-vector headers; states/prep/group/cache success Arcs share payload. Private Box<[u64]> deep-clones. Vec clones request length q, **not original P(q)**. CaseOutcome::Selected Box clone allocates outer Box<RetainedSolve> before cloning fields; cloning a bare RetainedSolve value alone has no such outer Box. | Existing clone child terms remain; **add one new q-u64 array per distinct deep clone** and rebind outer Box layout. Original P(q) rows + clone E(q,PublishedRow) overlap. Original and cloned radius allocations overlap. No verification-report clone/retention is added. Generic cached error-refusal Vec clones, if reachable, keep their existing separate terms. |
| H and VR consumer lifetime | H/main.rs:863-873 retains selected result through output/digest then drops it; saved attempts clone only attempts, not radius/publication. VR lane's selected outcome remains through comparison and case_record, then drops before CaseRun returns. | Add radius/row-capacity delta to the existing selected-outcome owner during H completion and VR lane comparison/record construction. **No radius accumulation across H repeats**, and no radius in post-lane JSON clones or sparse tail merely because a record persists. This does not change staged-H versus global-VR metrics. |
| Combinations, K/combine.rs:80-132; A:922-947,4413-4441 | Operands are borrowed selected solves: their existing radii remain live, but are not cloned/read to form the combination. Identity buffer and preps reference Vec survive through run_schedule. The inner sources reference Vec ends after CasePrep::combination. Combined prep owns new source/ledger/prescribed-term/factor/identity data; group/cache success Arcs share. New combination runs the same certificate and owns its own radius on selection. Outcome wrapping moves the same selected Box. | Keep input selected-owner union (including existing radii) as caller context; add the same new certificate/result terms for the combination, once. Shared Arc payloads counted once; no a-fold factor copy. Preexisting combination buffers are **not** covered by a single-case fixed term without explicit composition. No new combination admission theorem is asserted. |

## The compact phase composition

For a given candidate pair, let Kp be the existing caller/source/group/cached
shared/states/attempt-owner union; Rp the current verification report owner;
Dp the returned R7 decision's summaries/floor (not its dead tracker scratch).
Define:

- L=MG(q,QuantityMeta)
- V=E(q,Binary64Outcome), S=E(B,[f64;4])
- Pr=G(q,PublishedRow), Pg=MG(q,PublishedRow)
- Br=A(4B,BodyScaleRow), Bg=MA(4B,BodyScaleRow)
- U=E(q,u64)=8q, the specific pinned radius allocation only.

New or relocated alternatives above existing owners are:

```text
before_R7_shape = Kp + Rp + L
certificate_shape = Kp + Rp + Dp + L
draft = Kp + Rp + Dp + V + 2S + max(Pg, Pr + Bg)
rows_H_RU = Kp + Rp + Dp + Pr + Br + U
accepted_finish = existing finish union, excluding old V,
                  using Pr instead of q*sizeof(PublishedRow),
                  plus U and actual new Box<RetainedSolve> layout
```

Take maxima at **each reached precision**, including eventual certificate
rejection and terminal budget/arithmetic error. R7 tracker scratch is dead
before certificate; Dp persists. The two canonical layouts do not overlap
each other or V/Pr/U. Draft values/scales do not overlap U/H.
These are source owner equations, not a completed numeric E_max; existing Kp,
Rp, Dp and finish composition still require their outstanding bindings/review.

Publication capacity can be material independently of U. Frozen H counts hash
remains fab0466a846ef26b1c530a261a6f1d134fc32ba1d70f57682010877a52373c05.
For either orientation at 10,000 members, q>8 makes P(q)=next_pow2(q):

| Family | q | New row capacity | Extra retained row slots vs q | Radius payload request only |
|---|---:|---:|---:|---:|
| CHAIN/TREE | 250,013 | 262,144 | 12,131 | 2,000,104 B |
| CONT | 265,013 | 524,288 | 259,275 | 2,120,104 B |

These are pure integer/source calculations from committed counts, not sampled
capacities or total-byte bounds. Actual PublishedRow stride is deliberately
not multiplied in. Existing lower-size and VR cases retain their real q/B;
nothing is silently zeroed or excluded.

## Clone and combination details that prevent double counting

The supplementary installed boxed.rs page identifies the same Rust1.97.1
commit as source_02 and decodes to the existing historical hash
31fe2ec856527253e2a78279f44b7dec14b73d9f1114a75584b031dcd6f64418.
At :2075-2078 Box<[T]>::clone uses to_vec_in then into_boxed_slice: for u64
the fresh exact q-element allocation becomes the new slice allocation.
At :2041-2046 Box<T>::clone preallocates its outer box before field cloning.
Original and copy are separate; Box conversion does not create a third q array.
The page origin/hash/transformation are in BOX_SOURCE_BINDING.json.

GroupCache::merged clones successful Arc slots; no operand publication or
radius enters that cache. For n operand references, input radius ownership is
the union of distinct selected allocations (sum_i8q_i is a safe overcount if
aliases occur). Accepted combination adds its own8q. The combination's
preexisting identity/preps/new CasePrep buffers must stay in the caller/base
union; they cannot be replaced by the radius sum. Rejected combination
certificates release new U while all borrowed operands remain unchanged.

## Exact witness dependencies and residual ambiguity

1. **Changed aggregate layouts:** actual RetainedSolve (new slice-pointer
   field); AttemptStop; AttemptReason/AttemptOutcome/AttemptRecord;
   UnresolvedReason/CaseOutcome; Evaluated and its (u64,Evaluated) table entry;
   affected cached Slot/VerifySlot/GroupCache/GroupEntry aggregates. New enum
   alternatives may change strides even if no new child heap appears.
   No unchanged-size assumption is made.
2. **Existing stride/operator facts still needed:** actual QuantityMeta,
   PublishedRow, Binary64Outcome, BodyScaleRow, precision-state vector entries,
   summaries, old evidence/Box/Arc ownership facts. The new stack-only
   CertificateMeter/BoundVerification/CertifiedPublication do **not** need
   independent heap-Box sizing unless a later implementation boxes them.
3. **No guessed stack bound:** ExactWideSum/ExactAccumulator are inline arrays;
   no heap allocation is introduced by their Clone. Exact stack-frame bytes,
   optimizer effects and nested arithmetic scratch remain unmeasured and are
   outside requested-heap E_max. Do not recycle the old4304-byte tracker heap
   element size as a new per-row H allocation.
4. **Final source reconciliation:** this blob has not been compiled/tested by
   I21. Any repair changing expression form, scope, collection kind, wrapper
   allocation, clone behavior or error variants reopens the affected row.
   Final A1 candidate, full old owner union and bound review remain pending.
5. **Consumer consequences:** existing finite error-format/serde/argv/runtime
   envelopes must include the new certificate/error/rejection grammar and
   changed AttemptRecord-related layouts. No new radius field is present in
   public RetainedEvidence, so do not invent its serialization. Generic Debug
   of the entire RetainedSolve could visit it; the existing H repeat digest
   uses publication/evidence and does not do that.
6. **Domain:** source_01's finite H/VR scope remains. Combination and selected
   Clone lifetimes are reported because this brief asks for them; arbitrary
   numbers of live operands/clones are not admitted under the single-case
   estimate. Bind those counts and aliasing explicitly before any such bound.

All equations are A1-sensitive and provisional. No input ceiling, tolerance,
protected claim, admission rule or experiment was changed.
