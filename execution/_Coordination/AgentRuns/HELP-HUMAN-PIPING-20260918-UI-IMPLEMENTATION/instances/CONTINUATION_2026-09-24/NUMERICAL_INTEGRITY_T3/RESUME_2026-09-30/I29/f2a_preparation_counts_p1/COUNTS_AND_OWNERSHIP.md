# P1 source-preparation census and symbolic ownership component

Prospective component for independent review; no implementation or byte allowance.
Source is main 49034a940f; corrected C1/C2 at 0017eba992 / 0a4afd6318 and RV43
confirmation f2d6470460 supply the source/map/Call distinction. All names below are
conceptual interfaces, not reserved API/code names. P means projects/chirality-piping;
FK means P/core/solver/frame_kernel/src; R means the task's RESUME directory.

## 1. Borrowed census and its own admission

Input is a frozen borrowed view of the actual captured request, normalized model,
chosen actual material/basis objects, prepared boundary, admitted case/combination
recipes and typed routing evidence. Preserve I30's current case cursor, first
LegacyTerminal, diagnostic prefix and live ledgers. Existing ordinary owners are
R0, a separately bound resident term; newly reached ordinary/W2/basis/suffix work
is O_suffix, **not R0 and not this component**.

A RawCensus contains raw list lengths and per-field UTF-8 lengths/sums, including
duplicates/zero terms; normalized census records the same source recipe and its
provenance, not a rounded force net. No SourceParts, clone, encoding(), layout(),
body_nodes(), graph, sort or new map is run to obtain the raw census.

Concrete visitor contract:
1. Read only existing scalar lengths/capacities and borrowed strings/slices.
   C2 maps use full ids. Matching an existing built member/material/support can
   use bounded repeated borrowed scans; do not build a lookup HashMap for free.
2. Walk any generic captured Value with an explicit array/object cursor stack,
   no recursive calls, owned path strings or collected key list. A frame holds
   borrowed cursors only. Stop before pushing beyond its admitted capacity or
   spending its admitted visit credit; refusal preserves ordinary behavior.
3. The current CapturedInvocation stores encoded_len from the successful checked
   capture (PP/source_receipt.rs:100–128). Its immutable checked encoding length J
   supplies coarse V_raw≤J+1 and depth≤J+1 without another encoding. If this
   scalar/custody premise is absent, no such bound is supplied by a guessed hash.
   This counts already resident capture; it does not relabel capture parsing free.
4. Before allocating the cursor workspace, check J+1 and its layout and reserve
   V_Frame(J+1). It may instead borrow a smaller already-admitted frame buffer;
   depth/visit exhaustion then declines W1. No numeric workspace cap is selected.
   An implementation that allocates iterators must add those sites or is unqualified.
5. Typed C2 fields are traversed by explicit finite loops. Count visits as the
   sum of actual field/list entries, string bytes and repeated lookup comparisons.
   A sufficient comparison upper is the product of queried and candidate list
   lengths per named lookup; a combination visits each referenced occurrence.
   Checked addition/multiplication precedes scheduling that work. It has no W1 LME
   price. No data-dependent recursion, unbounded trace or normalized-model rebuild.

The census owner records exact input/cursor/basis identity, raw cardinalities,
UTF-8 sums and all capacity observations. Allocation/growth of the census itself
is part of the preflight expression. A fixed small failure discriminant may be
returned; formatting/copying its eventual diagnostic remains explicitly reserved.

## 2. Counts and exact native encoding lengths

For a case: n nodes, m admitted straight members, s boundary springs, r raw
constraints, l nodal terms, g support groups, Cs raw support-spring child entries,
Ca constraint-attribution support indices, and I=sum of nodal source-id UTF-8
lengths. C2 product d=0, zero constraint values, and t=3m stations.
Raw r may exceed6n or include duplicates; do not use valid-source facts early.
Canonical child counts are no larger than raw counts; dedup does not shrink
already allocated capacities.

After PrimitiveSource validation let N=6n, k=unique constraints, F=N−k,
b=bodies, f=free blocks, and v=distinct loaded DOFs. Then:
- v≤min(N,l), b≤n, f≤F; raw pattern positions P=144m+s;
  upper-triangle contribution count U=78m+s; stored Z≤min(N²,P).
- Layout Q=7n+12m+6t+s+k+2g=7n+30m+s+k+2g.
- Case K4SRC byte count:
  E_src=38+24n+84m+17s+13k+17l+I+16t+22g+4Cs_canonical.
- K4STF byte count: E_stf=26+24n+84m+17s+5k.
  Loads, constraint values, stations and support groups are not stiffness identity.
- Ledger K4LED bytes: E_led=10+18v+8Σ_j a_j, where a_j is actual net-limb length.
  a_j≤lambda_acc; current exact_sum.rs has lambda_acc=68. Cancellation may reduce
  actual length but supplies no preallocation credit. Arithmetic-format changes
  require a new bound/profile; 68 is not a hardcoded future byte coefficient.

For h ordered combination operands:
- Load visits L=sum_i l_i over **occurrences**, including repeated refs/zero factors.
  Prescribed pairs P_c=h*k. The first source is cloned natively, even when wire
  maps/materials/layout reference it rather than duplicating them.
- E_cmb=10+Σ_i(12+E_src_i), from header/count and each factor/length/source encoding.
  CasePrep owns the exact combined ledger and prescribed pairs; never allocate a
  rounded combined PrimitiveSource or multiply/net terms in binary64 first.
- CombinationSource wire metadata has h ordered factor/source/digest refs and
  one representative ref. It has no combined nodal_terms/constraint-value array.
- A pre_source_refusal retains requested-operands/Call/reason only. Its new
  Source/Group/Build/Run counts are zero; partial preparation allocations still count.

Each material basis registry is first-use ordered and records used actual material
descriptors once, plus its case-index references. Case maps reference that basis.
Equal selectors/hashes do not discount copies or prove equal E/G. No re-resolution
or new ordinary material construction is part of a supposedly free census.

## 3. Checked guards at the actual use

Evaluate all formulas with checked nonnegative integers; reject bool/fraction/
negative counts in mathematical input tooling. Before each Rust use/cast:
- Every Source::encode count/id and each source-id byte length fits u32.
  Check 6n,3m and3*member+j before allocation/indexing. t≤u32::MAX implies
  m≤floor(u32::MAX/3); for m>0 the last station id is3m−1.
- Global indices and N+1/Z+1,2P/2Z, tuple/array lengths and all products fit
  usize. Every array layout satisfies checked stride*capacity and Layout's
  target isize/alignment rules; a count fitting usize alone is insufficient.
- Every node/component reference is range-valid before indexed access. Preserve
  current native validation order for semantic source failures; raw memory
  bounds cover invalid prefixes rather than forging successful counts.
- All source encoding buffers fit their allocation/index type. An operand's
  CasePrep.identity length must additionally fit u32 **before** K4CMB prefixes it.
  Do not infer this from each constituent list length. E_cmb itself is checked.
- K4LED v and each a_j fit their written u32 widths; body/source/map/call/group
  ordinals and declared wire counters fit C2's safe-JSON U range when emitted.
  Typed private invalid data is not narrowed merely to create a refusal record.
- Before unchecked profile accumulation, F(F+1)/2 fits usize; before neighbor/
  contribution construction, the checked raw uppers cover every child insertion.
  The exact returned profile is later bound to the same prepared object.
- free_blocks uses u32::MAX as unvisited (bound.rs:90–106). Before assigning a
  new block, positions.len()<u32::MAX; final block count f≤u32::MAX is sufficient,
  or use the safe pre-loop upper F≤u32::MAX. Legal ids then stop at f−1, below
  the sentinel. The current stronger premise checks the unreduced F*(F+1)
  product in usize: on a≤64-bit target this already implies F≤u32::MAX.
  If a later implementation checks only the reduced triangle, add the explicit
  sentinel premise. RV48's reduced-triangle scalar witness does not satisfy the
  stronger unreduced-product premise and is not its counterexample.
- Later numeric/state-encoding consumers must separately check their N/Q/u32
  uses and I34 exactness/scalar premises. These preparation guards do not prove
  complete work-counter safety or permit a numeric phase.

Allocation/free-byte allowance is absent here: the guard accepts only a separately
adopted permit with bound coefficients. Unknown coefficient, stale source/count
identity, overflow or missing permit refuses this W1 preparation before its first
affected allocation. I30 determines original-base/first-terminal fallback; no new
ordinary block, code vocabulary, input family or diagnostic priority is chosen.

## 4. Symbolic requested/moving algebra

For type T, let stride s_T and alignment a_T be qualified actual build parameters.
V_T(x) bounds one Vec backing under every used construction mode (push/insert/
collect/clone/with_capacity); V_T(0)=0. A sufficient profile family is
V_T(x)=s_T(alpha_T*1[x>0]+beta_T*x), with checked evaluation and a proved
capacity contract. alpha/beta are **unbound parameters**, not copied K6c constants.
For q child vectors with total at most x elements, use
C_T(x,q)=s_T(alpha_T*q+beta_T*x).
String backings use V_u8(length) under their own cloned/formatted capacity contract.
Deque, tree, stable-sort scratch and Arc/Box request functions have separate named
profiles D_T(x), Tree_T(x), Sort_T(x), Arc_T. No assumed BTree node size or spare
Vec capacity is imported. Headers embedded in heap containers belong to their
element stride; stack headers are not counted as another heap allocation.

For each phase below, P_prev denotes actual prior W1 owners, with payload aliases
counted once only when actual shared ownership is proved. R0 is read-only here.
Let H_j be the sum of the listed W1 live/request upper terms for that phase,
including P_prev and records. A conservative pair is
E_requested_j=R0+H_j; E_moving_j=R0+2H_j.
The second deliberately counts every W1 term twice: each sequential growth site's
old backing is no larger than its final per-site bound, while sort/clone temporaries
are already listed. This is a coarse upper, not measured moving behavior.
For the repaired roster, H'_RCM=H_RCM+S_RCM and
H'_import=H_import+Delta_import (plus any actual new heap capsule holding the
cache header). Apply R0+H'_j and R0+2H'_j to those complete phase sums.
P_prev still includes the original snapshot vectors. Sort scratch is counted at
the RCM phase, not borrowed from the earlier constructor phase. Neither new term
is presumed absorbed by unrelated slack or an unbound status/type coefficient.
Before actual degrees exist, use a qualified Sort-profile upper over
0≤degree≤max(F−1,0), rather than running RCM early to obtain sort counts.
Concurrent source preparations require a separate aggregate composition.
Stack workspace has its own finite expression; none becomes an RSS bound.

The roster below targets the current ordinary, unseeded construction path.
source.rs:366–377 adds caller_order deep copies under cfg(test)/mutation-controls;
that branch requires explicit extra member/spring/directional/constraint/load/station
backings and copied IDs, not merely another sizeof coefficient. Any qualified
test/seeded profile must add those owners. No profile transfer is assumed.

## 5. Allocation/ownership roster and phase expressions

All referenced buffers survive until their actual move/drop; summing mutually
exclusive temporaries is permitted conservative slack, not claimed exact liveness.

| Phase / source | W1 heap terms before/while allocating; output ownership |
|---|---|
| Census / PP source_receipt.rs:100–139; C2 §7 | Cursor V_Frame(J+1), bounded census record/store if not stack, and fixed failure-storage reservation. Borrow raw/normalized inputs. No new raw Value/model clone. Iterator/frame/visit contracts and actual J binding are required. |
| InputMap/material/source registries / C2 §3–4 | V_NodeMap(n)+V_MemberMap(m)+V_SpringMap(s)+V_SupportMap(g)+V_ConstraintMap(r)+C_U(Ca,r)+V_NodalMap(l)+V_StationMap(t)+V_SectionMap(m), all actual owned UTF-8 copies, selected material/basis descriptors and owner refs. Build under admission, then move buffers into finalized maps where valid; same text held by native source and map is two owners unless actually shared. |
| SourceParts / retained/source.rs:156–178 | V_Coord(n)+V_Member(m)+V_Spring(s)+V_Constraint(r)+V_Nodal(l)+Σ V_u8(id_bytes_i)+V_Station(t)+V_Support(g)+C_u32(Cs,g). Product directional arrays are empty by C2, not by silently dropping data. Raw invalid multiplicities remain in this upper. |
| PrimitiveSource constructor / source.rs:320–599 | SourceParts plus max stable-sort scratch over member/spring/constraint/load/station/support lists (and qualified unstable-sort workspace), V_ExactWideSum(3) chord buffer, V_OptionF64(N), V_usize(n) parent,2V_u32(n) root/body maps. Parent/root are temporary; constrained and body_of_node survive. Errors retain partial buffers through their real return/drop; no body/layout/source hash on invalid source. |
| Source clone / source.rs:271–286; adaptive.rs:909–950 | Original source and one complete clone, including constrained/body arrays, every load String and support child Vec. Use raw-capacity bound for original and validated-length clone bound. This conservative derivation takes no move/consume saving; any further clone must be added. Original may drop at an explicit prepared-object transfer. |
| Exact ledger / ledger.rs:88–126,151–177 | With v≤min(N,term_visits): V_(usize,ExactAccumulator,bool)(v) coexists with V_(usize,LedgerNet)(v)+C_u64(lambda_acc*v,v)+V_bool(v). Netting creates outputs while accumulator entries remain borrowed. Zero/cancelling terms still create their DOF entry. ExactAccumulator/net_parts stack buffers are separate stack terms. Failed add/product drops actual partial owners. |
| Prescriptions/identity / adaptive.rs:909–976 | V_(usize,VecPair)(k)+C_Pair(k,k) for a case, or C_Pair(h*k,k) for a combination; zero prescribed values do not remove these allocations. V_f64(h) factors, V_u8(E_src or E_cmb) identity. Combination source-ref and prep-ref temporary vectors each≤h; first stiffness key plus one candidate key coexist during checks. |
| Layout/extents / recover.rs:101–198; adaptive.rs:950–976 | V_QuantityMeta(Q)+V_f64(b), plus at most V_u32(n)+V_Coord(n) temporary body lists/coordinates while extents grow. Do not call these allocating helpers during census. Source/map/ledger/identity remain live. Arc_CasePrep includes actual inline fields/status; retain this prep for execution. |
| Source/map identity encoding / source.rs:652–742; ledger.rs:218–238; C2 §3 | CasePrep identity remains owned. Stiffness key V_u8(E_stf) persists only with its actual group/key owner; K4LED temporary V_u8(E_led) is hashed/compared once then dropped under an explicit scope. Product map hashing includes typed maps plus qualified Value/canonical-text/escaping/hash scratch; see §6. No rebuild to recreate a supposedly free hash. |
| Body membership map / C2 §3 | V_BodyMap(b)+C_u32(n,b)+C_u32(m,b), with source.body_of_node borrowed and each node/member assigned once. Include isolated nodes. New map capacities use b≤n upper before allocation; do not infer membership from a different graph. |
| Geometry / factor.rs:130–218; rigid_body.rs:33–246 | Retained V_BodyGeometry(b); active body node/coordinate lists≤n, Tree_local(n), grounds≤r+s. Assessment: V_Coord(n)+2V_Row6(r+s)+V_Row6(2n+7); exact witness V_Expansion6(n) with per-node child terms≤3*10+3*1, plus V_Row6(n) recovered motions and local/difference expansion scratch. Expansion.add keeps old and new term Vecs concurrently. Failed geometry may overlap original and cloned geometry/error data. |
| Pattern/tagging / assemble.rs:539–648; structural/sparse.rs:48–160 | Let Zu=min(N²,144m+s), U=78m+s, P=144m+s. Bound V_Pair(U)+V_VecU(N)+C_U(P,N)+V_U(N+1)+2V_U(Zu)+V_Tag(U)+2V_U(Zu+1)+V_Contribution(U). This overcounts mutually exclusive row-building/tagging phases safely. Persistent result is pattern row starts/columns/transpose, starts and items; no numeric values/factor yet. |
| Ordering/RCM / factor.rs:223–401 | Persistent source/geometry/Structure plus free/position/order/rank/first (4V_U(F)+V_U(N)); adjacency V_VecU(F)+C_U(Zu,F); RCM neighbors V_VecU(F)+C_U(2Zu,F), degrees V_U(F), visited V_bool(F). Add S_RCM=max_i Sort_U(degree_i) for factor.rs:310–317 stable sort_by_key, after deduplication while adjacency/neighbors/degrees remain live; degree_i≤max(F−1,0). Sorts are sequential: maximum, not sum; the empty F=0 maximum is0. A monotone bound Sort_U(max(F−1,0)) requires its qualified sort profile; no absorption into Vec terms is assumed. Safe temporary allowance5V_U(F)+2V_bool(F)+D_U(F) covers component/old-last/current-next BFS levels and queue. Sequential peripheral searches do not require multiplying peak storage by iteration count. |
| Free blocks / bound.rs:84–125 | V_u32(F)+V_VecU(f)+C_U(F,f)+V_u32(f), plus stack/component temporaries2V_U(F). Components partition free positions. Actual GroupPrep owns final Structure/Ordering/geometry/blocks; Arc_GroupPrep and empty-cache/status headers need current coefficients. |
| Case Call/group metadata / C2 §4,6 | CaseBatchCall owner/source/run-ref arrays bounded by admitted batch occurrence count; Group records≤batch source occurrences, key bytes and source-ref lists counted per call. Geometry refusal gets actual Group/preparation reason; exhausted-before-start and other branches retain true associations. Empty slots are not factor payloads; Build records are created later, not fabricated during preparation. |
| Combination preparation / combine.rs:85–129; adaptive.rs:922–976 | Borrow operand preparations and selected snapshots in authored order. Check identity/layout/stations/supports before combining. Native clone is the first source only; exact ledger seesΣ l_i, prescriptions h*k, identity E_cmb. Reuse an actual selected GroupPrep when available; any genuinely new group takes the entire group expression above. New Group metadata is not proof a new graph exists. The actual GroupCache::merged owner additionally deep-copies selected failed verification vectors; the explicit import row below applies. |
| Combination cache import / adaptive.rs:3571–3591,3795–3827,4416–4438; combine.rs:114 | Construct the new GroupCache owner. For each of v256/v512/v1024, scan actual selected snapshots in authored order and select the first occupied slot (success or nonbudget failure); later occupied slots do not overwrite it. For every selected Err, read its borrowed vector length r_j and add Delta_import=sum_j V_BlockRefusal(r_j), at most three vectors. r_j≤f_origin_j follows from verify.rs:439–441,525–526 and bound.rs:1321–1332 (one Uc refusal option per originating free block). This upper is for that cache vector, not an arbitrary attempt-refusal list. The original vectors remain in P_prev; new copies belong to the new cache through its actual drop or execution transfer. Success payloads share Arc; source refs/inline status headers do not pay for copied failure backings. The current cache header is inline/stack; any new containing heap object's stride and its fixed stack/transfer storage are separately bound. |
| Pre-source/partial failure / C2 corrected §4,6 | Preserve requested-operands/Call/typed reason and actual temporary drops; source_refs/run_refs empty and meter snapshots unchanged. No combination source/map/ledger digest, group/import/build or Run is created after operand/ledger refusal. Nonfinite factors retain private native cause and existing receipt-encoding fallback. Valid combined-source group refusal is a different later branch with true source/Run ownership. |

Geometry scratch is finite at the current source: each coordinate delta has at
most2 expansion terms; each translation has at most 2+4*2=10 and each rotation
at most1. During exact_scalar, add a clone of at most 10 terms plus its new
next buffer of at most 11; during construction add three 2-term delta buffers.
These are element counts for the roster, not reused byte sizes or a source-action
proof. They follow rigid_body.rs:198–246 and structural.rs:711–780.

## 6. Format and record cardinality parameters

Typed map counts are fixed by C2; actual strides/status fields are not. For JSON
scratch, a recursive symbolic content bound uses: Text(b)≤6b+2 UTF-8 bytes,
B=18 bytes including quotes, H=66, bool≤5, null=4, and safe-U decimal≤16.
Object/array punctuation and exact fixed C2 key literals add their lengths.
Apply it to actual bounded cardinalities, including nested membership/attribution
and every text copy; serializer Value/container capacity, key-sort, canonical text
growth and escaping workspace still require named build coefficients.
A raw Rust type sizeof or serialized content length is not that heap bound.

A case-source decline has its owned InputMap/counted constructor payload and
typed error, but no valid source registry item. Combination Call count includes
pre-source refusals; Source/Run count does not. Valid groups are call-local and
first full-stiffness-equality ordered. Cache-import metadata has at most seven
slot refs per selected operand snapshot, but this reference count is separate
from the at-most-three actual failed-verification vector copies in the new cache.
Expose an owner-bound borrowed slot census (occupied state, refusal length and
origin block upper) before GroupCache::merged; a caller-supplied length or cloned
vector is not that census. Success Arc sharing does not erase Err Vec ownership.
After merging, transfer the new cache's owner and Delta_import backings into the
execution live set, or release them only at actual Drop. Later obtain_verify
failure copies and finish_selected cache.clone() copies belong to the later
numeric/returned-state roster; preparation does not silently pay for or erase them.
Numeric Build/attempt/state payloads remain outside P1. Reserve every origin
array actually initialized here; later growth requires its later permit.

The persistent registry roster also includes V_SourceRegistry(valid case+combination sources),
V_CallRegistry(actual calls), V_GroupRegistry(actual groups), V_BasisRegistry(actual bases),
and V_SourceDecline(actual case declines), with their owned reference arrays and text.
No numeric Build/factor payload is silently allocated in these preparation headers.

Changed recorded result types, I34 exactness/status fields, source-format helpers,
Arc/Box/Vec/map/Deque/std sort implementation, formatter and target/compiler facts
must be bound by a reviewed profile. No K6c private byte coefficient is substituted.
Unbound terms prevent numeric evaluation/admission; they are not zero. A new
dynamic allocation/field needs an explicit roster/cardinality amendment, not an
unconstrained coefficient hiding a missing algorithm.

## 7. Branch order, prepared-object transfer and refusal

The roster is an ownership inventory, not permission to reorder native preparation:
- A valid case PrimitiveSource/map may register CaseSource before its run. A
  case-source construction refusal has only source_decline and no Call/Run.
- For a reached case run, preserve exhausted-before-start first, then actual
  call-local group lookup/preparation, then CasePrep::new, as solve_cases does.
  A group refusal must not be replaced by an earlier speculative ledger failure.
- PreparedCaseSource for an ordinary combination operand calls CasePrep::new
  once and has no GroupPrep, factor, solved state or cache (C1 §1).
- A combination keeps operand checks first, then CasePrep::combination. Only
  successful combined preparation registers CombinationSource/WithRun. Reuse
  an actual selected operand's GroupPrep after compatibility; production
  ordinary-only mechanics is bypassed under C1, not an extra source-only run.

A prepared-run handle owns exactly the GroupPrep/CasePrep that produced the
counts; later execution reuses those objects rather than reconstructing a graph.
The preparation scope drops comparison keys/ref-vector/hash scratch before
handoff only through an explicit scope/drop; otherwise transfer those live
owners to the later component. Original source plus one CasePrep clone is the
conservative bound; an implementation that consumes/moves the original may
take that saving only with reviewed ownership evidence.

P1 does not settle where a future whole-batch numeric guard obtains every exact
group count. It may use safe raw upper counts or a separately admitted prepare-only
seam, but that seam must preserve C2 call/group ids and native refusal/budget
precedence. This is a remaining numeric/aggregate placement obligation, not
permission to execute an otherwise unreached group or emit fake origin evidence.

A permit is bound to input identity, raw/validated counts, actual prepared owners
and source/build profile. Missing proof, overflow or a failed reservation takes
the existing W1-decline/original fallback path before affected allocation.
Semantic constructor/operand failures retain their native order and typed cause.
Return the first legacy terminal when I30 routing requires it; never claim a
successful ordinary base that did not exist or rerun a prefix solve to rebuild one.

This component leaves numeric phases, retained solves/certificates, ordinary
suffix growth, PP/headless/native reply/completion and full invocation peaks to
their owning cells. Work-exactness/headroom is independent. No memory allowance,
RSS bound, new input domain, policy/code reservation or implementation is selected.
