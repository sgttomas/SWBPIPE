# P3 — aggregate retained-kernel ownership (revisable)

Status: PROPOSED; source ownership derivation, not a qualified byte profile.
Basis: maintained main 49034a940f3f8cd3f3da4d4cbc839943b808063d and K6c
81c03849033f3ce745668f581f446530789397b8; checked-work candidate
fdae294643b798c1849da8b2e643085562593686 is unchanged and RV51-cleared per ROOT.
RV52-1 is corrected below; this documentation awaits the same RV52 backcheck.
P1 b1ebe245ec and RV48 confirmation, corrected C2 0a4afd6318/RV43, M1 and
I34 API 51d8d9fc1e remain the selected design basis. No code/policy changes.

## 1. Metric and input contract

This is requested heap allocation ownership, plus an explicitly separate stack
term. It is neither RSS nor allocator-resident memory. Freed blocks need not
leave the allocator/process. The eight failed RSS projections and unavailable
precision-isolated heaps do not become evidence for this expression.

For each real run j use P1's borrowed census/preparation identities and validated
counts (N=6n,F,m,s,d,b,B,Z,H,Q,k,source-byte-length,ledger-byte-length), exact
profile row lengths h_i and block lengths f_i where available. Sum h_i=H and
sum f_i=F. Product C2 selects d=0 and zero prescribed values, with Q=7n+30m+s+k+2g;
these are adapter premises, not universal native-input facts. All count/index/
encoding checks are upstream obligations from P1/I37, not waived by this packet.
Let tau be the number of nonzero prescribed operand pairs, tau_max the largest
per-DOF child length. Product C2 gives tau=0 only after its actual mapping is bound.
For a native combination with h operands, tau<=h*k; keep tau until then.

V_{site,T}(k)=stride(T)*Cap_{site,T}(k) bounds one vector backing at its
maximum occupancy k. Cap covers that exact constructor/growth/collect/clone/
shrink site and retained high-water capacity; it is NOT asserted equal to k.
Use the maximum applicable site law when the site subscript is omitted.
C_T(a_1,...,a_r)=sum_i V_T(a_i); when only total A and row count r are known,
C_T(A;r)=max over nonnegative a_i with sum<=A of that sum. This is finite under
qualified finite Cap; the coarse r*V_T(A) is available. Vec headers are charged
in the outer V_Vec<T>(r), not again per child. Empty cleared vectors use their
prior capacity; V_T(0) is valid only for a truly unallocated/qualified empty site.
A_T is the allocation request for ArcInner<T> including control/padding/inline
T; X_T is Box<T>'s request. Do not add inline T again. All T fields, discriminants,
Option layouts, padding, Arc metadata and alignment remain target/profile facts.
Tree_{K,V}(k) and Sort_T(k) are qualified BTree and stable-sort workspace bounds.
Neither fits into unspecified vector slack. Stack_frame(source,build) includes
inline contexts, fixed sum arrays, CloneWork and call-frame overlap separately.

## 2. Allocation identity and aggregate expression

An allocation token is (actual construction event, field/site, copy generation).
An Arc reference adds an edge to its existing token; equal source bytes/digests
never merge distinct construction events. A Vec clone creates a new token.
A move changes an edge and leaves the token unchanged. Inline headers are part
of their actual parent allocation (or stack), never independent heap requests.

Let Roots(e) contain all live input/prepared owners, group table/cache headers,
completed outcomes retained by M1, operands and combinations, current schedule,
current native publication, and actual C2 inventory buffers at event e. Let
A(e) be the reachable allocation tokens plus local temporary allocations at e.
With w(a) the bound for its actual backing/capacity, define:

    K_requested = max_{e in E_M1} sum_{a in A(e)} w(a)
    K_moving    = max_{e in E_M1} [sum_{a in A(e)} w(a) + sum_{o in Old(e)} w(o)]
    K_stack     = max_{e in E_M1} Stack_frame(e, frozen source/build)

Old(e) contains old backing still live during realloc/shrink/Vec-to-box moves,
excluded from the new allocation token's requested metric. Ordinary clone
source+destination overlap already belongs to A(e), not Old(e). Stable-sort
scratch also belongs to A(e). Include ancestor scratch around nested allocating
calls; do not assume every old-capacity term can be independently maximized and
then discarded. A coarse sum of all applicable old backings is conservative.
For sequential allocation calls, max of their old backings suffices only after
the enclosing lifetime roster is fixed. No allocator-transfer credit is taken.

E_M1 is finite: authored case order, actual group match/build, then authored
combination order and first-occupied import; within a run the source schedule
below and each bounded buffer growth/copy/drop transition. Branches not known
before solving are maximized over. This does not change the scheduler.
At each new stage retain P1 owners and every prior outcome still held by the
actual caller; add the new-stage tokens by UNION, not by adding another single-
case H baseline. Returned terminal data remains through caller custody/release.

A conservative alternative when dynamic origin tokens are unavailable is:

    K_req <= P1_all_live + Container_headers + C2_origin_delta
             + sum_{real run j} RetainSuperset(j)
             + max_{j,phase} ScratchSuperset(j,phase)

RetainSuperset allows one distinct successful S at each of four precisions,
one VS at each of three verification precisions, one solved state per precision,
all actual cache-failure copy generations, attempts/evidence/geometry/publication
for j, and terminal containers. Count an independently allocated build per real
run even if this duplicates a possible shared value; no undercounting dedup is
required. Partial new-build buffers are bounded by their eventual full children
plus stage scratch, including stopped builds that never produce an Arc. Cache
metadata for their Err result is additional. This sum is deliberately loose;
it is not a proposed numeric allowance. P1/C2 owners listed here are interfaces,
not a second addition if already in P1_all_live. Add moving old backings as above.

## 3. Persistent numerical owner weights

For p=(128,256,512,1024), L=(4,4,8,16), residual R=(4,8,16,16).
Verification exists at p=(256,512,1024), W=(8,16,16). W and R happen to have
matching limb widths there, but q_W and q_R differ. Keep semantic types separate.

    Factor_L = 2 V_usize(F) + V_Vec<Wide<L>>(F)
               + C_Wide<L>(h_1,...,h_F) + V_i64(F) + V_PivotScreen<L>(F)
    S_p = A_Shared<L,R> + V_MemberOperators<L>(m) + V_DirectionalBlock<L>(d)
          + V_Wide<L>(Z) + V_Wide<R>(Z) + V_BoundedCoefficients<R>(m)
          + V_DirectionalBlock<R>(d) + Factor_L + V_Wide<L>(B) [last empty at128]
    Z_p = A_Solved<L> + V_Wide<L>(N) + V_[Wide<L>;6](m) + V_Wide<L>(Q)
    VS_p = A_VerifyShared<L,W> + V_Wide<L>(Z) + V_[[Wide<W>;12];12](m)
           + V_[[Wide<W>;3];3](d) + V_BlockBound<L>(B)

Factor_L counts children only; RetainedFactor's inline struct is in Shared.
A cached success and every snapshot/import reference reach the same S/VS token.
Z_p is case-specific; a verifier reused as next candidate reaches the same Z_p.
The states vector contributes V_PrecisionState(at most4) independently of Arcs.

    Report_p = A_VerificationReport<L> + V_[f64;2](b)
               + 5 V_Option<Wide<L>>(Q) + 2 V_Wide<L>(F)
               + V_BlockCertificate<L>(B) + V_BlockNorms<L>(B)
               + V_Option<Wide<L>>(B) + V_BodyReport<L>(b)
    Summary = V_[f64;2](b) + V_f64(b) + V_Option<f64>(b)
    Attempts = V_AttemptRecord(a<=4)
               + sum_{records with summary} Summary
               + sum_{records r} V_BlockRefusal(refusal_highwater_r<=2B)
    Geo = V_BodyGeometry(b) + sum_body V_(u32,SpringKind)(unassessed_body)

Across geometry children total unassessed<=2n; in selected product d=0 these
children are empty, subject to adapter binding. A geometry clone is a distinct
Geo owner. Report vectors MOVE from verify_state; Report_p is not added on top
of the same resolution/r_hat/delta/e_rows/w/a_s/charge/w_plus allocations.
A Summary clone is distinct. Report/StopDecision do not survive in RetainedSolve.

Cache(c) is an inline GroupCache plus, for each occupied failed verification
slot, one V_BlockRefusal(r<=B) child. Failed shared slots are inline error/work/
stage payloads. A cache inside RetainedSolve or group-table tuple has its header
already charged there; the combination local cache header is on stack.
Do not multiply successful payloads by cache-header count.

Selected(j) owns X_RetainedSolve, Arc edges to P1 prep/group, cache children,
states backing and Z_p edges, Attempts, Geo, native Publication, radii, and Evidence:
- Publication: V_PublishedRow(Q) + V_(u32,Kind,u64)(4b); radii Box<[u64]>(Q).
- Evidence: stop summary<=4b; independent body-scales clone<=4b;
  Dof input-derived k; (QuantityId,u64) absolute<=Q; (QuantityId,Binary64Outcome)
  unpublishable<=Q; not_covered is unallocated in current finish; source identity
  byte clone; K4LED bytes from P1 ledger; K4RST bytes=22+(N+6m)*(9+8L_selected).
- Evidence also has resolution triples b, estimate/charge triples each<=2b,
  theta pairs b, certified-bound pairs<=b, optional floor triples b.
Attempt/geometry allocations are MOVED into Evidence. Native selected finish
also temporarily owns another Summary and cloned selected AttemptRecord with
its own summary/refusal children. Preserve that overlap through Box creation.
K4LED/K4RST growth, source clone, collection capacities and any box shrinking
are separately qualified; encoded length is not allocation capacity.

## 4. Finite active scratch roster (counts, not inherited byte coefficients)

Each row is a conservative local allocation multiset. Its named returned fields
are the SAME tokens as section 3 or another row when moved. The union/max rule
avoids duplicate physical charging. Distinct executions of a helper have distinct
tokens; concurrent parent/child allocations remain. Successful partial-prefix
and error unwinding are included up to actual drops. A sum of every row's local
upper bound for the active run is also finite and conservative, but deliberately
looser than the phase maximum. Fixed numerical arrays/temporaries go in K_stack.

| Phase / source functions | Owned backing families and bounded occupancy |
|---|---|
| shared formation/assembly | Partial S_p children; temporary residual MemberOperators<R>(m) below ceiling; assemble done bool(Z), optional terms Wide(L or R)(U), U<=78m+s+6d; temporary bounded full matrices [Wide<R>;144](m) where assemble_bounded is called. |
| factor/condition/pivot | Factor work Wide<L>(F); condition x,y,signs,z,alternating each F with solve helper overlap; BlockRatios outer Vec<usize>(B), children total F, est Wide(B) moved to S; pivot Tracker(F). |
| solve helper | scaled(F), solve_scaled x(F), solve_scaled out(F), final unscaled output(F). Sum of all four bounds is conservative; not four outputs retained after return. |
| ordinary solve/refinement | u(N), rhs(F), u_free(F); evaluated outer Vec(<=4), four child snapshots F; residual rows (bool,f64,Wide)(F); residual Tracker(F); correction(F), factor-solve helper, delta(F). No multiplication by number of loop iterations. |
| bounded fallback | abar_q(Z), temporary bounded matrices(m), one full u clone(N), ratio rows (ExactWideSum,ExactWideSum,f64)(F); up to4 stored tracker states plus active tracker, worsts<=4 inline GateRatio pairs and outer state/worst vectors; candidate ratios are fixed sum structs; chosen u_free clone overlaps prior u_free and all evaluated children. |
| recovery | q_all([Wide;6],m) and values(Wide,Q) become Recovered; end_actions([Wide;12],m), spring_action(Wide,s), directional_action([Wide;3],d), reaction(Option<Wide>,N). Test-only seeded u not in production profile. Support-group tail appends existing values; fixed local6, no group-specific numeric heap. |
| verification shared | Partial VS_p; bounded coefficients(L,m), uc_refused(Option<BoundRefusal>,B); wide members(W,m) and directionals(W,d) below ceiling, full-matrix formation temporary(m); block_of_row(u32,F), earlier u_pass a/c(each F) dominated by the existing four-buffer maximum below; original Uc c(F) retained through nl_pass at/bt/ct(each F), two block-max vectors u/n_l(each B), bounds(B). Returned failure refusals(B) overlap uc_refused until return. |
| formation_scale helper | output Option<Wide>(Q), q_all([Wide;6],m), actions([Wide;12],m), spring_e(Wide,s), directional_e([Wide;3],d), reaction_e(Option<Wide>,N); no hidden source-group array. |
| resolution helpers | top [Wide;2](b), resolution [f64;2](b), independent resolution_hat-check vector [f64;2](b); caller e_rows and w_abs remain. |
| verification pass | w_abs(N), terms_w/terms_p outer Vec headers(N each), children total tau at W/L; prescribed_nonzero/negative bool(N each); u_free,r_hat,sr_row,delta,sr2_row,sas_inf_row,sas_one_col,sau_row each Wide<L>(F); delta_full and w_s each N; recovered q/values; e_rows,w,a_s,charge,w_plus each Option<Wide>(Q); resolution(b); data bool(B), s_refused(B), start triples(B), shifts/results(B), report block/norm/theta/body arrays; failure-refusal(B). |
| contribution_products | one cloned operand Vec<Wide<W>> of length<=max(1,tau_max), overlapping caller terms; for bound C2 tau=0 the outer terms vectors still exist. |
| shifted profile/factor | ScaledProfile first(F), outer rows(F), children H, block_of_row(F); ShiftedFactor distinct first/rows clone(H), failed bool(B), shifted Option<Wide>(F); local factor work(F) drops before nl_pass. |
| shift schedule | results(B), in_flight usize(B) high-water despite clear, refused Option<BoundRefusal>(B), current and next triples(B each), sigma Option<Wide>(B), nl at/bt/ct(F each), block maxima(B); caller start/data/s_refused stay live. Previous factor drops before retry: not three factors live. |
| report/attempt transfer | Move report fields; transient remaining pass vectors stay until return; spent.refusals(B) coexists with record's Uc vector extended to<=2B and its moving old backing; Arc report + newly cloned Summary coexist. |
| stop rule | skip bool(Q), scales raw/coupled [Wide;4](b each), distinct rule hats [f64;2](b), optional separately allocated floor [f64;2](b), decision summaries(4b,2b,2b); TrackerSet keys<=8b, total offers<=Q+2Q_force, Q_force<=Q. Report/state owners remain. |
| native certificate/publication | canonical layout Vec<QuantityMeta>(Q) during validations; rounded candidate Vec<Binary64Outcome>(Q); publication s/scales [f64;4](b each), rows(Q), body_scales(4b), radii(Q). Source-current fixed CloneWork/certificate sums are stack, no heap registry. |
| selected finish | all selected output fields above; transient Summary, selected-record clone, report and decision, local cache AND cloned selected cache failure children; encoding and filter vector growth old backings. |

RV52-1 repair: rule::hats is a new Vec<[f64;2]> collected from report.resolution
(adaptive.rs:2284–2290), charged as V_rule_hats,[f64;2](b). It is distinct from
report resolution, every Summary clone and the optional floor. The earlier
resolution_hats result (verify.rs:794) is dropped at that standalone statement;
it supplies no live allocation credit to rule. During rule, hats overlaps skip,
coupled scales, the TrackerSet and accumulated summaries; at verification
precision1024 it also overlaps separately allocated phis/floor (2293–2306).
The hat closure reads hats in force/estimate/charge loops through2430; hats stays
in the rule closure's scope through summary production and drops when that
closure returns, including early exits. The report remains owned by its caller;
floor/summaries move into StopDecision, hats does not. The raw scales_at scratch
drops before hats construction; the table's coarse scales bound may include both
but that surplus is not used to omit hats.

At each stop-rule event e where hats is live, add its distinct token h to A(e):

    R_stop(e) = R_other(e) + V_rule_hats,[f64;2](b)
    M_stop(e) = R_other(e) + V_rule_hats,[f64;2](b)
                + Old_other(e) + Old_rule_hats(e)

Old_rule_hats(e) is the old backing still live only during a moving growth at the
actual collect site; its maximum capacity law remains unqualified. It is zero
at later events once that old backing is released. Hats stays in R_stop while
other buffers grow, so its live weight also overlaps their moving terms. Do not
replace this explicit mapping with an earlier check/floor buffer or unspecified
surplus. No allocation-capacity coefficient or measured-byte claim is added.

RV52's Uc domination mapping is explicit: u_pass has distinct a and c vectors,
whereas nl_pass overlaps retained c with at, bt and ct. Let Vmax_Wide<L>(F)
be the maximum bound across these actual sites, as allowed by section1. Then
max(V_a(F)+V_c(F), V_c(F)+V_at(F)+V_bt(F)+V_ct(F)) <=4 Vmax_Wide<L>(F).
Thus the earlier two-buffer phase is already covered by the row's four-buffer
coarse bound; a is not an alias of c and no fifth buffer is added. The distinct
block-max vectors u and n_l each contribute their existing B-vector term.
Site-qualified old-backing terms still follow section2; logical counts alone
are not a capacity proof. This is RV52's source mapping, not a new byte profile.

Tracker(O): lazy tuples (2 ExactWideSum,u64,u64) have length<=min(O,T),
table (u64,Evaluated)<=O, kept table<=O, and Sort_table(O) workspace.
Bound it by V_lazy(min(O,T))+2V_table(O)+Sort_table(O), plus inline header.
T is actual tracker_rows source/profile (default512), not a newly selected cap.
During collapse the taken lazy allocation remains with table; during prune
old table remains with kept, and shrink_to_fit can add old backing. Early work
failure drops the taken lazy on exit; table and all enclosing owners survive
until their actual scopes end. ExactWideSum/AttemptStop changed strides matter.
TrackerSet(K,O) may conservatively use K*Tracker(O) plus qualified BTreeMap(K)
and BTreeSet(K) nodes. A tighter distribution bound sums Tracker(o_k), sum o_k<=O,
and preserves the actual capacity-triggered collapse; do not substitute logical
length for the source's held capacity. No old 4304/40/etc byte constants transfer.

## 5. Batch, failure and combination transitions

Native schedule has at most4 solve records/states,3 verification passes and3
candidate comparisons/certificates: candidate failure advances c by1; failed
verification solve advances c by2; successful verifier becomes pending same
state/record at next c. A failed verify_state is terminal. Work loss adds early
terminal paths, not another precision. Earlier states/attempts stay live until
schedule return even when no longer the active candidate. Failed partial stage
allocations count while forming but are not fictitious stationary Arc payloads.

Group cache success occupies its first slot for that group; nonbudget failure
occupies an Err slot; budget failure is not cached. Missing slots may be tried
again by a later case/combination. Each physical retry has fresh transient owners.
Do not assume a cached numerical failure is empty or all retries share one token.
Successful case snapshots freeze only slots populated when selected. Later group
fills do not backfill earlier snapshots. All selected operands remain owned by
the grouped caller until its actual release; independent solve calls with equal
stiffness may have independent S/VS allocations.

For each real combination, P1 counts borrowed operand refs, identity checks,
first-source clone/exact combined ledger/prescribed pairs/factors/encoding and
new CasePrep. GroupCache::merged selects FIRST OCCUPIED slot in authored operand
order independently at each precision. Success clones Arc refs; Err verification
copies its vector once per selected slot (<=3), not once per operand. The local
cache can acquire new S/VS and failures. finish_selected clones that local cache
again. Original operands and snapshots are never mutated/backfilled. All new
states and output evidence are combination-owned; prep/group Arc edges share
only actual selected source/group owners. No combined rounded PrimitiveSource
or invented factor clone is added.

I37 actual solve_recorded still accepts selected RetainedSolve operands only,
uses first.group, has no full C2 origin inventory/Prepared operand API. C2's
selected-group choice (possibly not first), PreparedCaseSource operands, call/
source/group/build/run inventories, recorded case results and their index arrays
are named NONZERO-UNKNOWN additions, not implemented facts or zero-cost seams.
They must preserve this ownership graph after source binding. Pre-source refusal
has no core/cache import; preceding identity/preparation scratch still counts in
P1. In current code run_core can see an invalid meter after combination prep;
selected upstream pre-source guards must not be assumed already implemented.

CoreRun owns one ExecutionOutcome plus four inline WorkTotal fields in RunWork.
Selected moves its Box. Refused and Unresolved retain attempts and geometry.
Ordinary case into_legacy drops Refused attempts before pushing the CaseOutcome;
Unresolved keeps them. Combination mapping moves attempts into CombinationOutcome
but DROPS terminal geometry; Selected moves the same Box. RecordedCombination is
one enum owner, not a second copy of CoreRun/outcome. Its legacy adapter CLEARs
Refused attempts: child summary/refusal allocations drop, attempt backing stays
at old capacity in returned outcome. It is incorrect to cost that as V(0).
Public Clone derives can make additional deep copies; only actual M1-invoked
clones are included. Arbitrary downstream clones belong in P4's caller contract.

## 6. Boundary and next bounded work

Corrected source ownership roster, pending RV52 backcheck: numeric retained S/VS/Z/report families,
cache success aliases/failure copies, schedule attempts/states, partial build
children, terminal native custody, native SI publication/evidence and the local
scratch families above. This is conditional on independent review of this roster;
not every capacity/stack profile is qualified. I37 source binding confirms inline
work states, borrowed nonheap WorkStream/Snapshot, fixed CloneWork and real
terminal/combination custody. ROOT reports RV51 CLEAR on unchanged fdae294; any
later source repair requires affected type/lifetime rebinding.

MISSING after this narrow correction: source-qualified stride/alignment/Arc/Vec/Box/BTree/sort capacity and
moving laws for final compiler/target/features; exact stack bound; finalized C2
case/operand/origin buffer implementation; independent P3 derivation/backcheck.
P1 remains responsible for preparation census/refusal windows and all upstream
source/map/graph/order owners. The kernel table is not a free census/rebuild.

Future private facade/certificate addition must account native selected owners
still live with canonical safe projection, defensive identity/accessor metadata,
receipt/certificate draft/output bytes and formatting/encoding scratch; include
any ordinary-base plus W1-draft coexistence and count late validation buffers.
Current native SI certificate is INCLUDED above; a second private product-layer
scratch plan is NOT silently zero. P4 must add each qualified captured caller's
ordinary inputs/results, headless clone/finalization/export/serialization window,
output publication lifetime and request-owned release/refusal plan. Nonqualified
callers keep ordinary behavior and decline W1. PP-only is not whole invocation.
No generic allocator/parser/transaction framework is proposed.

PROPOSAL
- Evidence: frozen candidate allocation/terminal sites in OWNER_LEDGER.md; P1 and
  selected C2 distinguish actual sources/copies; constructive controls below.
- Change: next bind the C2 owner/origin additions to a frozen implementation and
  qualify one final target/build profile for these exact types and container
  sites, then evaluate the owner-union phase maximum before M1 admission.
- Why: this is the smallest step that supplies missing weights and edges without
  changing grouped order/cache reuse or relying on old single-call coefficients.
- Risk: a source/layout/feature repair can invalidate a term; capacity/RSS/stack
  and caller suffix are separate warrants. No numeric allowance is selected.
- Status: PROPOSED; fresh independent P3 review before reliance/implementation.

NEEDS_HUMAN_RULING: none added; owner-reserved memory/deployment/product choices
remain reserved. DEPENDENCY_NOTES: RV52 repair backcheck, any changed-source rebind, C2 binding,
P1, private facade and P4.
Independent reviewer should rederive failed-cache copy generations; clear/drop
capacity; report moves; Uc c with nl_pass; shift retry drops; geometry children;
conditional zero-prescription terms; finite4/3 schedule; and capacity/moving laws.

