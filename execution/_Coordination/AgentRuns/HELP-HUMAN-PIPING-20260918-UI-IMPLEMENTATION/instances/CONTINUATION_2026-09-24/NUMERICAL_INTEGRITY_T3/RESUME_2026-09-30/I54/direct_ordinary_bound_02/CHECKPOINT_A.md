# I54 ordinary bound — checkpoint A

Pinned implementation: 24af17c47011608d187811024b51807395e4e526. Reuse I54/f3f6fd1f2e container laws only within RV73's accepted qualifications (review manifest seal 2f7bd352f7). This checkpoint is a source expression skeleton, not a complete numeric profile.

For actual phase e, the requested ordinary/capture expression is

    Req(e) = Raw + Typed(e) + Basis(e) + Boundary(e) + Diagnostics(e)
             + Case(e) + Active(e) + Suffix(e)
             + Observation(e) + OldSourceCapture(e) + TracePending(e).
    Move(e) = Req(e) + OldBackingsNotAlreadyCounted(e).
    OrdinaryWindow = (max_e Req(e), max_e Move(e)).

Each term is an allocation-token union; moves preserve identity, deep clones add a token. Observation/old-source capture reuse the accepted P1/W1 roster. TracePending is a separate **unpriced** supplied term until I51's final typed source is pinned; ROOT's shape message is not a layout fact. This packet binds the ordinary terms, not future trace/native/proof/receipt terms.

## Executable primitive operations and exact inputs

All arithmetic is checked. `S[T]` and `A[T]` are actual final-build type size/alignment parameters. `Exact(T,n)=S[T]*n`; `Push(T,n)=S[T]*PushCap(S[T],n)`; `Append(T,c0,n)=S[T]*max(c0,min_nonzero(S[T]),2n)` for n>0 and the reviewed construction premise. `Nested(T,r,c)=S[Vec<T>]*r+S[T]*r*c` prices a vec![vec![x;c];r]. Hash and tree functions are precisely the reviewed I54 HB/T rows, with actual K,V layout and construction history. Add children separately.

Required structural inputs for the supported one-case ordinary straight family: node count n, member count m, scalar spring entries s, raw and unique rigid constraints r/k, support count g and restraint-child total, primitive nodal contributions l with source-ID byte lengths, material/point/section populations and child string/capacity facts, selected modulus-basis presence, and mode. `N=6n`, `F<=N`, `Z<=min(N*N,144m+s)`, `C<=144m+s` ordinary full-matrix assembly contributions, lower-entry bound E<=78m+s, skyline H<=F(F+1)/2. These ordinary C/E are distinct from the retained source/native row counts. An exact pattern census is unnecessary for the conservative Z/H upper.

Current complete BorrowedValueFacts gives a source-evaluable raw-resident upper:

    Raw = S[Value]*array_capacity_elements
          + key_capacity_bytes + string_capacity_bytes
          + TreeNodeMax(String,Value)*(objects + floor(object_entries/5))
          + captured_digest.capacity.

The root Value and capture headers are inline/stack here, not separate allocations. Current eleven top-level typed Vec facts do not include nested String/Vec/map/Value children. A bounded borrowed nested typed accumulator is indispensable unless the exact source construction history replaces those facts. This is specifically a capacity gap, not a request for a whole new census graph.

## Actual phase transitions

| Phase | Ordinary owners that remain / active allocation |
|---|---|
| Capture and typed parse, before observer | CapturedInvocation::parse owns raw Value, checked text/hash clones and the typed conversion; lib.rs:2204–2220 creates census only after it returns. No ProductCapture overlaps the earlier parse workspace in this implementation. Raw capture and typed owner persist. |
| Normalization / early observer | lib.rs:2283–2347 moves request.model; request materials move or model.materials is cloned. Normalization/shared-section work precedes normalized(). The early observer allocates node/material copies. It survives all later ordinary phases. |
| First build/boundary | lib.rs:2364–2438 builds FrameNode/StraightPipeElement/FrameElement, section HashMap and LinearSupport children; prepare_boundary allocates restrained/spring vectors. Original typed model/materials remain. Hash maps in build are separate temporary maps. |
| Basis assembly | FK sparse.rs:615–702 simultaneously holds formed m Matrix12 records, connectivity, spring DOFs, pattern-construction neighbors/rows and then pattern+values. Temporary formed matrices are real, despite sparse publication. Each admitted stiffness has pattern row_starts/columns/transpose and values backings. |
| Basis cache and selected material basis | lib.rs:2458–2607 retains default materials.clone(), default BuiltModel and stiffness. A selected temperature/modulus basis adds a second materials/BuiltModel/stiffness owner even for one case; at most2 bases for the first single-case family. No “one case means one matrix” reduction. |
| Case assembly/reduction | lib.rs:3416–3626 retains primitive loads, LoadApplication, pipe/material maps, ledger→AssembledForce children, prescribed/prescribed_values, SparseReducedSystem and observation_force. Force formation and stiffness are distinct. ExactAccumulator has inline fixed 68-limb magnitudes (no child Vec). |
| Typed structural solve | lib.rs:5437–5503 constructs SparseAssemblyEvidence, which **clones** the sparse pattern and frame primitives. Its construction also owns four Z arrays before retaining two (adapter:475–535,1021–1109). Dense branch additionally owns dense K+roundoff+counts (adapter:707–727); sparse branch retains sparse prepared arrays (FK sparse:1298–1420). Both include contribution Expansion children/audit and actual report/fidelity/formation results. |
| Legacy sparse observation | lib.rs:5504–5551 runs after typed structural solve while its checked StructuralSolution survives. It forms ReducedSparseEntrySystem, RCM adjacency/order, both original and ordered skyline profiles, factor clone and solve vectors (sparse_direct/lib.rs:700–744). Guard is only 24*identity_profile_entries <=6GiB; it does not price these other owners. |
| Existing source recovery / W2 | lib.rs:3651–3888 can materialize N*N recovery stiffness only for N<=256, then attempt the existing exact-block method; a range-triggered ordinary refusal can reach W2. These are reached after early observation. Their actual typed outcomes/retained owners must remain in the branch max; no “ordinary success” inference from input counts erases their prefix. Existing retained numerical subprofiles are separate named inputs, not zero. |
| Dense-only later observation | lib.rs:3922–3960 additionally materializes dense view, dense reduced system and legacy dense observation when DenseScrutiny + formed + no W2. Dense view drops explicitly before legacy solve, but reduction and main linear result remain. |
| Recovery / maximum / late capture | lib.rs:4016–5011 builds complete displacements, pipe/support results and preview CaseRecord. Each straight maximum is called serially; supported no-uniform-load family has two boundaries/one span. Heap node high water<=131073; actual Node stride remains a build fact. At prepared_case_source(:5008), BuiltModel/basis stiffness, force/reduction/linear reports, loads, maps, result vectors and preview records have not yet dropped. |
| Preview and aggregation suffix | lib.rs:2615–2814, preview_physics.rs:528–714: preview takes CaseRecord, holds old rows while building replacement support/max rows, and emits a separate preview JSON tree. Aggregation creates id_map and rows_by_base_id's deep ResultItem clones. Those clones, basis states and support maps survive envelope construction and observer.finish. Rendered::evidence json! borrows preview_cases, so original preview tree and envelope copy coexist (preview_physics:183–184). |
| Error returns | blocked_envelope consumes model/diagnostics while outer lexical owners can still be alive; it creates quality/formulation/empty evidence. Error formatting and partial allocated prefixes must be included, not treated as an empty final result. FrameKernel/Structural error variants with vectors remain owned through their actual conversion/drop. |

The current DenseScrutiny guard is 96*N*N<=6*1024^3; for the named n=2 input it compares **13,824 bytes** to6GiB. This is the existing guard value, not an ordinary memory total. The observation guard compares at most24*45=1080 bytes for F=9. Both guard branches remain in the expression; numerical success is not proved by those comparisons.

Named input: fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json; immutable retained_product_tests.rs:123–125 supplies it, :2584–2614 uses PreparedCase::prepare_observed in both SparseInteractive and DenseScrutiny. Counts from the file: n=2,m=1,g=4,s=3,k=3,l=3,N=12,F=9; no selected modulus basis, no distributed/thermal/pressure loads/components/nonlinear supports/combinations. Source connectivity gives Z=144,C=147,d_max=2,E<=81,H<=45. No runtime was run to obtain these facts.

Next output replaces the remaining ordinary family placeholders with closed subterms where source permits and gives first-case arithmetic plus the exact layout/accessor/branch-profile residuals. The broadest remaining issue is not a container law: actual nested typed capacity facts and finite structural report/formation/failure payload types must be bound to the final build. No observed DWARF scalar is installed as a production coefficient.

## ROOT precision follow-up

The final BOUND domain starts after CapturedInvocation::parse/assess; earlier parse workspace is not claimed to fit. For a rectangular zero-row constructor, add the evaluated inner-row temporary when c>0; square zero-dimensional matrices have zero inner capacity. ROOT permits a clearly labelled conservative sum of distinct phase maxima when exact peak intersection is costly. BOUND applies that permission and includes the existing non-selected FinalizedSourceBlockCase separately from future C3.
