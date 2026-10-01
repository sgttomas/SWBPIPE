# source_09 — finite caller-to-kernel descriptor proposal

TASK I21; source-only start2026-10-01 13:39:03 UTC; boundary14:04:03 UTC.
Frozen source40129a225d73860ac2a53da9a2fa73869df668f3. No builder, generator,
model, solver, scale run, API implementation or new expected output was run.
This is a proposed descriptor contract, not an admission/record change.

Basis: reviewed source_05/source_06 finite kernel roster, corrected resolution
identity, reviewed source_07 queue recurrence. ROOT's native update reports
RV30 layout08 runtime backcheck passed (sealef796a0a53ce403ce99a4132b14487ed53156bebb0e2046ee0ba50b642107693,
K6C31fdd2794ea1c588571257eff660a8dbb29ee259, ruling
NUM52a2dec3105178a40d41bb4a34e9d4f7801c4ceb). Thus the exact T1 requested bytes
and five T2 layouts are usable only on40129/Rust1.97.1/aarch64 release.
No private-node alignment, observed-baseline allowance or full E_max follows.
This status update does not restart the tranche.

P=projects/chirality-piping; H=P/core/solver/performance_harness;
VR=P/validation/benchmarks/numerical_robustness; K=P/core/solver/frame_kernel/
src/structural/retained. Model/count/adapter names below are source paths.
HM=H/src/k6/models.rs; HC=H/src/k6/w1/counts.rs;
HA=H/src/k6/w1/adapter.rs; HS=H/src/k6/w1/staged.rs;
VC=VR/src/cases.rs; VS=VR/src/scale.rs.

## Domain and pre-solve seam

HM:141-158 names exactly24 RF-LARGE (three families×four sizes×two
orientations) plus nine DEC053 fixtures. FIXTURE_INPUTS.json carries all33
existing committed W1 count rows, using only structural fields. Measured
heap/time/estimate fields from that file are deliberately not descriptor data.

VR/tests/scale.rs:40-71's193 means factored non-large CI cases, not the full
family domain. The ten committed family files contain213 cases:201 embedded
models (including eight declared geometry refusals) and12 externally supplied
large models. Keep all201/213 in coverage; retain193 as the original test
obligation. The metadata census preserves88 axis-spring cases,22 directional
cases,38 cases with multiple load terms at a DOF and one zero-load model.
No such category is excluded to make a formula fit. The12 external large
models retain their committed hash/generator binding, not fabricated arrays.

Counts are available from existing pre-solve count paths, not from future
kernel allocations: HC:166-217 constructs/validates adapter source
and graph counts; VS:134-153 uses Model and PrimitiveSource.
For an estimate needed before those count helpers themselves run, use the
input-only uppers below and account their construction separately in the
applicable caller window. This proposal does not erase count-stage memory.

## Finite proposed shape (data contract only)

A descriptor carries a source identity/hash plus construction provenance
(H fixed Builder / H DEC053 conversion / VR typed Model) and scalar fields:

    input: N,m,s,d,r,l,t,u, sum_source_id_bytes, max_source_id_len,
           support_axis_id_lengths[], support_directional_id_lengths[]
    structural: n,f,B,b,z,h,q, source_valid
    refinements optional: body_node_counts[], block_free_counts[],
           pre_dedup_row_counts[], free_neighbor_counts[],
           load_terms_per_DOF[], directional_counts_by_node_kind[]
    bound_mode: exact_pre_solve_count or declared_input_upper per field
    source_capacity_class: exact_collect / fresh_push / exact_clone /
           fixed_format / owned_map_with_explicit_byte_envelope

These are field names and mathematical modes, not implemented Rust types.
Physical supports in VR remain represented by constraints and axis/directional
springs. u=0 means the adapter does not create aggregate SupportGroup records;
it does not zero or remove those physical support contributions.
Arrays can be omitted using the global sum/max bounds below; no hypothetical
heap address, observed peak, solver outcome or future capacity is an input.
Existing public counts remain unchanged; a later shared H-library estimator
may consume an enriched descriptor only after separately authorized design/
implementation/review. VR's combined springs field must split into s and d
at its actual source_parts branch; passing springs as axis-only is invalid.

| Field | Exact caller/source origin | Conservative input-only alternative |
|---|---|---|
| N,m | H K6Model node/member lengths; VR Model nodes/members lengths | Direct input lengths; no numerical evaluation |
| s,d | H adapter default empty proves0 for its fixed33; VR cases.rs:493-511 splits Some(axis)/None | Count actual spring variants. k=0 omissions are existing generator semantics, retained separately, not new exclusions |
| r | H restrained mask popcount via models.rs:99-107; VR constraints.len | Count raw entries. On valid source they are unique; duplicate constraint is SourceError, not f=n-r underflow |
| l | H loads.len after its fixed nonzero builder/filter; VR model.loads.len including every authored cancellation term | Never net/cancel/filter again for this descriptor |
| t,u | H stations=m; VR stations.len; both adapters leave supports empty | u=0 is source-proved for these callers only. A supplied generic support path requires actual u and child lengths |
| n,f | Source dof_count and free_dofs; valid n=6N,f=n-r | Before validation use n=6N and f<=n; invalid source takes source-construction/refusal envelope, not a fabricated successful solve |
| B | Source member-graph body_count, source.rs:535-564; VR Model::bodies same topology | B<=N. Springs do not join bodies; isolated nodes remain separate |
| b | H structural_profile DFS:141-161; VR profile:112-131 on free adjacency | b=0 if f=0, otherwise1<=b<=f. Do not force b.max(1) as a real block count |
| z | H counts:191-194:36(touched_nodes+ordered_unique_node_pairs); VR scale.rs:58-90 adds untouched-node spring entries | z<=min(n*n,144m+s+9d), checked. Repeated members/springs dedup pattern entries but retain contribution multiplicity |
| h | Same exported K4 RCM order and skyline rank rule, H:125-139/VR:95-110 | h<=f(f+1)/2, checked; no numeric factor/solve needed |
| q | layout(source).len or explicit grammar | q=7N+12m+6t+s+3d+r+2u on valid source |
| U,p_g | K/assemble.rs:548-625; unique member endpoints validated source.rs:399 | U=78m+s+6d; sum p_g=144m+s+9d. For global DOF g: p_g=12*incident_member_count(node(g))+axis_count(g)+3*directional_count(node,kind(g)) |
| d_a | Distinct free neighbors in K4's structural graph | sum d_a=0 when f=0; otherwise<=min(f(f-1),sum p_g), each<=f-1. Structural zeros still count; numerical coefficient values do not prune adjacency |
| N_c,f_c | Body/member connectivity and free-graph components in existing count passes | sum N_c=N,max N_c<=N; sum f_c=f,max f_c<=f. No assumption B=b=1 |
| v,J | v=number of distinct loaded DOFs, including zero or cancelled rows; ledger.rs:103-129 | v<=min(l,n); J<=68v from fixed ExactAccumulator limb range. No need to compute exact cancellation to use this bound |
| source-ID lengths | H k6:decimal_global_DOF; VR load src strings, cloned verbatim | H length3+digits(g),g<n; VR input UTF-8 byte lengths. Length is not capacity, hence capacity table below |
| support children | source.rs:512-517 sorts/dedups but does not shrink supplied ID Vecs | Current H/VR support arrays empty. Generic path keeps original capacities or explicit construction provenance; unique child IDs<=s/d does not bound pre-dedup capacity |

All valid-source quantities are structural or input grammar. Geometry refusal,
zero data, cancellation, NotAssessed and successful witness branches remain
covered by the source_05/source_06 roster; none is predicted away.

## Kernel-input capacity table

E_s(k)=s*k; G_s(k)=s*P_s(k), P_s(k)=max(mu_s,nextpow2(k)) for k>0,
zero at0; A_s(k)=s*max(mu_s,2k) at k>0. Pinned source_02 contracts apply.
RawVec minima:8 bytes for element size1,4 slots for size2..1024,1 above1024.
Fresh clone/borrowed exact collect requests logical length; old input and
new output remain separate where a map cannot reuse.

| Source owner | H adapter.rs:39-85 | VR cases.rs:470-533 |
|---|---|---|
| nodes | exact borrowed map:24N | exact nodes.clone():24N |
| members | exact borrowed map:88m | fresh pushes:G_88(m) |
| axis/directional springs | empty by fixed adapter | separate fresh pushes:G_24(s)+G_48(d) |
| constraints | restrained_dofs creates G_8(r), then owned map to16-byte Constraint; source destination exact16r; input backing transient | fresh pushes:G_16(r) |
| loads | exact borrowed map:40l | fresh pushes:G_40(l) |
| source-ID children | fixed format bound F(3+digits(g)); precise derivation below | String.clone copies source length:sum byte lengths |
| stations | exact range map:16t, t=m | fresh pushes:G_16(t) |
| support arrays/children | zero for this source_parts | zero for this source_parts |

H constraint map expands actual element size8->16 with equal alignment;
source_02's in-place eligibility fails, while map of IntoIter has exact
remaining length. Count the old G_8(r) input during adapter construction.
PrimitiveSource::new moves these arrays, sorts in place and preserves their
capacities; it adds exact16n constrained slots and4N body IDs. It does not
shrink loads after cancellation or deduplicate its source arrays. Source clone
inside PREP makes exact-length copies of every array and String, independently
of SRC0's retained capacity. Source sorting scratch stays in K01.

Hence one original kernel source request is explicitly:

    SRC0_H = 24N+88m+16r+40l+16t+16n+4N+sum_i F(len_id_i)
    SRC0_VR = 24N+G_88(m)+G_24(s)+G_48(d)+G_16(r)
              +G_40(l)+G_16(t)+16n+4N+sum_i len_id_i
    PREP_source_clone = 24N+88m+24s+48d+16r+40l+16t
                        +64u+16n+4N+sum len_id_i+support_child_bytes

PREP clone's shallow source headers are already in ArcCasePrep; do not add
another source struct Box. All formulas use actual source type strides
already bound on the selected target. Generic support-child capacities do not
silently enter the fixed33/213 callers.

## Geometry/contribution refinements without future outcomes

A caller need not run geometry to supply NotAssessed counts. Per body its
child tuple count<=2N_c; grounds pushes<=r_c+s_c+6N_c before dedup. Directional
input list for one node/kind has e entries and may append at most3 axes;
start from fresh filtered P_24(e), then use the RawVec recurrence, or safe
capacity max(P_24(e),4,2(e+3)). If e=0 no axis append is entered. Dedup and
spans_space results do not justify shrinking or removing this upper.

For missing body/component histograms, use these source-only substitutions:

    sum_c G_s(k_c) <= s*(mu_s*C + 2*sum_c k_c)
    sum_g G_8(p_g) <= 8*(4n + 2*(144m+s+9d))
    sum_a G_8(d_a) <= 8*(4f + 2*sum d_a)
    sum_a G_8(2*d_a) <= 8*(4f + 4*sum d_a)
    sum_c G_8(f_c) <= 8*(4b + 2f)
    sum_c G_8(not_spanning_c) <= 8*(4B + 4N)

C is the number of indexed lists, including empty lists harmlessly in the
upper. The inequalities follow P_s(k)<=mu_s+2k; they bound capacities before
observing a future Vec. Active-body helpers use N_c<=N and global r/s/d for
their maxima, while completed body outputs use B<=N. These can be loose but
do not narrow spring/multibody/geometry paths or borrow measured slack.

## Checked arithmetic and unresolved boundary

Compute all descriptor products/sums in checked u128, then separately check
representability for the target usize/Layout request and public u32 IDs.
Use checked_add/mul/sub and checked_next_power_of_two semantics; no saturating
wrap, float conversion or unchecked source casts become proof. For valid
counts require r<=n, sum body nodes=N, sum block DOFs=f and all IDs in range.
Compute f(f+1)/2 by dividing an even factor first, still checked.
Overflow/invalid provenance means descriptor unavailable/refusal of the
proposed estimate, never product source rejection or altered admission.

Current old H/VR count structs omit s/d split, contribution/child/string
capacity provenance and some histograms. Global uppers above close the
kernel's need for unknown future helper populations, but binding every
concrete prelaunch caller and implementing the checked arithmetic remains
future work. This packet does not alter their estimate functions or records.

H's persistent model/frame/args/saved-attempt/runtime owners and staged error
formatting remain its caller term; outer prefix sampling includes stage-line
emission as already reviewed. VR's parsed Case/Model, control/comparison/
record/IO/runtime owners remain its separate global envelope. A generic serde,
stdout/argv/file read or numerical error-format audit is outside this tranche.


## Fixed String and retained caller-model bindings

This section concerns the finite input Strings, not Debug/JSON/IO formatting.
The four installed source pages in STRING_CONTRACTS.json identify the pinned
1.97.1 implementation. alloc/fmt.rs:649-659 starts format! with an estimated
capacity or literal clone. core/fmt/mod.rs:754-806 chooses zero, literal
length, or twice total literal bytes. For these fixed templates literal bytes
are part of final output, so c0<=2L, where L is final UTF-8 length.
String::write_str -> push_str -> Vec::extend_from_slice
(alloc/string.rs:3355-3359,1110-1112), and source_02 RawVec give:

    F(0)=0 for an empty cloned string;
    F(L)=max(8,2L) for nonempty fixed-template formatted input Strings.
    String clone of length L requests exactly L bytes.

F is a retained backing-buffer upper, not an API guarantee of exact capacity.
A growing format output adds only its active old request, bounded by half
its new request. It is not another persistent String owner. Nonempty literal
to_owned/to_string may be exact L; F remains a safe upper where only output
grammar is needed. String::clone directly clones its byte Vec (:2363-2366).
str/string ToString specialization (:3050-3085) delegates through String::from
(:3121-3127) to str::to_owned. alloc/str.rs:246-253 copies as_bytes().to_owned()
and wraps that Vec without another buffer; the source_02 slice/Vec clone
contract makes this exact L. Kernel VR source IDs also use actual
String::clone and are exact L.

H source ID is format!("k6:{global_dof}") with L=3+digits10(g), g<n, no
dynamic width/precision or floating conversion. Use sum F(L_i), or
l*F(3+digits10(pred0(n))), where pred0(n)=0 at n=0 and n-1 otherwise.
This derives an input bound without reading
future capacity. Node/member labels use their fixed one-character prefix plus
decimal index; model id/source lengths are known input strings. No arbitrary
"16 bytes per every String" rule is introduced.

| Retained caller owner | Source construction / capacity in elements or bytes |
|---|---|
| H RF-LARGE nodes | HM:363-370 then406-410: pushed (String,[i64;3]) mapped by owned iterator to same-size48-byte (String,[f64;3]); retain <=G_48(N). Counting both source and destination during model creation is safe if reuse is not relied upon. Each moved label retains <=F(label_len). |
| H RF-LARGE members | HM:389-400 borrowed exact map:64m plus exact cloned label lengths. Builder's40-byte member tuples drop when finish returns. |
| H RF-LARGE restraints/loads | HM:363-388,412-413: G_16(R_nodes)+G_16(l); sort changes no capacity. R_nodes is number of restrained nodes, not r constrained DOFs; R_nodes<=min(N,r). |
| H DEC053 model | HM:509-555: members reserve m exact; restraints push distinct nodes, loads borrowed filter ->G_16(l). Nodes are owned fallible Option<Vec> from a preallocated Vec<Option<[f64;3]>>; use max(G_48(N),N*S_coord) retained bytes conservatively if owned-source reuse is possible, where S_coord=actual sizeof(Option<[f64;3]>). This unbound caller-only layout is explicitly below; never guess it from other tuples. |
| H model strings | id/source: bound by their logical UTF-8 lengths with clone/str conversion class; node/member format labels: F(one-prefix+digits(index)); cloned member labels exact len. |
| H frames | HM:85-96 borrowed fallible collection ->G_136(m), not merely136m. main.rs:574 retains this caller Vec through stage dispatch. It is independent of kernel source/member arrays. |
| VR parsed Model | VC:165-220: node_names G_24(N), coordinates G_24(N); exact borrowed collections for members (112m), springs88(s+d), constraints r tuple slots, loads48l, stations16t, omitted-name Vec24*omitted_count. Text children use logical input lengths and the string conversion class, never raw JSON parser capacities. |
| VR cloned Model | Derived Clone at VC:101 makes exact-length Vec copies and String clones; embedded Case's original Model may remain alongside the clone. SourceParts::nodes clone and source load-ID clone are additional identities, counted by SRC0_VR. |
| Caller frames/model headers | Stack/inline model wrappers do not get an extra heap Box merely because their actual sizeof is known. Their element buffers and String children are the heap owners. |

H's source phase includes adapter construction (HS:52-64), so count one
active formatted source-ID grow and the restrained-index input backing before
PrimitiveSource validation, even though the completed SRC0 formula already
pads all destination fields. This gives direct construction alternatives
rather than treating SRC0 retained bytes as the entire source-phase maximum.
H's model-building temporaries occur earlier and must not be silently added
to every solve stage. Persistent model/frame buffers remain caller ownership.

VR SourceParts nodes clone exactN; all other source fields push from empty.
VC:493-511 preserves the axis/directional split; generator:169-218 explains
existing k=0 omission and RF-CANCEL c<k> terms. No net-load reduction is made.
Model and Case/string/parser ownership in the global metric remains distinct
from the kernel-source descriptor. This proposal does not infer a serde
allocation envelope from serialized bytes or array counts.

### Exact remaining caller cells and completed subcells

| Cell | Disposition |
|---|---|
| C1 kernel SourceParts/PrimitiveSource/PREP source clone | Construction classes and retained request equations now explicit for both fixed callers; the H fixed-format source-ID bound is source-derived. Must be independently reviewed and implemented only under a new grant. |
| C2 future helper populations | U/raw-row, neighbor, body, block, geometry-child and ledger-limb uppers now substitute finite input/pre-solve counts; no future allocation census is needed. Optional histograms tighten rather than change the domain. |
| C2 concrete coverage | All33 H existing structural rows and all213 VR case identities retained.201 embedded model scalar/string metadata is read directly. Original193 factored-case obligation and eight geometry-refusal cases are distinct. No solver/profile/model regeneration performed. |
| C2 exact VR graph numbers | Exact B/b/z/h and histograms for each VR case were not regenerated here. Existing pre-solve count code supplies them; otherwise the displayed input-based uppers are the proposed substitution. Three committed cases already force multiple bodies by N-m>1: RF-MECH-DISC-CHAIN100, RF-MECH-DISC-CHAIN100-SPRING, RF-MECH-LINE-IN-CHAIN1000. Their geometry paths remain covered. |
| C2 external12 VR large inputs | Existing generator and committed model/source hashes are bound; actual future file path/bytes must match before its typed model lengths feed this descriptor. This tranche does not generate/load12 missing model files or invent their full JSON lengths. |
| C1 H DEC053 final-node capacity | Precise conservative owned-map formula retains S_coord=sizeof(Option<[f64;3]>) as an accessible, currently unmeasured caller-type fact, or requires a separate proof that the selected iterator cannot reuse. Kernel SRC0's exact borrowed node map does not depend on this caller-layout gap. No new witness is run/requested automatically. |
| C1 remaining caller tuple strides | VR Model constraints use actual (u32,usize) elements; capacity count r is exact, but its nominal tuple byte size was not one of the five layout08 targets. Keep it named if closing caller bytes. No substitution by a similarly sized measured type. |
| C1 source validation/refusal | Raw input counts bound constructor arrays and partial failures; successful-source algebra f=n-r/U/q requires validated uniqueness/ranges. SourceError formatting and caller stop branches remain finite W1 terms, not dropped because all common cases validate. |
| O2 / W1 | Reconcile final A1 source and independent full-bound composition. H staged source/solve/prefix versus VR global parse/count/control/solve/record/RCM/parity remains unchanged. No runtime/serde/IO programme or acceptance is implied. |

The source-side capacity/classification rows and global inequalities are
proposed conditional bindings. Old estimator constants, current admission
meaning and committed records are untouched. This is deliberately a finite
caller map with exact remaining cells, not a universal process theorem.
