# I35 integration-02 — entries, loops, ownership and allowance

The earlier A=`8192*(...)+4*(...)` and U=`2^21*(D+C)+A` remain historical reservations. **This integration does not use them as proved execution bounds.** It chooses an explicit finite visit-permit interface, together with the following closed numeric grammar and source-count loop ledger. No numeric permit value, facade LME price or charge to the kernel's 20B/60B limits is selected.

## 1. Scalar entries and I34 checked APIs

DA/DS/DM/DD/DQ/R4 own a fresh p=1024 WideContext and ExactWideSum; EC owns its fresh sum. B64U owns only the actual fresh sum inside the spent-return conversion seam in §1a; it has no WideContext operation. Each captures/merges the actual producing owners on *every* return and discards the value on refusal. There are no cross-entry cumulative local counters or accumulator clones. The destination facade meter uses the selected I34 `checked_lme`, checked merge and `WorkTotal`/`WorkStatus` transitions; no numeric legacy getter is imported as exact evidence. The existing source algorithms supply the actual WidthWork/SumWork counts, not the following upper reservations.

| Entry | Finite operation grammar | Fresh numeric work upper / carry fact |
|---|---|---|
| DA/DS | existing directed add/sub, at most one step | <=5 lifetime raw insertions, <=3 live; <=2 Round; SumWork<=2922 |
| DM | existing exact TwoProduct, directed round, at most one step | same raw bound; TwoProduct=1, Round<=2; SumWork<=2922 |
| DD | nearest div; exact q*b−a; at most one step | 1025 div iterations; Div=1, TwoProduct=1, Round<=1; SumWork<=2922 |
| DQ | nearest sqrt; exact q*q−a; at most one step | 1026 root iterations, 17 active limbs; Sqrt=1, TwoProduct=1, Round<=1; SumWork<=2922 |
| R4 | four saved exact binary64-product terms; initial sign check on lower pass; round_toward; at most one step | **7 lifetime raw inserts, 5 simultaneously live before clear**, <=2 sign observations and <=2 Round. term<=1038, shift<=1792, net<=768, rounded<=256: SumWork<=3854. Span<=4199 before clear, <=1025 at step. |
| EC | fresh exact final comparison, <=5 direct/scaled terms | <=5 live, no rounded core call; SumWork<=2218 |
| B64U | new narrow binary64_up_spent seam around the unchanged binary64_up algorithm | RN binary64 conversion; actual fresh exact x−candidate sum/sign; at most one bit successor, no loop. Returns actual SumWork/status. <=1152 is a local safety upper only, never a spent charge. |

All rows preserve range/span/zero/sign rules from arithmetic_01/RV50. R4 is the explicit extension to the earlier five-insertion grammar. Its five live terms still give magnitude <5*2^8128<2^8131; its stronger actual source span is 4199. Seven insertions over the helper's *lifetime* do not mean seven simultaneously live terms. Positivity is the exact numerator sign, not rounded output or endpoint positivity. The four-term sum is rebuilt for its second direction, never cloned or cleared into an unrecorded fresh counter.

I34 changes signum/is_zero/make_absolute/net to fallible observations; every use here propagates `?`. Its pre-mutation prospective charge checks, pending base charge, checked carry/index containment, poisoned-value discipline and state-preserving full reset apply unchanged. Numeric Span/Exponent aborts this facade certificate rather than retrying a block. WorkAccounting cannot be swallowed as Span/Exponent or converted through `.ok()`/default zero. Collect both context and sum status before dropping either; an otherwise successful value with non-E work is unusable. Preserve the original numeric refusal alongside accounting state when both occur. No reset restores exact evidence.

The primitive arrays/loops remain the reviewed fixed ones (16x16 product, 32-limb double buffers, 1025 div and 1026 sqrt steps, <=128 carry tail, 8128-bit span). I34 adds checked transitions inside those loops; their actual candidate correspondence is a prerequisite, since I37 is not a stable source input. We do not assert a machine-instruction or wall-time bound from their source-level counts.

## 1a. RV50-INT02-1 correction: B64U exports its producing owner

Select **spent return**, not an outer unused accumulator. Add to the later full-integration `directed.rs` seam (not I39's §0 two-helper scope):

    binary64_up_spent<const L>(x: &Wide<L>) -> Binary64UpSpent
    Binary64UpSpent { result: Result<f64, AttemptStop>, sums: SumWork }

Both fields and construction stay private to the retained conversion integration. A consuming `into_parts` is available only to the actual facade collector and legacy numerical wrapper; no public constructor or serialized arithmetic record is added. SumWork includes the selected I34 status. B64U has no WideContext call, so its WidthWork contribution is exact zero, not an invented rounded-operation count. The existing `binary64_up(x)->Result<f64,AttemptStop>` becomes a thin **legacy numerical projection of this one execution**. It retains the current nonnegative precondition, result/error/zero/infinity behavior and numerical operation order; it remains the legacy historically unpriced route. No new charge is silently installed in its callers.

Implementation structure: preserve the same to_binary64 match. A nearest Overflow returns `Ok(+infinity)` plus fresh exact-zero SumWork, with no sum created or numeric sum event. Otherwise create one fresh sum, then evaluate the existing two additions and fallible sign inside a local Result-producing closure. `?` may leave that closure, **never the enclosing function before capture**. On either Ok or Err, read `sum.work()` from this same owner, place the unchanged numeric Result and that actual work/status into Binary64UpSpent, then drop the sum. No clone, replay, inferred-count reconstruction, theoretical charge or second sum is used. On zero, the two zero additions/sign are still executed as the current algorithm does; their actual returned zero/prefix counts, not a guessed shortcut, are captured.

The full facade's B64U entry consumes its one entry permit before calling the seam once. It merges returned SumWork into its checked facade sum owner, joins returned work status **and any WorkAccounting fault in result**, and only then extracts success. Collection has no early `?` capable of dropping result/work before both have been observed. Non-E or a merge failure blocks success; a simultaneous original numeric error is retained alongside the joined status. A first/second-add or sign refusal therefore exports exactly the actual committed prefix and flags. Rejected prospective work is not fabricated as spent. The caller subsequently checks that a numeric successful result is finite: both early overflow and MAX-successor infinity become the existing finite-scale/range refusal **after** collection. The legacy wrapper continues returning infinity as before.

The SIF operational-k call chain is concrete: DM forms the exact k_sqrt2*i product and collects its own actual entry work; B64U then calls binary64_up_spent on that product and collects the conversion work once; positive finite k is used by the unchanged four-operation stress-scale expression. A failed DM never dispatches B64U. B64U's numerical return never by itself proves its work was collected. Typed facade error/receipt paths retain both entries' spent prefixes.

Count correspondence: one B64U entry and one value conversion per attempted SIF-k conversion; on non-Overflow, at most two raw-add attempts and one exact sign observation, plus zero or one scalar successor. WidthWork=0; actual SumWork is what the producing owner exports. Early Overflow has no sum events; MAX-successor infinity has the two-add/sign prefix. Preserve both in controls. The <=1152 SumWork upper proves only local safety; it is neither an equality nor a tariff. No conversion retry or B64U cache is introduced. The seam's spent record moves the existing work fields into caller custody; actual return/header liveness is included in C_ctl, with no new endpoint or limb array and no claimed Rust layout.

Required later source/tests: `directed.rs` spent struct/core/legacy wrapper; `product_certificate.rs` B64U collector and SIF caller; existing `FK/tests/retained_k4/directed_tests.rs` or a separate narrowly registered B64U test module for legacy parity, actual exported counters/status, every numerical branch and injected refusal-prefix custody. Do not edit I39's `directed/certificate.rs` or its helper tests for this repair. The same-file integration hunk is serialized by ROOT. Abstract repair controls are in `_run_records/b64u_repair_03`; actual Rust collection/parity remains a later code witness.

## 2. Numeric count vector, without hiding B2 in 47q

Counts are from the frozen case. Let mX exact-profile, mB ordinary base/point, mI ordinary interpolated members; m=mX+mB+mI, mOrd=mB+mI. Let n/c be free/constrained DOFs, b blocks, v bodies, s actual station visits, k direct spring visits and a actual attributed-support-term occurrences.

Let qD, qA, qT, qS, qC, qO be required verified direct, signed-quotient, torsion, SIF, circular-maximum and surviving-open rows. Add superscript o for the ordinary subset of a stress family. qV is their sum; total q additionally includes existing InputDerived and other disclosed row classes. Do not populate qO with retired rows or qS/qC/qO from a prohibited combination. Define

    AS = qD+qA+qT+2qS+6qC+6qO
    AK = qA^o+qT^o+2qS^o+6qC^o+6qO^o
    NS = qS+qS^o; NC=qC+qC^o; NO=qO+qO^o.

AS is the number of source action-interval constructions when operands are not cached across rows. AK is the corresponding represented-only construction count. Streaming maxima price both ends, including repeated axial intervals even where a copy could save work. qAbs/qRel are final selected classes, qSmall<=qAbs the small-scale rows; qMM/qKN/qMPa count required verified raw unit conversions. gMag counts actual magnitude rows needing component-error sums, with at most three components each.

| Phase/family | DA | DS | DM | DD | DQ | R4 / other |
|---|---:|---:|---:|---:|---:|---|
| Common member builder, **per construction** | 2 | 18 | 23 | 11 | 0 | <=15 exact shifts is a common upper including material shifts; no material arithmetic in this row |
| Exact material, per construction | 2 | 0 | 0 | 2 | 0 | exact E/nu branch |
| Ordinary base/point material | 0 | 0 | 0 | 0 | 0 | exact lifts/comparisons only |
| Ordinary interpolation, per construction | 0 | 2 | 8 | 4 | 0 | 4 R4, 9 extra source lifts |
| F2 contractions, per member | 248 | 0 | 236 | 0 | 0 | includes <=12 M constructions |
| F3 contractions, per member | 280 | 0 | 244 | 0 | 0 | includes scatter and <=12 M constructions |
| Per block | 0 | 1 | 2 | 1 | 0 | exact beta shift, strict comparisons |
| Per station visit | 24 | 1 | 24 | 0 | 0 | actual fractional/side association |
| Springs/support terms | k+a | 0 | k+a | 0 | 0 | at most k+a T shifts; visits are occurrences |
| Q_S / Q_K construction | 2AS+AK | AS+AK | 0 | 0 | 0 | no hidden single-operand assumption for B2 |
| Mapped magnitude bridge sum | <=3gMag | 0 | 0 | 0 | 0 | component T/e bounds; source norm value is not recomputed |
| B1 signed quotients, both required branches | 0 | 0 | 0 | 8(qA+qA^o) | 0 | 6 min/max comparisons per branch |
| B1 torsion, both required branches | 0 | 0 | 16(qT+qT^o) | 16(qT+qT^o) | 0 | 14 comparisons per branch |
| B2, both required branches | 2NS+8NC+8NO | 0 | 6NS+8NC | 2NS+8NC+8NO | 2NS+4NC | abs-range calls=2NS+6NC+6NO; max comparisons=2NC+2NO |
| Represented Z in F3 | 0 | 0 | 0 | 2mOrd | 0 | verify circular axis identity; two hull comparisons/member |
| Final raw truth interval | 0 | 0 | 2qMM | 2(qKN+qMPa) | 0 | SI/rad identity otherwise |
| Final Hn/HU | 0 | 4qV | 0 | 0 | 0 | two max comparisons/row |
| SIF operational k_i | 0 | 0 | qS | 0 | 0 | qS B64U spent-return entries, exact input product; collect actual returned sum work |

Multiply common and route-specific builder rows by **three**. Sum columns with checked nonnegative integer arithmetic. The exact-profile builder is 58 entries; base/point 54; interpolation 68 directed calls+4 R4=72. The material-replacement count agrees with I36's +42m interpolation common upper, but the route-separated table avoids charging base/point as if interpolation occurred. This replaces the old single 47q B1 reserve for integration; B2 and its extra action inputs are now explicit.

Ordinary stress SI hulls use two exact endpoint comparisons per qA^o+qT^o+qS^o+qC^o+qO^o. Each abs range has <=2 abs operations and <=4 comparisons. All endpoint order/finite/denominator checks are separately visited in the loop ledger.

Final EC entries are bounded by

    qAbs + 4qRel + 64qSmall.

The small-bound ECs are four-term comparisons. Failure/short circuit can reduce actual work but cannot omit already entered work. Norm/source recipes, raw conversion and final predicates execute once on the hull; no duplicate final predicate per branch is necessary.

Observable arithmetic is a separate exact count vector: normalization <=2q f64 operations; one maximum midpoint costs 1 subtraction+1 multiplication+1 addition; each support/combined magnitude has 2 ordered hypot calls; each SIF observable has 1 hypot+1 division+1 multiplication; each representation guard has 1 subtraction, 2 abs, 1 max, 2 multiplications (conservative, even if 64*epsilon is folded), and 1 comparison. Headline numeric comparisons are at most two complete row scans plus per-case completeness visits; lexicographic tie bytes are metered. The five A_f64 operations, class threshold, b0/r fixed scale-back operations, four-operation stress scale, four body-coupling operations and existing p512 min/max checks are named scalar events, not buried in an unexplained 32q coefficient. The exact implementation records actual branch-specific counts.

## 3. Loop, callback and lookup ledger

Each row below identifies an implementation loop or a remaining prerequisite. P1's raw borrowed census and corrected allocation/import/RCM ownership stay in force. Its exact Q/layout and K4 encoding lengths may be used only after their source/count prerequisites. No new sort, normalized-model rebuild, encoding, source clone or cache import is performed by the certificate to obtain counts.

| ID | Loop/access site | Closed bound / implementation correspondence |
|---|---|---|
| L0 | Raw generic Value/capture traversal, if required | P1 explicit cursor stack; encoded length J gives visits/depth<=J+1 only with captured-custody premise. Preserve its permit/exhaustion and frame ownership. No recursive finite_tree call is substituted. |
| L1 | Full source/ledger/stiffness equality | Compare length then explicit borrowed-byte loop; <=2 times the sum of compared byte lengths. No hashing/encoding/cloning call. Count every actual repeated comparison. |
| L2 | qK publication/layout/radius validation | One qK loop after shape checks. Preserve all current owner/precision/policy/meta/class/sentinel/finite/ceiling checks. At most five RN64 allowance operations for a relative ceiling, plus named scalar tests. Later radius access is direct checked-index borrowing, not another full identity check. |
| L3 | Member/node/material/station/support/final maps | Linear field validation over each admitted flat slice. If a referenced ordinal must be resolved from a legacy list, use an explicit repeated borrowed scan with query_count*candidate_count upper, metering each compared byte. That lookup is a counted P1/P2 adapter site; no HashMap is built for free. |
| L4 | Final ids/binding uniqueness | Pair scan i<j: q(q−1)/2 comparisons. Equality checks count actual bytes, with upper (q−1)*sum_id_bytes when both operand byte reads are counted. No new sort/map allocation. Similar source-id checks use their own named list lengths. |
| L5 | Ordinary point validation | For a member with p_m actual point records: one p_m pass to choose below/above/at-target and <=p_m(p_m−1)/2 temperature comparisons for duplicates among present temperatures. Record missing fields and resolver validity. Exactly-once F0 validation/member; three numeric material builds reuse this immutable witness. Existing resolver sort/format/copy work remains ordinary-source work, not this loop. |
| L6 | Row grouping | q count/fill scans and m+1 prefix loop, checked prefix/cursor increments; q ordinals, m+1 offsets, m cursors. Drop fill cursors before numeric work. No sorting. |
| L7 | Source builders | Exactly 3m member visits; constant scalar construction schedules in §2. Borrowed input access consists only of fixed-field reads by validated ordinal. No callback may run a resolver, source constructor, allocation or sort. |
| L8 | Matrix-vector contractions | F2: two traversals of fixed Bbar 48/D 10/BbarT 48 and 12 scales/scatters/member. F3: two traversals of Bbar 48/D 10/HbarT 16/BbarT 48 with explicit scatter additions. Dimensions are constants, not a sparse-library callback. |
| L9 | M/T/scaling accesses | <=24m M-row accesses for F2/F3 plus AS+AK final action-row accesses; T reads by checked DOF/block mapping. No lookup by coincident values or units. Each access reads a bounded record/indices from the same immutable owner. |
| L10 | Data flags | n free-position visits plus at most z actual prescribed-pattern visits. A ledger presence lookup has an explicit prospective lower-bound binary search over vL sorted ledger entries: at most bit_length(vL)+1 key probes, then <=1 equal-key/nonzero-flag read. Add a narrow private metered lookup/fill seam; do not assert a bound for an uninspected std-library implementation. No net-load shortcut. |
| L11 | Block maxima/tau | <=2n max visits then b blocks. Verify data/zero and associated body B before division. No repeated alpha tests or refinement. |
| L12 | Station/support iterations | s station visits, k direct springs and a support-term occurrences. A repeated support lookup contributes again to a. Source map adjacency/attribution is validated; source omission is not a smaller count. |
| L13 | Source/represented recipes | §2 row-family counts, streamed fixed corners (4/8) or two B2 endpoints. No maximum optimizer, adaptive subdivision, atan/pi series or unbounded nextafter loop. |
| L14 | Final rows/coverage/scales | One q normalization/max scan, <=v body coupling passes, one final visit/coverage-bit update per row, one q completeness scan. Source/member stress scale is on demand with fixed formula. Private observer/receipt qualifiers do not create extra row copies. |
| L15 | Extrema/SIF evidence decode | Explicit iteration over each actual evidence object and its fields; fixed accepted key roster (13 extrema fields). A bounded key-match loop and seen mask replace hidden repeated generic map lookup. Every number/string field and byte is visited. No collect, owned path string or JSON clone. serde_json number conversion/iterator layout must be qualified for the actual features in P5; not assumed allocation-free across all builds. |
| L16 | Observable component resolution | Prefer validated FinalBinding ordinals. When preserving a legacy last-match nodal rule, an explicit <=q scan chooses that actual last row; support uniqueness is checked, not inferred. Count query_count*q scans if ordinals are absent. Zero matches refuses. |
| L17 | Headline finalization | Two scans over actual invocation rows qInv (stress/displacement), plus case completeness list. Compare emitted values; ties compare actual UTF-8 ids bytewise. No sorting or source-interval aggregation. A typed private alias record is constant-size; actual final serialization remains P4. |
| L18 | Cache/verification selection | Closed requested P match to one of three verification slots; at most four retained state inspections with an enforced source-profile shape. No build, cache merge, failed-slot clone or report allocation. If actual candidate permits another shape, revise this source bound before use. |
| L19 | Fixed scratch clear/copy/lift/compare | Loops over exactly the declared endpoint/limb arrays or their checked used prefixes; binary64 lift <=16 limbs, endpoint compare<=16, abs/sign copy<=16, explicit exponent/index checks. Each full variable-array initialization/reset visits its admitted length. No source exponent controls allocation or loop length. |
| L20 | f64/hypot/format/allocator boundaries | Named f64 operations and hypot calls are counted as such, not guessed instructions. No diagnostic formatting or allocator call is hidden in a numeric error path. Existing libm implementation, allocation/capacity and eventual diagnostic/serialization bounds remain P4/P5 prerequisites. |

Flat proof records are the single source-owned C2/private map representation, not a second normalized model. PP prepares admitted `MemberProof`, `MaterialSelectionProof`, `FinalBinding` and `ObservableBinding` records using its actual producer facts. The FK evaluator consumes borrowed slices of those records, not a generic user closure. Their record strides/capacities and construction loops belong to P1/P2/P4. Any unavoidable integration callback must have one of the listed fixed field-access bodies; otherwise its name/cost/allocation is an unresolved prerequisite and the caller cannot qualify W1.

All pair products, q+7, m+1, 2n, 3b, layout counts, index sentinels and target Layout bounds are checked before scheduling/allocation, using P1's raw-before-canonical distinction. Record actual UTF-8 byte multiplicities; B_source alone does not cover repeated row ids/keys. This ledger is the code correspondence to verify, not proof that current source already implements these sites.

## 4. Explicit finite visit allowance and failure accounting

The caller must provide an admitted, source/count/profile-bound private permit; no missing permit defaults to u64::MAX. Its values remain ROOT-owned policy. The permit has separate finite u64 allowances for:

    numeric entry counts by DA/DS/DM/DD/DQ/R4/EC/B64U,
    source/list/loop iterations, scalar field decodes,
    borrowed byte reads, key/ordinal probes, map writes,
    f64 operations, hypot calls, and admitted allocation sites.

Before each event the private meter checks existing WorkStatus, checked-adds the prospective count, and checks the corresponding allowance. Only then does it commit the count and perform the event. A visited scalar entry counts even if its numeric body then refuses; core work records the actually incurred algorithm prefix. If prospective accounting itself overflows, latch O and stop before that event. A permit miss with otherwise exact counters is a distinct VisitExhausted refusal: the rejected event is **not** executed or charged as spent. Return its category/location and exact committed prefix. Neither condition is an assertion that actual spent work exceeded MAX. No counter or permit resets on next phase, next case or fallback; per-case locals may start fresh only under the enclosing invocation owner.

Every data-dependent loop consumes before its next body/byte/probe. Every arithmetic entry has the reviewed fixed inner grammar, including the new seven-insertion entry and I34 checks; it consumes its entry permit before dispatch. Library/host hypot is separately counted but has no claimed source-level time bound until P5 qualifies it. A visit permit is a deterministic algorithm-admission/exhaustion mechanism, **not a wall-time/RSS bound** and not an LME tariff. Source/caller work executed before this permit remains separately recorded under its owner. Unknown or hidden callback work cannot be relabelled a single cheap field access.

FacadeSpent retains the actual WidthWork/SumWork counters, entry/visit counts, joined status, phase and optional original refusal. Use I34 checked aggregation and public exact extraction only on E. Join dependency/exit status before accepting a value or dropping a scope. No arbitrary caller-supplied WorkTotal constructor or foreign snapshot pair is admitted. This schedule uses fresh-operation collection, so it needs neither CloneWork nor chronological subtraction of facade totals; invocation call custody remains the selected C2/I34 core.

Keep facade counts **separate** from existing kernel RunWork/InvocationMeter amounts until ROOT explicitly selects their work/admission integration. Non-E facade evidence invalidates a successor receipt; it does not refund kernel work or change an earlier numerical row's value/standing. A later caller retry is a new counted call with the same enclosing invocation history, not a free local reset. No retry exists inside this certificate. Unknown pricing/qualification remains a pre-execution W1 admission prerequisite.

## 5. Constant and source-count storage schedule

Retain arithmetic_01's logical slots: Member64, Vector48, Row24, Primitive16 =152 endpoints. Add I36's material21, represented-Z2 and saved-first-recipe2 =**25 endpoints**. No overlay is assumed. Exactly 177 active endpoint slots plus two readonly pi endpoints are reserved; every endpoint's field payload remains 137 bytes. The existing limb scratch is 659 u64 slots (sum magnitudes256, term130, scaled17, net128, core128), unchanged by R4.

Thus explicit endpoint/limb payload is `179*137+659*8=29,795` logical bytes. The earlier extra 1088 control-byte reservation is **not requalified** by adding I34 fields. This integration instead requires the actual typed control/header profile C_ctl below. Constant payload is `29,795+C_ctl`; no concrete actual Rust size is asserted.

C_ctl is the finite sum of the actual selected types and fixed call frames: one fresh WideContext<16> header/counters, one ExactWideSum metadata/work/status/poison header, one checked facade accumulator/status, one visit-permit/count record, one constant typed failure record, one member/row phase cursor record, fixed extrema/SIF decode/seen-mask record, and the non-owning slice/reference headers for the workspace and inputs. I34 WorkTotal/WorkStatus and actual indices/enum payloads are included. The source candidate must enumerate these structs/locals and prove their call-frame overlap; P5 supplies stride/alignment/generated-copy facts. There is no unnamed growable control registry. Missing C_ctl/profile prevents a complete memory admission claim, but does not change the fixed mathematical arrays.

| Phase | Explicit endpoint liveness |
|---|---|
| Material construction | 21 extra slots: 9 inputs, 4 exact products reused for E/G, 2 h, 2 numerator, 4 H_E/H_G. Release the first17 before geometry; the last4 remain until coefficients are built. Exact profile/base/point may use fewer; reserve all21. |
| Geometry/coefficient construction | Original Member64 schedule, with source material cover borrowed from the extra4. Its retained c,A/Z/J, invell and compact DG/Ddelta occupy their existing slots. No m-sized coefficient cache. |
| F2 | Vector h12, B*h6, D*...6, contraction12; local-end12 slots idle. eta/v occupy two n-sized slices. Per-block eta/v/tau take3b endpoints. |
| F3 action | Same vector slots; local-end12 accumulates both error families and remains while rows are checked. c reaction accumulators occupy the reset shared row array. Represented-Z2 persists only for the active ordinary member. |
| B2 row | Row0–5 hold one end's N/My/Mz intervals; 6–11 their abs ranges;12–13 norm/sum;14–15 current endpoint result;16–17 running endpoint maximum;18–23 bounded scalar/shape temporaries. Primitive16 handles nested scalar calls. Reuse current-end slots for the second end. |
| Dual readout | Save the first complete recipe interval in the extra2 slots, stream the second in Row24, then hull. Do not retain twelve source+represented action intervals together. |
| Final predicates | Release action/abs temporaries; reuse Row24 for SI/raw endpoints, n/y, Hn/HU and exact allowance operands. R4 and EC never need simultaneous sums. |
| Failure | No successful value may borrow poisoned scratch; collect its status/prefix, then drop the same owners. Formatting/copying a public diagnostic is an explicit later P4 operation. |

Variable new arithmetic/index payload remains

    137*(max(2n,c)+3b) + block_tags(b) + 32v
    + 8q + 8(m+1) + ceil(q/8),

using 64-bit logical index slots, and the map-building phase additionally uses8m cursor bytes before numeric scratch becomes live. Block tags require explicit selected status/zero-data representation in the actual profile, not an assumed one-byte Rust enum. Source-owned proof-record/map backings add `V_MemberProof(m)+V_MaterialProof(material_selections)+V_FinalBinding(q)+V_ObservableBinding(observable_records)` only if not already the same C2 owners; identity and alias proof decides reuse. Their construction and capacities are P1/P2/P4 obligations, not hidden in the constant payload. P1's corrected failed-cache Vec clones/RCM sort terms remain in their own phases.

Borrowed solve/cache/source/ordinary outputs and final envelope, source maps, existing optimizer/resolver work, earlier cases, caller/receipt/hash buffers and concurrency overlap all remain P2–P5 terms. Logical payload does not establish allocator capacity, actual stack, generated temporaries, whole-process memory, universal format fit or actual availability.
