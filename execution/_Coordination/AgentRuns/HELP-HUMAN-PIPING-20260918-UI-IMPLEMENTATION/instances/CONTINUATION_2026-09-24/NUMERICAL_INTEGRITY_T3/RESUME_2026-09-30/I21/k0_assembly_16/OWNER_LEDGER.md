# Canonical phase/owner ledger — candidate K0

Source40129; all application formulas remain final-A1-sensitive. This file integrates the sealed source05/06/07/09 ledger and additive reviewed caller facts. It does not rerun those proofs or assert a complete numeric bound. Bytes mean registered allocator-request bytes, not RSS, footprint, allocated size classes or stack.

## Algebra, bound facts and finite schedule

For one phase phi, R_phi is the sum of distinct live heap identities; M_phi is R_phi plus the one currently growing old buffer. Deep copies are new R owners. Moves and shallow Arc clones preserve identity. Rebuild old/new buffers coexist in R; they are not a realloc surcharge. Sequential helpers use max, not sum. Checked overflow is an unavailable bound, never a wrapped estimate.

E_s(k)=s*k. Fresh push/borrowed lower-zero/fallible G_s(k)=s*P_s(k), P_s(0)=0 and P_s(k)=max(mu(s),next_pow2(k)); mu=8 for byte,4 for2..1024-byte elements,1 above1024. General fresh capacity A_s(k)=s*max(mu,2k) when k>0. A buffer starting at c0 uses the actual recurrence max(2c,len+additional,mu) or proved max(c0,mu,2K), not an empty-Vec power-of-two fiction. Fresh clone uses length. Stable sort extra is s*max(k,48) for k>=2; unstable sort adds no heap on the pinned source. Only the active grow receives its old request. source02 supplies these implementation contracts.

Kernel widths p128/p256/p512/p1024 use (L,R)=(4,4),(4,8),(8,16),(16,16), w_L=48/80/144. Verification v256/v512/v1024 uses (L,W)=(4,8),(8,16),(16,16). Option<Wide<L>> has the same measured w. Original one-source calls have at most4 distinct S/U slots and3 V slots; a pending verification becomes candidate without another solve. Reports are current locals, not a four-report cache.

Kernel layout08 requests, scoped to its authenticated40129/Rust1.97.1/aarch64 release artifact:
- geometry tree leaf/internal144/240; tracker-map1072/1168; tracker-key-set104/200. Requested-byte TI(k)<=[1+floor((k-1)/5)]*max(leaf,internal), k>0. No private alignment inference.
- (u32,SpringKind)8; Dof8; resolution/floor tuple(u32,u64,u64)24; theta(u32,f64)16; certified(u32,u64)16.
- Derived Arc request sizes: PREP432, GROUP368, Shared720, Solved104, Report344, VerifyShared560/592/656. These use the reviewed ArcInner repr(C) derivation, not an observed fixed baseline.
- Final transfer of these request/layout facts to a different built candidate remains required.

Descriptors are N,m,s,d,r,l,t,u,B,b,n,f,z,h,q, original input String lengths and construction classes. n=6N; U=78m+s+6d; raw pattern insertions Pi=144m+s+9d; q=7N+12m+6t+s+3d+r+2u. Ledger limbs J<=68v, v<=min(l,n), without cancellation assumptions. Force/moment rows F=q-7N. Use actual H/VR descriptors; no generic spring/body zeroing. H's explicit adapter has s=d=u=0; that source fact is not transferred to VR.

## Persistent kernel identities

Define the following retained children (all counts are length or the specified capacity class):
- SRC0: one original PrimitiveSource array/String/support/prescribed/body-index owner set. Array strides24N,88m,24s,48d,16r,40l,16t,64u; prescribed16n, body4N, actual load-ID Strings and4-byte support IDs. Capacity classes come from the H/VR constructor adapters.
- PREP: Arc432, a distinct exact source clone, ledger48v+v+8J, single-case prescribed32r+16r, encoded SRC, G_20(q) layout,8B extents. Input source remains SRC0.
- GROUP: Arc368, geometry24B+sum G_8(k_c), pattern8(n+1)+A_8(z)+8z+8(z+1)+8U, orderingG_8(f)+8n+24f, free blocks4f+G_24(b)+sum G_8(f_c)+4b.
- CALL: one1464-byte GroupEntry backing,80-byte outcome backing and stiffness encoding. Its inline1360-byte GroupCache is not another heap allocation.
- GEOM_COPY:24B+sum E_8(k_c), one schedule clone. It moves to the outcome. Error clones fit the reviewed two-copy geometry envelope, not another three-copy accumulation.
- ATTEMPTS: G_832(4)=3328 backing; STATES: G_16(4)=64 backing. Current stack records/handles are not heap headers.
- ATTEMPT_KIDS(v): three finite slots40B+G_24(2b); summary resolution clone is exact16B. A current summary build aliases its slot, adding no duplicate40B.
- Failed-verification cache slot VE(v): E_24(b). Success/failure for a slot are alternatives; padding both is explicitly conservative.

With M_L=(7888,13136,23632), D_L=(440,728,1304), C_L=(248,408,728), Pivot_L=(104,168,296):
  Factor_L=h*w_L+f*(48+Pivot_L).
  S(p)=720+m*M_L+d*D_L+z*(w_L+w_R)+m*C_R+d*D_R
       +Factor_L+[p>=256]*b*w_L.
  U(p)=104+(n+6m+q)*w_L.
  V(v)=Arc_v+z*w_L+144m*w_W+9d*w_W+b*(208,336,592)_L.
  Report*_v=344+5q*w_L+(f+P_w(f))*w_L
             +b*(976,1584,2800)_L+B*(624,1040,1872)_L+G_16(B).

The single finite padded prefix is
  Kpad=SRC0+PREP+GROUP+CALL+GEOM_COPY+3328+64
       +sum_p[S(p)+U(p)]
       +sum_v[V(v)+40B+G_24(2b)+E_24(b)].
It intentionally pays complete finite roster slots early and retains some mutually exclusive storage. It is not an arbitrary runtime-address census or tight prediction. E_sel128 uses only p128/p256,v256 slots and their reached phase alternatives; header minimum-capacity padding remains explicit.

When a roster owner is under construction, OMIT its returned identity before inserting its construction envelope, or prepay its complete result and add only helper extras. This is an identity operation, not subtracting an unrelated observed upper from live bytes. Every phase below uses that rule.

## Canonical phase maximum

| Phase / frozen sources | Distinct union or construction replacement |
|---|---|
| Source adapter/constructor; source.rs352-590 | SRC0 constructor plus caller adapter owners. Above returned padding: max(one input stable-sort scratch,3*2144 exact-sum entries,union-find8N+4N); source09 constructor capacities/ID format and old source inputs retained at their actual edge. Source refusal is its own unwind/format edge. |
| Group geometry; factor.rs130-219,rigid_body.rs45-247 | GROUP geometry prefix plus one active body's node list/tree/positions/ground directions/relative coordinates/SVD/candidates/motions/Expansion children exactly as source05 K03. Use geometry TI240,NotAssessed stride8; different bodies are sequential, completed geometry children survive. No whole geometry-copy multiplication. |
| Pattern/tagging; assemble.rs550-646 | Pattern return plus positions G_16(U), raw row headers24n+sum G_8(p_g); then tagged G_16(U), starts8(z+1), next8(z+1), items8U. Consumed source backing remains during output construction; raw rows drop before transpose. |
| Order/blocks; factor.rs226-401,bound.rs84-121 | Ordering return plus adjacency/neighbor capacities and source07 reach/eccentricity/queue/sort alternatives. Kernel eccentricity has4 buffers, not SD's6. Free-block DFS stack is current-component only; completed blocks remain. |
| PREP/ledger; adaptive.rs876-977,ledger.rs103-129 | Original SRC0 beside exact clone; G_1104(v) ledger-build backing coexists with48v+v+8J result, then drops. Encoding growth and one extent scratch occur at their own phases. |
| S build; adaptive.rs1243-1395 | Current S replacement; helpers are max(z flags, residual-width m*M_R+z flags, f*w factor work,5f*w+24b+8f condition, pivot tracker). Do not duplicate retained directional operators or condition estimates moved into S. |
| Own solve/residual/correction; adaptive.rs1467-1836 | Current U replacement plus RHS, fallible u_free, evaluated states and residual/tracker alternatives. Factor-solve helper above RHS: R=(2P(f)+f)w, M=(2.5P(f)+f)w. Correction delta is separate while old u_free remains. |
| Fallback/recovery; adaptive.rs1613-1804,recover.rs270-475 | Residual prefix plus bounded Abar/member-block OR current fallback-state/row-list/trackers. Up to4 eligible fallback trackers plus held residual; not4 row lists. Chosen exact-f clone overlaps old P(f) u_free at assignment. Residuals drop before recovery; recovery adds(12m+s+3d+n)w beyond its returned U children. |
| V shared; verify.rs441-501 | Current V replacement plus bounded coefficients m*C_L+16b and max(144m*w_L,widened m*M_W+d*D_W,Uc). Widening drops before Uc. Uc first4f*w_L+4f; later2f*w_L+2b*w_L plus its returned BlockBound padding. |
| Resolution TOP/HATCHECK; verify.rs289-360,761-789 | Scale=Kpad+n*w+q*w+2B*w+G_16(B). HatCheck=Kpad+n*w+q*w+2G_16(B). TOP and HATCHECK do not overlap. RES is one G_16(B) owner; add only the currently growing collect's old request O16(B). |
| Verification pass before shift; verify.rs761-967 | Pass*=3n*w+(7f+P(f))*w+(q+6m)*w+50n+PrescribedChildren+G_16(B). Replace delta's ordinary term by solve construction; recovery and formation-scale helpers are sequential. Prescribed headers/flags50n remain even with zero values. |
| Shift factor/work/Nl/retry; bound.rs1190-1378 | Kpad+Pass*+3q*w+profile+factor+controls+max(f*w,3f*w,f*w+b*w+next). profile=h*w+36f; shifted=h*w+(32+w)f+b. Controls= b+16b+G_start(k)+E_result(k)+A_8(k)+16b+G_start(k)+b*w, k<=b, start stride64/96/160,result272/432/752. Next is a distinct current-backings overlap; old shifted factor drops before retry. k=0 creates no profile/factor. |
| Report build; verify.rs1013-1224 | PassRest=Pass*-G_16(B)-(f+P(f))*w. Use Kpad+PassRest+Report*+caller shift controls+spent G_24(b). RES,r_hat,delta and5 row arrays are counted once inside Report*, not again as pass/early rows. After return only Report* survives. Summary40B aliases ATTEMPT_KIDS(v); original spent-refusal backing overlaps attempt extension then drops. |
| R7; adaptive.rs2040-2301 | Kpad+Report* with skip q, raw/coupled scale peak8B*w_v then4B*w_v, exact hats16B, current tracker set; later Decision=G_16(4B)+2G_16(2B)+[v1024]*16B. Drain may pad held trackers with completed Decision; one active grow only. |
| Canonical/certificate; adaptive.rs3080-3370,3948-4135 | Before R7 one G_20(q); after Decision a separate later G_20(q). They do not overlap. Draft provisional24q+64B scales+G_64(q)+A_16(4B); values/scales drop before H/RU with G_64(q)+A_16(4B)+8q radius. Borrowed fallible published-row collection uses P(q). Rejected drafts drop before retry. |
| Finish; adaptive.rs4184-4306 | Kpad+Report*+Decision+certified publication/radius plus1976 box, fresh40B summary, selected-record child clone<=40B+24*refusal_len, encodings,decision clones<=128B,body scales<=64B,r*8,G_24(absolute rows),G_40(unpublishable rows),B*(24+16),G_16(B) certified tuples,optional floor24B. Moves/Arc clones do not duplicate payloads. |
| Refused/unresolved/selected return | Partial success buffers unwind; attempts/geometry move or drop as code specifies. The selected result retains its referenced prep/group/cache/state/evidence/publication/radius, but no Report. Generic selected deep clones/combinations need their own finite operand identities; original H/VR makes one solve_case per invocation. |

PrescribedChildren=sum_g[G_w(tau_g)+G_wW(tau_g)] uses the actual adapter contract; the numeric subtotal in SUBSTITUTIONS excludes it explicitly. No zeroing is hidden.

Tracker high-water is f at pivot/residual, f per fallback (<=4), and total<=q+2F across<=8B R7 keys. Table capacity uses max(4,2R_j), including shrink/rebuild history. Its lazy/table/sort/old-kept/pruned-kept alternatives are source05 K19. Known lazy-only retained/move slot bounds: standalone512/768; residual+4fallback2560/2816; global set4352/4608, stride4304. Tables and tree nodes are additional. The two kernel tracker TI specializations now use1168 and200 request maxima. Do not put a lazy realloc-old charge on a simultaneous table grow.

QueueRet(c)=8*P8(max(1,c-1)) for c>0, else0. QueueOld=0 until population>4, then4*P8(population). Reach's vec![seed] old surcharge is0 at c<=1,8 at2..4,otherwise4P8(c). Only one queue/component exists at once. Kernel Ecc has4 G_8(c) buffers; SD Ecc has6 as reviewed.

Encoding bounds: SRC length38+24N+84m+17s+41d+13r+17l+ID_bytes+16t+22u+4*support_ID_count; STF26+24N+84m+17s+41d+5r; LED10+18v+8J; RST22+(n+6m)*(9+8L). Use known fresh String/Vec chunk-capacity bounds, not length for a growing retained buffer. Clone of a completed encoding is exact length.

## H caller union — original staged and prefix measurements

C_H=model/frame/Args children + stdout buffer + reviewed direct-runtime identities. RF and DEC model/label/frame constructor terms are source10/source09; public S_coord=sizeof(Option<[f64;3]>) remains directly bindable. Counts/Observer wrappers have no child heap; dead count/start-line temporaries are outside stage reset. Args only retains model ID, optional counts path and first-pass rows path; launch-byte descriptors are mandatory.

H_runtime=1024+M_stdout+beta*(M_stack+L_info+4), beta<=1. The exact reviewed layout08 executable gives64/544, hence conservative1700 bytes on that artifact only. Transfer to final H's selected compiled sites is a separate binding gate; do not call this a process baseline. H-R2 initialized dropped byte-write/flush extra0 and H-F0 float helper extra0 retain their reviewed completed-stage/main-thread qualifications.

Saved attempts S_r=0 on repeat0, otherwise one clone<=3328+120B+144b. PrefixList<=387 bytes (J<=9). Old/new saved clones overlap after the inner solve sample, not as two permanent copies. A current selected outcome drops per iteration.

  H_source(r)=C_H+S_r+max(source constructor/refused-source format phases).
  H_solve(r)=C_H+S_r+max(kernel phases, returned Outcome+ErrFmt).
  H_prefix(j)=C_H+Saved+PrefixList
              +max(kernel phases,Outcome+ErrFmt,Outcome+ErrString+StageEmit).

Inner Observer::end reads before stage-line construction. Outer prefix read follows it, so StageEmit belongs there; prefix_line follows the outer read. Error String and escaped Line copy are distinct. Preserve reviewed H no-error708 and finite error-string/line formulas, active old only for the growing String. Backend-fatal/general panic/foreign initializer qualifications are not erased by a fixed direct-runtime subtotal.

## VR global union — caller phases remain separate

Precut contains Args/id, original Case and optional embedded Model plus standalone Model clone, hashes, pre-cut parser/count/source peaks and runtime. Source0 input has dropped at the proposed tV boundary, but its prior peak remains in global history. A same-process cut is an unselected contract alternative, not a replacement here.

After cut take a maximum over:
1. Counts/Phase JSON, counts-only summary or half-cap refusal.
2. SourceParts/control lookup/exact controls; source validation/encoding/hash; one kernel union.
3. Selected comparison: Outcome+encoding+publication map+body/floor maps+diagnostic lists and row E/S plus max(floor,predicate,range) exact helpers.
4. Record1 build while Outcome/encoding remain; lane return drops kernel Outcome. Phase map then record2 clone.
5. Full-floor/report JSON and persistent not_covered_full; record wrapper serialized3 while record1/2 persist.
6. run drop, leaving record2/not_covered_full/expected-list78+precut survivors; RCM then sparse parity.
7. Final partial summary reads. Final emit is after those reads; earlier emit peaks remain globally relevant.

Thus VR_R=max(PrecutPeak_R,max_phi(PrecutSurvivors_phi+Caller_phi+Kernel_phi)), with the analogous move union. PrecutPeak and unclosed runtime/parse/IO requests are explicit blockers. H's fixed term is never substituted for these VR owners.

V-EXACT uses the reviewed parse/align/add/mul/scalar/magnitude/floor phase maxima, with E/S retained through later diagnostics. Public slice-reference size stays sizeof<&[u8]>. Floors and actual RF-LARGE24 divisions use source14's reviewed fixed-input proof;138 wider-lane rows additionally require the ordinary unmutated Selected40129/R7 consequence verified by RV28, quotient<=2^1020. No arbitrary-model or mutant finiteness claim follows.

V-SPARSE is Late+max(Hfront,W+HSolver,W+max(Solution,8n)+HClass,each early-error prefix). Use metric_design_04's complete preparation/audit/Expansion/order/refinement equations plus the mandatory corrected
  Build=Fct+16f+f+8f+8f+8f+epsilon*O_G(PivotEvidence,f);
  HFactor=max(n+16f,Build).
The16f is caller ordering in both metrics. L includes stored zeros. Sparse hs is value/order-sensitive; use its own bound, never W1 h. The reviewed source-only fallback hs<=f(f+1)/2 is permitted but may defer large runs. Expansion old/new replacement is two R owners; one active new-buffer grow is separate. Late contains no dead W1 cache/outcome.

V-JBUILD uses source15 prefix/object/array/temp/clone/merge equations. Record1/record2/serialized3 are separate; selected retained upper60*M_J+4*sizeof(Value)+I+Fm+D_geometry+3808 is NOT its construction peak. Refused-text formatting requires all3 phases: max(H_refusal_text,F_refusal+H_outer_format,F_outer+D_outer). Phase.fields sample prefixes are Leaf_J+19/+33/+45; normal summary Leaf_J+32+I/+54+I. Streamed to_writer creates no full byte-line String; itoa/wrapped IO-error and dynamic format cells remain.

## Exposed residual symbols

No free allowance parameter is accepted. Exact unresolved cells are:
- Final source/type/stdlib/request-site/feature/target transfer of recorded facts, including H64/544, private kernel strides and Expansion32.
- Input/launch byte lengths and construction-class correspondence, public sizeof expressions and checked descriptor validation.
- Seven actual VR consumer leaf/internal request pairs: JSON, borrowed controls, publication, floor, adjacency SetValZST, allowance, Case String map.
- Pre-cut VR read/parser/count/start-output/runtime history and survivors on the actual launch paths; source/counter estimates themselves must not be omitted.
- Dynamic diagnosis/RowClass/parity/remaining format-helper, itoa and wrapped IO/error/panic terms on reached branches.
- A useful source-valid SD hs upper if the chosen algebraic bound prevents the planned admission; correctness and usefulness are distinct.
- Final checked implementation/identity-union tests and independent candidate review.

All later steps remain plans/owner decisions. No new generic proof programme, runtime, layout probe or acceptance is commissioned by this ledger.

