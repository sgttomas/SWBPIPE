# I21 source_01 — finite formulas and owner map

This additive continuation refines U3–U7 of CHECKPOINT_0. It is source analysis,
not a numeric E_max or full-bound approval. Source basis remains
`d01ad98a754698631f927709d08284c272de85e8`. All K/FK/H/VR/SD aliases and the
original finite domain are as in CHECKPOINT_0. SD=P/core/solver/sparse_direct.
Current source reads, not the interrupted response author's declarations,
control the findings below.

## Operators and the unresolved library boundary

For an allocation site, let V(kind,k,T) be its requested retained buffer bytes,
and VM(kind,k,T) its largest old+new request during growth. 'kind' distinguishes
reserved/exact clone, fresh pushes, fallible collection, filtered collection,
resize/extend, and reuse of an owned input. Child heaps are always separate.
E(k,T)=k*sizeof(T) names the proposed exact request for a reserved/clone site.
Trees use TI(k,K,V) for insertion high-water and TB(k,K,V) for bulk construction,
including its input-vector/sort phase. Arc(T) includes the actual allocation
header and padding once; cloning an Arc does not copy its payload.

The response's pinned Rust-source candidate rules are:
reserved/TrustedLen/borrowed clone => E; push P=max(min(T),next_pow2(k));
general fresh collection/resize upper max(min(T),2k); realloc old+new;
stable-sort scratch max(k,48)*sizeof(T); insertion nodes
1+floor((k-1)/5), with an empty-root high-water exception; bulk construction
requires Vec+sort+tree nodes. They are **conditional library assertions**, not
newly qualified facts. The installed 1.97.1 toolchain's expected rust-src
paths were absent. No installation or source recovery was attempted.

Consequently every V/VM/TI/TB/Arc/Fmt below has an explicit binding obligation
in LAYOUT_AND_CAPACITY_GAPS.md. Merely knowing a vector's length does not close
its bytes. Safe *source-level* alternatives available now are:

- Count complete persistent payload while it is partly constructed.
- Permit both input and output buffers when owned-iterator reuse is uncertain.
- Keep old and replacement buffers together at assignment, including nested
  children; use a phase max where callees return before successors start.
- Preserve generic capacities as parameters rather than assuming exact length.
- Use arithmetic skyline f(f+1)/2 as a structural upper without allocating it.
- Retain every branch owner under a finite source-count envelope. A measurement
  cannot instantiate an omitted owner or certify a library growth contract.

These alternatives reduce dependence on optimization and path outcomes, but do
not establish numeric bounds independently of allocator/library binding.

## Shape descriptor; no zeroing of existing VR cases

Use N,m,s,d,r,l,t,u,B,b,n,f,z,h for nodes, members, axis springs, directional
springs, constraints, load terms, stations, support groups, bodies, free blocks,
DOFs, free DOFs, pattern entries and retained structural skyline.
Let p_g be the pre-dedup contribution count of global row g, d_a the structural
free-neighbor count, f_c block sizes, N_c body sizes, and L_ids total load-id bytes.
These are input/connectivity descriptors, not outputs inferred from a solve.

Current source gives:

- Upper contribution/tagged count U=78m+s+6d; sum_g p_g=144m+s+9d.
  K/assemble.rs:548-638 and FK/structural/sparse.rs:58-76.
- Publication rows q=7N+12m+6t+s+3d+r+2u
  (K/recover.rs:270-475; matching layout still requires identity validation).
- K4SRC payload
  38+24N+84m+17s+41d+13r+17l+L_ids+16t+22u+4*support_member_id_count,
  where the last count sums axis- and directional-spring ids in support groups.
  K/source.rs:652-744.
- K4STF payload=26+24N+84m+17s+41d+5r.
- K4LED payload=10+18v+8J, with v distinct loaded DOFs and J total magnitude
  limbs; v<=min(l,n), J<=68v (K/ledger.rs:32-63,103-129,218-232;
  FK/exact_sum.rs:19). One-limb nets require a separately verified unique
  single-load premise; they are not a valid default for RF-CANCEL.
- K4RST payload=22+(n+6m)*(9+8L), K/recover.rs:495-520.
- All named H adapters create no springs/directionals/supports and zero
  prescribed values (H/adapter.rs:39-85); VR source_parts distinguishes axis
  and directional springs and sets zero prescriptions (VR/cases.rs:470-533).
  The descriptor must preserve those actual counts and all actual bodies.

The 193 factored CI cases still call VR estimate in tests/scale.rs. Their
numerical results are not rerun here. The shared API must accept real s,d,B,b,
load multiplicity and geometry child data; no fixture exclusion or fabricated
zero is proposed. Combinations/multiple simultaneous solve_cases sources remain
outside the original one-case observation schedule; this is not a new restriction.

## Kernel formula and capacity table

Every row below is **A1-sensitive (A)**: final retained source can change the
owner, count, type, lifetime, result grammar or path reachability. Use a set of
named allocation identities so a payload moved into a result is counted once.
A complete phase envelope may overcount a future owner explicitly; no omitted
owner is justified by slack in another term.

| ID | Formula / live owner union | Current source and remaining binding |
|---|---|---|
| K01 Source constructor | Input adapter arrays+string children + E(n,Option<f64>)+E(N,u32), with max(sort scratch per sorted input, E(3,ExactWideSum), E(N,usize)+E(N,u32)) above completed source | source.rs:352-590; adapter capacities differ between H's map collects and VR pushes. Chord and sorting precede body union-find. |
| K02 Source clone/case | Exact-length source-array and string clones; ledger V(insert,v,LedgerBuild)+E(v,LedgerEntry)+E(v,bool)+E(J,u64); prescribed E(r,PrescribedPair)+r*E(1,(f64,f64)); identity encoding V(encode,Lsrc,u8); V(push,q,QuantityMeta); E(B,f64) extents | adaptive.rs:876-944; ledger.rs:103-129. Ledger build overlaps netted result. Extents transient additionally V(filter,Nc,u32)+E(Nc,[f64;3]) for one body. |
| K03 Geometry | Across prior bodies retain E(B,BodyGeometry)+all NotAssessed child tuples (at most 2N). Active body: V(filter,Nc,u32)+TI(Nc,u32,usize)+E(Nc,[f64;3])+V(push,r_c+s_c+6Nc,usize)+V(push,2Nc,(u32,SpringKind)) | factor.rs:130-219. Add one directional Vec whose starting filtered capacity can be followed by <=3 axis pushes. Ground dedup does not shrink. No B=1 assumption. |
| K04 Geometry witness | K03+V(push,Nc,[f64;3])+V(push,g,[f64;6])+E(g,[f64;6])+V(grow-from-one,2Nc+5,[f64;6])+V(push,Nc,[Expansion;6])+Nc*[3V(push,10,f64)+3V(push,1,f64)] | rigid_body.rs:45-247. Add current delta=3V(push,2,f64), one active next buffer VM(push,10,f64). To cover successful witnesses for non-H cases, also V(push,Nc,[f64;6])+E(10,f64)+VM(push,11,f64) for exact_scalar clone/add. No reliance on SVD success or fixed-root exclusion. |
| K05 Pattern | Positions VM(push,U,(usize,usize)); E(n,Vec<usize>)+sum_g VM(push,p_g,usize); compressed E(n+1,usize)+V(extend,z,usize)+E(z,usize) | assemble.rs:550-583; sparse.rs:62-159. Conservative sum permits all row buffers while columns build, including outer row headers until IntoIter ends. |
| K06 Tagged build | Complete pattern+positions+VM(push,U,(usize,Contribution))+2E(z+1,usize)+E(U,Contribution) | assemble.rs:584-646. Consumed tagged backing remains until loop ends; starts and next are both live. |
| K07 Ordering | V(filter,f,usize)+E(n,usize)+E(f,Vec<usize>)+sum_a V(filter,d_a,usize), then max(RCM, E(f,usize) rank+E(f,usize) first+returned order) | factor.rs:359-401. RCM base E(f,Vec<usize>)+sum_a V(push,2d_a,usize)+E(f,usize) degrees+E(f,bool)+E(f,usize) order. Above it allow four frontier Vecs, active VM, and one marked array; sort is a separate maximum. |
| K08 Free blocks | E(f,u32)+V(push,b,Vec<usize>)+sum_c V(push,f_c,usize)+E(b,u32)+max_c VM(grow-from-one,f_c,usize) stack | bound.rs:84-121. Earlier component buffers remain while a later component/stack grows. |
| K09 Call scaffolding | V(push,1,GroupEntry)+E(1,CaseOutcome)+Arc(CasePrep)+Arc(GroupPrep)+group K4STF encoding+cloned geometry+V(push,<=4,AttemptRecord)+V(push,<=4,PrecisionState) | adaptive.rs:3106-3276,3447-3513. Original source remains alongside clone during solve_case. Group/attempt headers are not covered by their child arrays. |
| K10 Shared retained | M_L+D_L+E(z,WideL)+E(z,WideR)+E(m,BoundedCoefficientsR)+D_R+Factor_L+E(b,WideL)+Arc(Shared<L,R>) | adaptive.rs:1179-1204,1243-1387. M_L is formed member array; D_L directional array. Factor_L=E(h,WideL)+E(f,Vec<WideL>)+2E(f,usize)+E(f,i64)+E(f,PivotScreenL). est_blocks absent at128; retaining it is an explicit upper. |
| K11 Shared build | Prior kept+K10+max(z bool assembly flags; M_R+D_R+z bool at residual formation; E(f,WideL) factor work; condition scratch+BlockRatios clone; pivot tracker peak) | adaptive.rs:1265-1362; assemble.rs:684-685; factor.rs:520-602,738-799; bound.rs:176-180. Residual operators die before factor; work dies before condition. Ratios clones E(b,Vec<usize>)+E(f,usize)+E(b,WideL). |
| K12 Factor solve | Above caller RHS, V(fallible,f,WideL) scaled + max(2E(f,WideL) in solve_scaled, E(f,WideL) y+VM(fallible,f,WideL) returned collection) | factor.rs:636-676. Conservatively permit all these buffers at once; old and new u_free coexist on replacement at adaptive.rs:1778. |
| K13 Solve/fallback | E(n,WideL)+E(f,WideL) RHS+V(fallible,f,WideL) u_free+V(push,4,Vec<WideL>)+4E(f,WideL) evaluated+E(f,ResidualRowL)+residual tracker. Add max(correction E(f,WideL)+K12; fallback; recovery) | adaptive.rs:1701-1825. Fallback=E(z,WideR)+max(E(m,[WideR;144]), E(n,WideL)+VM(push,f,FallbackRow)+fallback trackers+E(4,Option<Option<GateRatio>>)+E(4,Option<(bool,Tracker)>)). Residual list/tracker remains through fallback; evaluated buffers remain through recovery. |
| K14 Recovery/formation_scale | recover result E(q,WideL)+E(m,[WideL;6]); helper adds E(m,[WideL;12])+E(s,WideL)+E(d,[WideL;3])+E(n,OptionWideL). formation_scale instead returns E(q,OptionWideL), with same member/spring/directional/reaction helper terms | recover.rs:270-475; verify.rs:102-283. Helper buffers are real even with no support groups. |
| K15 Verification shared | Complete V=E(z,WideL)+E(m,[WideW;144])+E(d,[WideW;9])+E(b,BlockBoundL)+Arc(VerifyShared). Build adds bounded coefficients+uc_refused, with max(bounded member blocks, widened members+directionals, Uc vectors/row map/maxima) | verify.rs:441-501. Uc has 4E(f,WideL)+E(f,u32) at Nl; later 2E(f,WideL)+2E(b,WideL)+output already in V. Current BlockBound includes refusal field. |
| K16 Pass locals | 3E(n,WideL)+7E(f,WideL)+V(fallible,f,WideL) delta+recovered result+2E(n,VecHeader)+2E(n,bool)+E(B,[f64;2])+prescribed child operands | verify.rs:761-967. For current H/VR source_parts zero prescriptions, child operand lists stay empty, headers remain. **contribution_products creates a one-element Vec for each nonzero free operand (:660-670); add E(1,WideW) at its peak.** Do not multiply that sequential scratch by z. |
| K17 Shift | K16+3E(q,OptionWideL)+scaled profile+shifted factor+control arrays; max(factor work fw, Nl 3fw, ct fw+block maxima bw+next buffer growth) | CHECKPOINT_0 symbolic correction retained. Controls include start/results/current/in_flight/refused/sigma/data/s_refused/failed. Next is sequential per try; earlier results remain. All start/current growth capacities explicit. |
| K18 Report | 5E(q,OptionWideL)+E(f,WideL)+V(fallible,f,WideL)+E(b,BlockCertificateL)+E(b,BlockNormsL)+E(b,OptionWideL)+E(B,BodyReportL)+E(B,[f64;2])+Arc(VerificationReportL) | verify.rs:1013-1224. Construction also keeps K16, start/shifts/data/refusal controls; afterwards return moves report buffers. Refusal output V(filter,b,BlockRefusal) overlaps caller slots. |
| K19 Tracker alternatives | sum other lazy/tables+current max(lazy old/new grow; taken-lazy+table old/new grow; stable-sort scratch; old-table+kept old/new; old-table+shrunk-kept) + map/holding nodes | adaptive.rs:655-735,776-827. taken lazy drops before prune. Existing lazy caps 768/4608/2816 are inherited source candidates, not fresh compiled witnesses. Use per-tracker offer count Rj for table capacity after shrink; next_pow2(current length) is not valid after reuse. |
| K20 Refusal/finish | Failed verification cache clone + original attempt refusals; pass extension <=2b entries with growth. Selected: classify(values+raw/coupled scales+publication), then evidence(source clone+ledger/state encodings+filtered rows+summary/body clones), then Box(RetainedSolve) | adaptive.rs:2962-3080,3295-3411. Summary child per occurrence=E(B,[f64;2])+E(B,f64)+E(B,Option<f64>). Arc clones share. Selected record deep clone adds summary/refusal children. Filtered absolute/unpublishable rows total <=q. |

Kernel phase result is max over source/prep/group, each precision's K11/K13,
K15, K16–18, K19, and K20, above the allocation-identity union alive there.
Do not add four reports as retained caches: each report is local to its
decision; shared/solved/verification-shared caches accumulate. K09/header,
geometry, decision-summary and result child ownership must be carried to the
corresponding state edge. Independent review still must validate that complete
union; this table is not a claim that every symbolic operator is instantiated.

## H staged window and explicit caller term (A1-sensitive C)

The narrow caller term is the actual K6Model heap (every array capacity and
label/source/id/family string), frames, retained Args strings, stdout/runtime
owners, and one saved attempt clone from the preceding repeat. During prefixes,
add prefix-limit vector/labels and saved full attempts. These allocations are
alive at a reset; a reset does not subtract them.

The only formatting newly required *inside* the observed stages is:
H/staged.rs:59-63 SourceError Debug, and :77-82
Refused(Refusal Debug) / Unresolved(UnresolvedReason Debug), before obs.end.
Therefore H-stage completion adds
Fmt(max(D_SourceError,9+D_Refusal,12+D_Unresolved)), while the outcome is live.
Those enum grammars are at source.rs:190-259 and adaptive.rs:2528-2567.
Refusal's geometry input originates only in geometry_first/assess_rigid_body;
its reachable StructuralError payload is Range/InvalidInput static text.
MechanismWitnessed has six f64 values. Keep standard float Debug/string growth
binding explicit; do not import response's generic 907,668-byte Line allowance.

Stage_begin Line drops before reset; stage-end Line and later outcome/attempt/
record/digest/dump formatting follow the snapshot. Their *persistent* products
(e.g. saved attempts and initialized stdout) remain caller ownership on later
stages. Old/new saved clone coexist only in the post-stage assignment; they do
not become two copies during every subsequent solve. The prefix snapshot also
precedes prefix_line/prefix_matches. No H whole-process formatting programme.

## Finite VR global consumer envelope (A1-sensitive C)

The exact required comparison is still the global summary in vk_scale.rs:510-511.
Let C0 name Args/id/hash/path/runtime owners and selected Case/Model. Let J(v)
be the allocated Value-tree heap from its actual arrays, object nodes, strings
and children; Jbuild includes growth/parser scratch. It is conditional on the
locked serde feature/source and layout binding, never serialized byte length.

| ID | Owner formula / phase maximum | Source |
|---|---|---|
| V01 Family parse | Args/path/read(Ffamily)+sum Cases retained + max(family Vec old/new, current Jbuild(line)+typed current Case build) | cases.rs:223-298. Whole family exists before find; iterator drops unselected cases after selection. |
| V02 Typed Case | Text fields+V(collect,R,Row)+row strings+V(collect,C,Control)+control children+TB(scales)+optional TB(s_full)+not_covered Vec/strings+optional Model | cases.rs:223-281. **Value control child=V(collect,k,(String,String))+sum key/value bytes, NOT a BTreeMap.** The response's value-control SM term is corrected here. Input raw Value object remains live while pairs collect. |
| V03 Typed Model | V(push,N,String)+V(push,N,[f64;3])+E(m,Member)+E(s+d,SpringSpec)+E(omitted,String)+E(r,(u32,usize))+E(l,LoadTuple)+E(t,StationTuple)+all string children | cases.rs:163-220. Original selected Case may own an embedded Model; standalone model clone adds a second Model. Large load instead overlaps file text+raw Value+parsed Model+sha256 scratch (:331-334). |
| V04 Count stage | C0+source adapter/constructor+source encoding+sha256 scratch+VR count graph/profile/layout helpers | vk_scale.rs:381-416; scale.rs:54-153. Source drops at :399. Counts-only/global metric still includes earlier parse phase. |
| V05 Control prepass | C0+SourceParts+TB(R,&str,&Row)+ControlTally Vecs/strings+max(one exact-control comparison scratch) | lane.rs:182-199,48-77. This real borrowed-row tree was absent from the prior coarse parse-control ownership. It drops before solve. |
| V06 Lane solve | C0+ControlTally+lane source encoding+kernel full phase estimate | lane.rs:229-237. Source encoding remains through record construction, separate from CasePrep identity. |
| V07 Outcome/list/comparison | C0+kernel outcome+encoding+ControlTally + max(expected-list initialization; published row map+body map+floor member map+one row scratch+diagnostic lists; record build) | lane.rs:246-324. OnceLock initialization reads/parses file while outcome lives (:250). Floor map ends before case_record. |
| V08 Comparison scalar envelope | For d decimal digits, mantissa bits<=4d; binary64 mantissa<=53 and p2 in [-1074,971]. align raises bits by delta_p2+4delta_p10; add/sub adds <=1 bit, mul sums bits. Nat operator request sums below bound one row's scratch | exact.rs:67-222,265-408,465-510; compare.rs:38-68. Use actual committed reference/control digit/exponent descriptors, not model hexadecimal input as decimal. |
| V09 Diagnostic ownership | <=R+bounded case-level failures; <=R not-covered; <=2R class mismatches and <=2R input-derived strings (target source list max2); <=C entries across each control list. Each text length is its concrete format literals+input id/key/expected+finite reason grammar | lane.rs:205-321; :98-130,340-365. Arrays persist; one transient verdict/source-id String can coexist with a retained failure clone. Source-refusal path <=R+1 failures and separate record. |
| V10 Record/output | run.record+record clone+max(Jbuild(w1 line),Jbuild(report),Jbuild(record line)); also published map/control/failure lists until drop(run). Direct Value writing streams; generic full serialized String is not assumed | vk_scale.rs:435-485; records.rs:21-120. json! borrowing/copying needs locked serde binding. Take full-tree union as conservative alternative. |
| V11 Post-run RCM | C0+record clone+OnceLock + max(AdjBuild,AdjKept+K4RCM,AdjKept+K4order+SDRCM,AdjKept+two orders) | rcm.rs:13-59; vk_scale.rs:487-495. Set-to-Vec conversion permits both old trees and destination buffers. SD BFS clones last_level where K4 moves it; do not reuse K4's exact scratch. |
| V12 Sparse parity | C0+record+OnceLock+Binary64Envelope below; afterward retained solution/error plus class String | vk_scale.rs:498-507; parity.rs:200-247. parity(false) returns before dense allocation. |

Nat request operators for V08, with k=ceil(bits/32):
parse=Fmt(d)+V(extend,ceil(d/9),&[u8])+VM(push,k,u32);
shl=VM(grow-from-floor(shift/32),ceil((bits+shift)/32),u32);
mul_pow10=initial exact clone plus growth to ceil((bits+4power)/32);
add=E(max(ka,kb)+1,u32); sub=E(ka,u32);
mul=E(ka+kb+1,u64)+V(owned-map,ka+kb+1,u32).
Charge both old/new buffers if in-place behavior is unbound. Sum requests of
the finite expression DAG to bound its live scratch, then take max over rows/
controls. That is a source-backed conservative composition; this continuation
has not extracted/validated every decimal descriptor or accepted the previous
345,392-byte instantiated number.

Concrete file identity bindings now recorded: RF-LARGE family 986,182 bytes,
SHA256 16357afa5efeaaac3ab5798bb6f632104bec1e2dfada288f5b0936bfe6188759;
expected_unresolved 1,357 bytes,
SHA256 2e5d0975a90c69e5d198d61760cc7734d42615461710da1fe226f60b4a46f2c9.
Large model bytes were not found in the VR tracked subtree by the targeted
filename search; existing expected hashes are not regenerated inputs.
Future argv follows runner:119-126: binary,--case,id,--heap-cap-bytes,decimal cap,
optional --model-file,path and optional --counts-only. Supply actual path/argv
byte lengths and sealed file hashes; do not replace them with guessed strings.
Argv/file/runtime/library byte formulas remain explicit bindings, not Emax slack.

## Sparse Binary64Envelope

This finite path has actual axis-spring counts where present; vk_scale's
named RF-LARGE models have none. Directional springs return the named
binary64-form error before assembly (parity.rs:74-79), while the retained kernel
still handles their actual input. No dense branch is executed.

Let c=144m+s, z64 global stored pattern entries, zf stored free entries,
he=nonzero lower entries, hs numerical-zero-dependent skyline. hs is **not h**;
use hs<=f(f+1)/2 arithmetically if no tighter proved count is available.
Let te be contribution multiplicity at global entry e, and tr maximum sum
of te over one row. Each Expansion accumulates <=te scalars; an intended-action
row accumulates <=1+2tr scalars. Growth/clone/add scratch remains explicit.

| Phase | Complete named owner union above C0+record+OnceLock |
|---|---|
| Adapter | V(fallible,N,FrameNode)+V(fallible,m,FrameElement)+V(push,s,(usize,f64))+E(n,f64)+E(r,usize)+V(filter,f,usize)+E(r,(usize,f64))+prescribed-sort scratch. Temporary nodes/restrained die at adapter return (parity.rs:45-102). |
| Contributions/allowances | Kept adapter+VM(push,c,StiffnessContribution); then kept contributions+two TI(z64,(usize,usize),(f64,usize)) maps. Magnitude map is consumed into out (parity.rs:106-164). |
| Assembly | Prior kept+E(m,(usize,usize,Matrix12))+E(m,NodePair)+E(s,usize)+connectivity-neighbor buffers+spring bools+expanded rows+compressed pattern+E(z64,f64) values. Matrices are inline array elements, no heap per coefficient (sparse.rs:82-159,592-694). |
| Prepare | Prior kept+round/count arrays V(push,z64,f64/usize)+E(n,usize) prescribed_position + E(f,i32)+E(n,usize) free_position+E(f+1,usize)+three growing zf arrays(columns,values,source entries)+E(f,usize) diagonal+E(f,f64) RHS+E(zf,usize) transpose+one row entries/couplings+validation bools+symmetry String (sparse.rs:889-915,1299-1478). |
| Contribution audit | Prepared-under-construction **still retains free_position** until prepare returns, plus z64 Expansion headers+sum_e V(push,te,f64); clone differences headers/children and active add; V(push,z64,ContributionRounding)+cloned exact children <=sum_e(2te+1) doubles. One coupling-index Vec remains even when zero prescribed values remove products (sparse.rs:1523-1593). |
| Ordering | Prepared kept+V(collect,he,SymmetricMatrixEntry)+adjacency+SDRCM; then profile builder ordered_position/first/original order/row starts+VM(resize,hs,f64). Profile values die before actual factor, while returned order/first remain during factor construction (SD/structural.rs:56-83; SD/lib.rs:215-275). |
| Factor | Prepared+caller order/first+E(f,Vec<f64>)+8hs+factor order/first clones+seen/position+factor work and pivots. Factor construction returns before condition; caller ordering arrays die on return from factor_sparse_structural_ldlt (sparse.rs:1743-1795). |
| Condition/refinement | Prepared+factor+cloned pivots; max(condition x,y,signs and inner solve buffers; y+full-u+residual list+correction+inner solve; intended audit; success clone). Iterations are sequential, at most5 condition /3 corrections (structural.rs:1588-1646,1696-1803). |
| Intended/success | Original residual+new intended rows+fresh contribution sums+one Expansion of <=1+2tr scalar terms; then returned u/both reports/pivots/scale clone/rounding outer+child clones/symmetry String. Prepared/factor stay through finish; drop before class formatting (structural.rs:918-985,1750-1780). |
| Refusal | Complete prepared construction prefixes; negative direction up to n doubles; factor-failure direction mapping plus partial buffers before unwind. Class names avoid formatting direction Vecs (parity.rs:167-178). |

Thus Binary64Envelope is max over these complete phase owner sets, not their
all-phases sum. Vglobal=max(V01…V12 plus finite I/O/output/error phases),
and Hstaged=max(H source,H solve,H prefix snapshots) with their caller terms.
Neither quantity is numerically instantiated in this packet. Remaining finite
parser/output baseline and library bindings are listed explicitly; no metric
has been replaced.

