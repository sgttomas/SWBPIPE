# RV44 Stage 0 — independent source derivation

Frozen before reading I29/f2a_no_wrap_bound_03. Source main `49034a940f3f8cd3f3da4d4cbc839943b808063d`; inspected NUM HEAD `9f9abd2d78ea68c4d9cf5edbf36ec8529803018b` has no frame_kernel source difference. This is a source/count derivation and proof-obligation inventory, not a complete no-wrap theorem or a workload-overflow allegation. Source anchors below are relative to frame_kernel/src/structural/retained.

Receipt 2026-10-02 20:25:33 UTC; new-analysis cutoff 21:15:33; total deadline 21:25:33. Native harness child `/root/rv44_f2a_no_wrap`, delegated by ROOT HELP_HUMAN as TASK; no descendants. The brief restricts writes to this review directory and prohibits maintained/Git/index/API writes, numerical execution and host probes. Only source reads and standard-library exact integer/hash work have been used.

## Counter coverage

- `wide_sum.rs:77–105,188–210,291–294,323,367,443,467,511,542`: four cumulative u64 components; clear AND refusal reset retain work. Ordinary `+=` can wrap before later saturating LME/merge. max_span is a maximum, not a charge. Each reused accumulator needs a lifetime bound; a per-current-value term bound is insufficient. Clones inherit work, so clone delta subtraction at `adaptive.rs:2756–2762` also needs monotone no-wrap history.
- `wide/multi.rs:986–1110`: operation counts and merges saturate; prices are finite only with supported widths 4,8,16. Maximum single context price is sqrt at L16: 18×1026=18,468; a future bound must prevent saturation if it promises exact historical totals. `AttemptWork::record` counts a whole context; merged operation counts require the lifetime bound too.
- `adaptive.rs:1186–1205,1296–1400,1428,1724–1730,1845–1849,1877,2082,2288,4055–4056` and `verify.rs:443–444,522,743–747,1216`: ordinary aggregate/stage addition and monotonic differences require bounds on intermediate expressions, including stopped/error returns. Stage totals and budget/meter saturation cannot repair earlier wrap.
- Guard check uses saturating base+context+sum and `used > room` (`adaptive.rs:259–280`). Every pre-guard segment must independently fit; 20B/60B are stop thresholds, not upstream integer bounds. Work after the final check and errors that return before a check must still fit.

## Count provenance and conservative structural relations

Let N=raw source node count, D=6N, M=members, S=axis springs, V=directional springs, T=stations, C=constraints, H=support groups, J=total support membership lengths, F=free DOFs, A=full sparse pattern entries, E=profile entries including diagonal, B=free blocks, R=publication rows, and K=authored combination operand count. These are exact mathematical integers, not results of unchecked native expressions.

After a safely completed valid source/structure/order preparation: F≤D, B≤F, A≤D², E≤F(F+1)/2; tagged contributions =78M+S+6V; R=7N+12M+6T+S+3V+C+2H. Member distinct-node validation makes its 12 DOFs distinct. Support membership deduplication can reduce J but does not bound the number of groups; the same spring can appear in many groups. For pre-preparation admission use raw membership/operand/load/identifier lengths as safe upper counts and account for combination K×constraint and K×load expansions.

Trust timing matters. `PrimitiveSource::new` multiplies node_count×6 before constraints are validated and builds u32 body indices (`source.rs:352–590`); source encodings cast lengths to u32 (`source.rs:655–738`). Structure counting-sort offsets and `profile_entries` are ordinary usize sums (`assemble.rs:627–640`, `factor.rs:393`). An admission proof cannot rely on these derived native totals before proving their own formation safe. A future preflight must bind raw decoded counts, checked conversions/length products/encoding lengths and the selected pointer-width/build profile before affected construction. Dense algebraic bounds can be evaluated independently beforehand; actual narrower A/E only become trusted afterward with the earlier construction proof.

## Local finite arithmetic independently derived

For a valid accumulator with used≤128 and a globally established <2^64 raw-term carry premise:

| Primitive | Conservative charged work bound |
|---|---:|
| add_raw, including possible carry and anchor shift | 256 term +256 shift =512 |
| add_wide_scaled at width≤16 | 528 |
| add_scaled, both signs | 2×(128+512)=1,280 |
| signum / net / round SumWork | 128 /256 /384 |
| add_product | 2×512+320=1,344, including TwoProduct at L16 |
| add_product_of | 128×1,280+256=164,096 across destination and netted operand |
| round, including context Round | 416 |

These intentionally loose bounds include partial work on refusal. A coarse atom price 2^18 exceeds every listed accumulator primitive and every supported context operation. The table is conditional on valid span/carry and types, not by itself a cumulative theorem. A global atom cap Q with 2^18 Q<u64::MAX also gives at most 256Q<2^54 raw terms in any accumulator lifetime, sufficient for the stated 64-bit carry headroom; ordinary additions still require charging all affected atoms/lifetimes and copied work correctly.

One independently counted unguarded example is `factor::condition_observed` (`factor.rs:695–815`, `bound.rs:184–209`). With F>0 it performs at most eleven triangular solves (two per each of five iterations, then one safeguard); each solve uses exactly 4(E−F)+F rounded context operations. The norm visits at most A entries; each observer visits the partition of F free positions twice and at most B blocks; six absolute sums and five dot sums suffice. Counting one public accumulator operation or one direct context call as an atom, a conservative whole-condition atom bound is

`Q_condition ≤ 44E + 46F + 33B + A + 15`.

Derivation: solves≤11(4E+F); norm≤A+F; observers≤11(2F+3B); abs sums≤6(F+1); dot sums≤5(F+1); alternating vector≤F divisions; four final/initial context calls. Hence incremental condition LME≤2^18 Q_condition. This is only one segment; it excludes later pivot-margin trackers and inherited earlier work. It also presupposes that `3*n`, `n−1+i`, profile indexing and count casts fit. No claim of practical availability follows from this deliberately loose bound.

## Required lifetime and path proof

1. Shared formation uses one cumulative sum through p/q member formation, unguarded directional formation, assembly, factor and condition. Assembly checks on diagonal entries only (`assemble.rs:686–747`), factor per completed row and may enter an unguarded negative-pair error scan (`factor.rs:580–594`). Condition is wholly unguarded until `adaptive.rs:1380`; its subsequent pivot-margin tracker loop is after that guard (`1383–1395`). Include that tail.
2. Own solve includes all prescribed terms, reduced RHS and a triangular solve before the first check (`adaptive.rs:1734–1750`); at most three correction solves/four residual passes (`1760–1835`) and bounded fallback over all evaluated states. Recovery checks members and reactions but then has an unguarded support-group tail (`recover.rs:438–475`), including repeated group memberships and two magnitudes per group. Count all stations/springs/directional terms as well as members.
3. Verification shared formation/assembly, Uc passes and error-local reset paths are cumulative (`verify.rs:420–533`, `bound.rs`). Verification own work includes formation scale (another unguarded support tail, `verify.rs:244–283`), prescribed term arrays, contribution products, block data, at most three shifted factorizations, bounds, body/row loops, and stopped-stage closure. Refusal does not erase incurred work.
4. Decision trackers preserve lifetime across offers/collapses. Bound source row offers and evaluations, not merely resident T=512/G=4096 capacity. Directed ratio correction loops (`adaptive.rs:491–550`) use fresh local scratch accumulators per iteration; their iteration count affects finite total cost, but those scratch counters are not merged into legacy LME. Any primitive-count bound must distinguish that fact. A1 CertificateMeter DOES collect comparison scratch and checks successful operations (`2756–2889`); arithmetic-refusal precedence still charges partial work. Its cloned-counter deltas require a bound including inherited counters before subtraction.
5. Schedule has at most four physical solves (128,256,512,1024), at most three verification passes, three decisions and three certificates (`adaptive.rs:3937–4165`), with verification state reused as later candidate. Candidate and failed-verification escalation differ. Bind the future admission to these exact precisions, widths, ordinary production feature set (no mutation-controls/test hooks), call schedule and actual build identity.
6. Shared and verification-shared caches can store success or non-budget failure; budget failures are not cached (`3603–3637`, `3783–3829`). Case charges reused shared work again; invocation charges only actual builds. A case's combined shared stages add S and V. `solve_cases` groups only within that call; `solve_case` creates a new group scope. Combination caches are first occupied slot in operand order (`4413–4441`) and contain finish-time snapshots. A whole-invocation proof must conservatively count cold builds, retries/calls and combinations or bind exact proven cache origins; digest equality alone grants no deduction. One actual invocation meter must span all of them.

## Stage 0 conclusion

A useful partial primitive and condition bound is available. A complete practical no-wrap guard is NOT established here: the whole source-to-preparation path, all cumulative loop multiplicities, late/error tails, tracker/certificate lifetimes, all aggregate and stage merges, and exact invocation composition remain to be proved together. A source-count polynomial that bounds every path could be sufficient yet reject ordinary intended models; test its finite usefulness separately. Evaluate that polynomial with exact/checked arithmetic (including exponent/shift and coefficient multiplication) before native construction. Reject missing/overflowing premises prospectively to preserved ordinary fallback. Neither returned equality nor byte/work thresholds close this obligation.

An overflow-state alternative requires a separate complete counter-operation design and authorized write-set. It must cover ordinary additions, saturating merges and intermediate aggregate/delta operations and survive error/cached paths; a flag only on final LME would be too late. This review neither selects such a design nor authorizes a counter edit.
