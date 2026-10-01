# source_05 — coherent finite kernel owner formula

Status: source consolidation, not an accepted numeric E_max. TASK I21; parent
WORKING_ITEMS recovery manager; no delegation. Every application term below is
**A1-sensitive**. The basis is frozen A1 40129a225d73860ac2a53da9a2fa73869df668f3.
Read-only diff against layout_04's cb13fcf3ea560c0d07ad9ac02dac78e9d2e06e00
is empty throughout retained/. The 1.97.1 actual-layout facts therefore bind
these unchanged retained types provisionally; final A1 reconciliation is still
required. source_01, source_02, source_03 and layout_04 seals all verify.

P=projects/chirality-piping; FK=P/core/solver/frame_kernel;
K=FK/src/structural/retained. A=K/adaptive.rs. References below are source lines,
not measurements. Earlier detailed formulas remain in source_01/FORMULAS.md;
this packet replaces their uninstantiated scalar strides and separates their
phase overlaps. It does not silently close their caller/consumer gaps.

## One formula, two allocation metrics

Use an allocation-identity ledger. A moved Vec/Box retains its identity;
Arc clones share a payload; fresh deep clones have new identities. For each
reachable phase phi in the finite schedule define:

    R_phi = sum(requested bytes of each distinct live allocation identity)
    M_phi <= R_phi + largest current old buffer at a possible growing realloc
    Kernel_requested = max_phi R_phi
    Kernel_move      = max_phi M_phi

The R term includes the new buffer at a grow. Its old buffer is the additional
move-counter charge, not another persistent owner. Distinct old/new Vecs at a
rebuild/assignment are both real R owners; count both. Only one allocator
operation is active at a time in this sequential lane. Do not multiply the
whole phase by 1.5 or sum all historical realloc surcharges. Shrinking realloc
does not add an old+new move charge in the existing observers.

All operators are requested bytes, excluding allocator metadata, RSS, stack
frames and static storage. Arithmetic must be checked; overflow is a bound
failure, never a smaller answer. These equations are conditional while the
explicit parameters/cells below remain unbound.

For element stride s: E_s(k)=s*k; mu=8 for s=1, 4 for 2<=s<=1024,
1 for s>1024. P_s(0)=0; P_s(k)=max(mu,next_power_of_two(k)) for k>0.
G_s(k)=s*P_s(k), for fresh push and borrowed lower-zero/fallible collections.
A_s(k)=s*max(mu,2k) for k>0 (zero otherwise), a safe fresh general
extend/resize upper. A preexisting buffer retains max(c0,mu,2k) slots.
Actual RawVec growth is c'=max(2c,len+additional,mu); an active grow's old
request is <=new/2. Exact clone copies len, not source capacity.
Use source_02's pinned implementation contracts, not a Rust API growth promise.
Sort_s(k)<=s*max(k,48) for k>=2, otherwise zero; it is a separate transient.

For retained tree nodes, keep TI(k,K,V) symbolic: at nonzero insertion
high-water k it is [1+floor((k-1)/5)]*max(S_leaf,S_internal), plus children.
A populated then empty tree may retain a leaf. Actual LeafNode<K,V> is
repr(Rust) and unmeasured. InternalNode is derived from that actual leaf and
twelve measured 8-byte pointers by the source_02 repr(C) alignment formula.
No mirrored leaf or per-key allowance is substituted.

## Shape and identities

Keep N,m,s,d,r,l,t,u,B,b,n,f,z,h: nodes, members, axis springs, directional
springs, constraints, load terms, stations, support groups, bodies, free
blocks, DOFs, free DOFs, pattern entries and skyline slots. Keep per-row
pre-dedup p_g, free-neighbor d_a, component f_c, body N_c, loaded distinct
DOFs v, ledger magnitude limbs J, and actual input string/child capacities.
U=78m+s+6d; sum p_g=144m+s+9d; q=7N+12m+6t+s+3d+r+2u.
J<=68v, v<=min(l,n); do not assume one limb for RF-CANCEL.
Force/moment row upper is F=q-7N. The original 33 H fixtures/schedule and
existing VR cases remain unchanged. VR s,d,B,b are real descriptors.
Zero prescribed values are justified by the current H/VR adapters only;
generic combination terms keep per-DOF nonzero term counts tau_g.

The schedule has at most four solve precisions (128,256,512,1024) and three
candidate/verification decisions. A failed build is not a successful cached
payload. Define K_phi as the **actual union** of source/group/prep/scaffolding,
previous successful Shared, Solved and VerifyShared payloads, attempt children
and cached failure children live at phi. Current construction replaces the
corresponding identity in K_phi. Reports are local; do not add four reports to
the cache. For conservative branch envelopes below, a complete result can be
counted early while being constructed; that explicit padding is not a claim
that all its fields are physically live yet. Helpers include their returned
buffer: replace that buffer's ordinary retained term during construction,
rather than adding it twice.

## Instantiated persistent numeric owners

All measured facts are in FACTS.json, preserved from layout_04. For limb
width L=4,8,16 use w=48,80,144 bytes. Actual Option<Wide<L>> has the same w.

| Actual type / owner | L=4 | L=8 | L=16 |
|---|---:|---:|---:|
| MemberOperators | 7888 | 13136 | 23632 |
| BoundedCoefficients | 248 | 408 | 728 |
| DirectionalBlock | 440 | 728 | 1304 |
| PivotScreen | 104 | 168 | 296 |
| BlockBound | 208 | 336 | 592 |
| ShiftStart tuple | 64 | 96 | 160 |
| ShiftResult tuple | 272 | 432 | 752 |
| BlockCertificate | 592 | 944 | 1648 |
| BlockNorms | 336 | 560 | 1008 |
| BodyReport | 624 | 1040 | 1872 |

Arc requests are **derived** using authenticated repr(C,align(2)) ArcInner,
two measured 8-byte atomics, 16-byte data offset and measured payloads.
CasePrep=432, GroupPrep=368, Shared=720, Solved=104,
VerificationReport=344. VerifyShared(L,W)=(4,8),(8,16),(16,16) is
560,592,656 respectively. These are request sizes, not directly measured
private ArcInner layouts. Children are separate.

Let M_L,D_L,C_L,Pivot_L denote the table strides.

    Factor_L = h*w_L + f*(48+Pivot_L)
    Shared_LR = 720 + m*M_L + d*D_L + z*(w_L+w_R)
                + m*C_R + d*D_R + Factor_L + [p>=256]*b*w_L
    Solved_L = 104 + (n+6m+q)*w_L
    VerifyShared_LW = Arc_LW + z*w_L + 144m*w_W + 9d*w_W
                     + b*BlockBound_L
    Report_L = 344 + 5q*w_L + (f+P_w(f))*w_L
               + b*(BlockCertificate_L+BlockNorms_L+w_L)
               + B*(BodyReport_L+16)

Report block coefficients are 976/1584/2800; body coefficients
640/1056/1888. The refusal field makes BlockBound=4w+16, not 4w+8.
Shared instantiations are (L,R)=(4,4),(4,8),(8,16),(16,16).
A:1179-1387, factor.rs:520-799, verify.rs:441-501,1013-1224.

## K01–K09: source, group and prep sequence

| IDs / phase | Owner formula and actual lifetime |
|---|---|
| K01 source constructor | Input array capacities use strides 24N,88m,24s,48d,16r,40l,16t,64u; add actual load-id String children and support-id children (4 bytes/id), prescribed 16n and body index 4N. Here each count means capacity where supplied by caller. Above completed-source padding, take max of one input sort scratch, 3*2144 ExactWideSum Vec, and union-find 8N+4N. source.rs:352-590. Caller adapter/source formation belongs to the observed-window composition even though it precedes solve_case. |
| K03–K04 group geometry | prepare_group runs geometry before pattern/order/blocks. Output is 24B plus NotAssessed child buffers G_sigmaNS(k_c), sigmaNS=sizeof((u32,SpringKind)), k_c<=2N_c. One active body has G_4(N_c)+TI(N_c,u32,usize)+24N_c+G_8(r_c+s_c+6N_c), directional filter capacity followed by <=3 pushes, relative coordinates G_24(N_c), G_48(g)+48g SVD clone, candidate G_48(2N_c+5), G_192(N_c) motion entries and <=N_c*(3G_8(10)+3G_8(1)) expansion children. Add current 3G_8(2) delta and one next expansion G_8(10); successful-witness branch adds G_48(N_c), 80-byte scalar clone and G_8(11) addition scratch. Different bodies are sequential; previous NotAssessed outputs survive. factor.rs:130-219 and FK/src/rigid_body.rs:45-247. sigmaNS/tree/queue cells remain explicit. |
| K05 pattern | Retain rowstarts 8(n+1), columns A_8(z), transpose 8z. Build has positions G_16(U), outer row headers 24n and sum G_8(p_g). All raw row buffers can be padded live while columns form; consumed backing persists. Transpose follows raw-row drop. assemble.rs:550-583; sparse.rs:62-159. |
| K06 tagged contributions | Above pattern and positions, G_16(U) tagged backing + starts 8(z+1) + next 8(z+1) + items 8U. Tagged backing stays alive while consumed into items; afterward positions/tagged/next drop. assemble.rs:584-646. |
| K07 ordering | Returned free G_8(f), position 8n, order/rank/first 24f. During graph formation add 24f+sum G_8(d_a). RCM adds 24f+sum G_8(2d_a)+17f (degrees, order, visited), then max(one sort, BFS frontiers/marked). Preserve four frontier/current/next/saved identities and ring-queue Qdeque explicitly until its capacity contract is bound. No hidden constant absorbs it. factor.rs:359-401 and source_01 K07. |
| K08 free blocks | Returned 4f+G_24(b)+sum G_8(f_c)+4b; active DFS stack G_8(f_c), with earlier components retained. bound.rs:84-121. |
| K09 scaffolding order | Outcome Vec capacity1 requests80. STF key is built before prepare_group. GroupEntry is pushed only after prepare_group returns; fresh one-entry G_1464(1)=1464. GroupPrep Arc368 then remains throughout CasePrep creation. Group success geometry remains and schedule clones geometry (24B plus exact-length NotAssessed children). Attempts G_832(a), states G_16(vstates), a,vstates<=4. Header strides include inline enums/work counters; their Vec children remain separate. A:4310-4409. |
| K02 case prep after group | Original source remains beside exact-length source clone. Ledger building G_1104(v) overlaps net result 48v+v+8J; it drops before prescribed lists. Single-case prescribed entries 32r+16r (generic:16 times total terms). SRC identity uses encoded-byte capacity, layout G_20(q), extents 8B. One extent transient G_4(N_c)+24N_c. ArcCasePrep432 contains the shallow source/header fields, not child buffers. A:876-947; ledger.rs:103-129. |

Source clone children use exact lengths, not caller capacities. The encoded
payload lengths are source-backed: SRC=38+24N+84m+17s+41d+13r+17l+L_ids+
16t+22u+4*support_id_count; STF=26+24N+84m+17s+41d+5r;
LED=10+18v+8J; RST=22+(n+6m)*(9+8L). Use the actual RawVec chunk recurrence
from initial six bytes, or conservative max(8,2*payload_length) retained,
plus one old-buffer move surcharge. Do not assert all SRC string chunks<=8.
Source IDs and caller arrays still require concrete input capacity binding.

Failure edges: geometry refusal returns no partial success geometry; a
structure failure clones completed geometry into cached Err while the original
is briefly live. Copying cached Err into an outcome deep-clones those child
Vecs. This is a real separate owner even though the enum/header is inline.

## K10–K14: shared build and own solve

Above K_phi, pad Shared_LR during construction and add the **maximum**, not
sum, of: assembly flags z bytes; residual-width temporary members m*M_R plus
z flags; factor work f*w_L; condition scratch 5f*w_L+24b+8f;
pivot-tracker phase. D_R already belongs to completed Shared, so do not
charge another retained D_R for the residual formation branch.
Condition BlockRatios est storage moves into Shared. A:1243-1387.

Let C=P_w(f). A factor.solve helper above its caller RHS has retained peak
(2C+f)*w and move peak <=(2.5C+f)*w. It returns C*w.
This follows borrowed fallible collection for scaled and returned Vecs;
solve_scaled x/out and fresh evaluated clones stay exact f*w.
factor.rs:636-676; source_02 capacity derivation.

During solve_case_at, completed Solved padding includes u and recovered result.
RHS f*w, u_free C*w and evaluated state (96+4f*w) are separate.
The following are alternatives above the appropriate prefix:

- Initial solve: RHS + factor.solve helper, replacing the future u_free term.
- Residual pass: RHS+u_free+evaluated + residual rows f*(w+16) +
  one residual-tracker peak.
- Correction: same residual rows and held tracker, correction RHS f*w and
  factor.solve helper returning a separate delta. Existing u_free is updated
  in place while delta remains; this correction is not a u_free replacement.
- Fallback: same caller residual rows/held tracker, z*w_R and
  max(144m*w_R member blocks, fallback states). Fallback states include n*w_L
  u clone, one G_4304(f) row list, up to four optional worst-gate/tracker slots
  (4288+96 each), previous fallback trackers plus one active tracker.
  Row-list growth and tracker.offer are sequential; take their maximum.
- Chosen fallback return: bounded_fallback scratch has dropped; exact f*w
  clone of evaluated[k] overlaps the old C*w u_free at assignment, while
  caller residual rows/tracker/evaluated buffers still remain.
- Recovery: residual list and residual tracker have dropped; RHS, u_free and
  evaluated state remain. Beyond the returned (q+6m)*w already in Solved,
  helper adds (12m+s+3d+n)*w.

A:1701-1825, recover.rs:270-475. Four fallback row lists are not simultaneous.
Both old/new u_free are explicit during replacement; retained successful
states from earlier precisions remain through later attempts.

## K15–K18: verification shared, pass, shift, report

VerifyShared construction above complete-result padding adds m*C_L+16b
bounded-coefficient/refusal slots and max(144m*w_L bounded member blocks,
temporary m*M_W+d*D_W widened operators when used, Uc).
Widened operators drop before Uc. Uc peaks at 4f*w_L+4f row map;
later 2f*w_L+2b*w_L while BlockBound output forms. This is why adding
all Uc, widening and Nl arrays as one live peak is wrong. verify.rs:441-501.

A failed VerifyShared build unwinds success buffers. Refusal filtering has
16b slot storage overlapping G_24(b); cached error clone adds exact24*k while
the original capacity persists. The successful VerifyShared term is absent
on that error trace.

Pass locals, excluding the early three row vectors, have:

    Pass_L = 3n*w + (7f+C)*w + (q+6m)*w + 50n + 16B + PrescribedChildren
    PrescribedChildren = sum_g [G_w(tau_g)+G_wW(tau_g)]

The two prescribed outer header arrays and two bool arrays are 50n even for
the original zero-prescription adapters. The three full vectors, free vectors,
delta and recovered data stay live through report construction. During
delta solve, replace C*w by the factor.solve construction envelope.
Recovery helper adds (12m+s+3d+n)*w; formation_scale helper above its returned
q*w adds (18m+s+3d+n)*w. contribution_products has one sequential operand Vec,
at most max(1,max tau_g)*w_W exact clone slots, not z copies.
verify.rs:660-670,761-967.

Shift has Pass_L + early 3q*w. For k<=b blocks needing work:

    profile = h*w+36f
    shifted factor = h*w+(32+w)*f+b
    controls = b+16b+G_start(k)+E_result(k)+A_8(k)+16b+G_start(k)+b*w

The two G_start identities are initial start and current; use actual measured
start/result strides. Controls distinguish data, s_refused, results,
in_flight, refused and sigma. Next-buffer construction adds G_start(k) while
consumed current backing remains. Add max(f*w factor work,3f*w Nl,
f*w+b*w+G_start(k) ct/maxima/next). Old shifted factor drops before retry.
If k=0 no factor/profile is built. bound.rs:1190-1378 and verify.rs:967-1012.

After shift, profile/factor/scheduler controls die. Caller data, s_refused,
start and returned shifts remain until pass ends. Report construction keeps
these plus Pass_L and refusal output G_24(b); count Report's five row arrays
once (three were early arrays), not Report+five more. After pass returns,
report buffers move; pass locals/controls drop. Caller extends attempt refusals
to <=2b using G_24(2b), overlapping the spent refusal backing, then adds
summary children 16B+8B+16B=40B. Reports are Arc344 each, success summary
clones deep-copy only those child buffers. verify.rs:1013-1224; A verification
call and source_03 owner table.

## K19: tracker/R7 phases

A tracker has lazy row stride4304 (align16) and table-entry stride40. Use
offer high-water R_j for table capacity <=max(4,2R_j), not current key count
or next_power_of_two after shrink. Lazy threshold is 512 rows. Model one
current tracker by the maximum of these alternatives, plus other held trackers:

1. lazy growth, with table retained;
2. taken lazy backing plus extending table;
3. table plus Sort_40(table_len), after taken lazy drops;
4. old table plus growing kept Vec;
5. old table plus shrunk kept before assignment drops old table.

Realloc move adds only the active old request. Independent kept/old tables
are already two real retained owners. Standalone trackers have no map nodes.
TrackerSet adds the actual BTreeMap key=(RuleTest,u32,Kind), value=
BoundedExtremeTracker and BTreeSet key of the same actual tuple, value=
private SetValZST. Both private LeafNode specializations remain parameters.
A:655-735,776-827.

Call-site offer high-waters bind the R_j parameters, rather than leaving a
hidden per-tracker population assumption:

| Site | Per-instance high-water and lifetime | Frozen source |
|---|---|---|
| K10 pivot margin | R_pivot<=f, one offer per factor screen with nonzero denominator; one fresh tracker per shared build | A:1383-1395; factor.rs:561-588 has one screen per free factor row |
| K13 residual | R_residual<=f, one offer per free row; new tracker on each refinement iteration, at most four iterations, previous iteration drops before next | A:1467-1508,1758-1766,1793,1835-1836 |
| K13 fallback | R_fallback,j<=f; at most e<=4 evaluated states. Each gets a fresh tracker; prior eligible trackers remain through later states and final selection. The caller's last residual tracker is an additional held instance | A:1613-1697,1764,1793-1804 |
| K19 R7 | R_j<=q for a disagreement key; <=F for each estimate/charge key; sum_j R_j<=q+2F, keys<=8B. One fresh set per rule call; prior decision's trackers do not survive | A:2136-2277 |

Thus each pivot/residual/fallback table uses A_40(f) as the retained
high-water upper (zero when f=0), with the prune/rebuild alternatives above.
For R7 one may use 40*sum_j max(4,2R_j) over offered keys, or the coarser
40*[2(q+2F)+32B]; preserve old/kept table overlap during prune. No numerical
outcome or observed capacity is needed for these call-site bounds.

Normal-production T=512 and G=4096 are source constants (A:583-596).
Consequently one standalone lazy backing is <=512 slots; its growing-realloc
move is <=768 slots. A TrackerSet starts an offer with total retained lazy
capacity <=4096; one noncollapsing growth adds at most256, giving <=4352
retained slots before the global collapse and <=4608 slots at a lazy grow's
move sample. Residual plus four fallback trackers give <=2560 retained /2816
lazy-move slots. Multiply these slot counts by the actual4304 stride.
These are lazy-buffer coefficients only: table arrays, their alternative
grow/sort/prune phases, tree nodes and other owners are additional. Do not
add a lazy move surcharge to a simultaneous table grow; those calls are
sequential. source_02's old coarse768/4608/2816 move coefficients are thereby
preserved with their retained/move distinction made explicit.

R7 source A:2040-2301 closes the previously coarse stop-rule term. It retains
skip q bytes; scale creation peaks at raw+coupled 8B*w_verification, then retains
4B*w_verification. It has exact hats16B; the 1024 verification floor16B escapes.
Tracker offers total <=q+2F, keys<=8B: disagreement<=4B,
estimate<=2B, charge<=2B. Thus use actual B, not a fixed one-body key count.
The three final decision arrays have retained bound:

    Decision = G_16(4B)+2G_16(2B)+[verification=1024]*16B

During tracker drain, remaining tree/trackers overlap growing decision arrays.
A safe union retains the full held tracker set plus completed Decision, with
one active summary realloc surcharge; this is deliberate padding. Earlier
tracker-prune peak and later decision-drain peak are alternatives.
After closure return, skip/scales/hats/trackers drop; only Decision survives.
ExactWideSum arithmetic and meter/counter objects are stack/static here, not
new tracker heaps. The same rule applies to H/comparison/RU arithmetic.

## K20 and A1: canonical shape, certificate, retry, finish

For a candidate pair use actual retained prefix K_phi, current Report and
Decision. These are sequential branches:

| Phase | Additional retained owner union; move adds only its active old buffer |
|---|---|
| Before R7 shape | G_20(q); Decision not yet alive |
| Certificate shape | G_20(q); Decision now alive |
| Publication draft | 24q provisional Binary64Outcome +64B raw/coupled f64 scales + max(G_64(q),G_64(q)+A_16(4B) body-scale build) |
| H / RU / accepted certificate | G_64(q)+A_16(4B)+8q radius allocation |
| Accepted finish | Same publication/radius moved, plus finish evidence below; no new provisional24q values |
| Rejected/partial certificate | Drop partial publication/scales/radius before retry; retain only scalar rejection/error, caches/states and attempt owners |
| Terminal refusal | Keep terminal attempts/geometry; failed success payloads and certificate drafts are not retained |

The row collect is borrowed Result<Vec<PublishedRow>> and has capacity P_64(q),
even when every row succeeds. Its actual stride is64. Radius is exact8q from
with_capacity(q) through into_boxed_slice; no second radius array appears.
Values and raw/coupled scale temporaries drop before radius/H arithmetic.
Canonical layout is called twice but those two Vecs never overlap each other.
A:3080-3370,3948-4135; source_03/OWNER_DELTA.md.

Finish A:4184-4306 has report.summary40B, selected AttemptRecord deep child
clone (another summary40B if present, plus exact24*refusal_len), SRC identity
exact-length clone, LED/RST encoded capacity, stop/estimate/charge clones
<=128B, publication body-scale clone<=64B, constraints input-Dof array,
filtered G_24(a) absolute rows and G_40(u') unpublishable rows (a+u'<=q),
and resolution/theta/certified/floor tuple arrays. The latter tuple strides
are precise residual layout cells below. geometry moves, attempts move;
source/ledger/state encodings are new. The outer selected Box requests1976.
Equivalently the additional finish upper, beyond K_phi+Report+Decision and
the existing certified publication/radius, is

    Finish = 1976 + 40B + SelectedRecordChildren + Encodings
             + 128B + 64B + r*sigmaDof + G_24(a) + G_40(u')
             + B*(sigmaResolution+sigmaTheta) + G_sigmaCertified(B)
             + [floor present]*B*sigmaResolution
    SelectedRecordChildren <= 40B + 24*selected_refusal_len

Encodings are exact SRC clone plus LED/RST growth envelopes above. Counts
use the safe full B or 4B ceilings; real cloned lengths may be smaller.
Report and Decision remain through finish and drop before the result reaches
the consumer. Group/source call scaffolding dies on return; selected prep,
group, success cache Arcs, states, evidence, publication and radius survive.

Success cache clones are shallow Arcs. Generic cached error slots may
deep-clone refusal Vecs; parameterize those where an existing shared cache
can hold them. They are zero for a fresh one-case cache only with its
terminal-on-verification-error control-flow justification, not for arbitrary
multi-case/combination caches.

A selected deep Clone adds exact64q rows and8q radius, exact evidence/attempt/
state-vector children, and outer1976 only when cloning Box<RetainedSolve>.
It shares success Arc payloads. Original P(q) rows remain alongside exact-q
clone. H saves attempts, not selected radius; repeats do not accumulate radius.
Combination keeps the distinct operand selected-owner union (including
radii), identity/prep-reference buffers and new CasePrep; success cache/group
Arcs share. Then it runs the same finite phases. The original single-case
comparison does not admit an arbitrary operand/clone count.

## Closed and unclosed cells; caller boundary

Closed source lifetime/capacity cells: K01–K09 ordering and failure clones;
K10–K14 numeric payloads and residual/correction/fallback/recovery alternatives;
K15 Uc-versus-widening overlap; K16 operand scratch and delta capacity;
K17 shift controls/retry lifetimes; K18 report/summary/refusal handoff;
K19 R7 key/offer/decision-drain ownership; K20 certificate/drop/selected clone.
“Closed” here means the displayed owner/lifetime equation, not a numeric
full-bound proof; tree/queue/input parameters still affect earlier rows.

Exact remaining finite cells:

| Cell | Required fact / conservative alternative |
|---|---|
| T1 actual private tree leaves | LeafNode<u32,usize> for geometry; LeafNode<(RuleTest,u32,Kind),BoundedExtremeTracker>; LeafNode<(RuleTest,u32,Kind),SetValZST>. Need actual leaf size/alignment, not mirror structs. Internal repr(C) derivation then uses measured pointer facts. Until available keep TI parameters; no observed peak replaces them. |
| T2 omitted actual nominal/tuple layouts | sizeof/alignof((u32,SpringKind)) NotAssessed children; Dof; (u32,u64,u64) resolution/floor; (u32,f64) theta; (u32,u64) certified_bound. These are not measured by layout_04's differently typed tuple labels. Keep sigma parameters in G/E, including their capacity minimum class. No guessed Rust tuple layout or substitution by a similarly shaped measured tuple. |
| C1 finite capacity bindings | RCM VecDeque capacity/high-water Qdeque; actual caller-supplied source array/String capacities and support-id children. Existing contracts cover Vec but do not prove ring-queue growth. Use explicit actual-capacity descriptors or a separately justified source upper; do not assume empty/one body. |
| C2 descriptor coverage | Bind concrete adjacency/component/body/ledger/source-string descriptors for all original shapes/cases, with input-based uppers where desired. Structural h<=f(f+1)/2 is available without a solve. No spring/body/domain zeroing. |
| O1 finite owner review | Independent check of the identity union, tracker high-water recurrence, preparation failure edges and finish overlap at final A1. All above are provisional source derivations, not an implemented estimator or mechanically checked theorem. |
| O2 final A1 | Any change to expression/scopes/collection/enum/certificate flow reopens affected terms, even if the broad algorithm is unchanged. Layout reuse is justified only by current frozen retained-byte equality. |
| W1 original comparison windows | H caller model/args/frames/saved attempts/runtime and in-stage refusal/error formatting; finite VR parse/control/comparison/JSON/RCM/parity consumer terms from source_01. Private consumer tree nodes/runtime/format/serde contracts remain open. This packet does not expand to a whole-process library audit. |

H's staged metric and VR's global summary remain distinct. H must include
in-stage formatting before Observer::end. Metric-design nuance: the outer H
prefix read at main.rs:965-966 follows emission of the completed stage line,
so the prefix envelope includes that emission; prefix_line at967 follows
that sample. Inner repeat samples still precede stage-line emission. Source
prelaunch/admission remains required; measured bytes do not replace it.
No full E_max, admission replay, fixture narrowing, new observer or runtime
claim is accepted here.
