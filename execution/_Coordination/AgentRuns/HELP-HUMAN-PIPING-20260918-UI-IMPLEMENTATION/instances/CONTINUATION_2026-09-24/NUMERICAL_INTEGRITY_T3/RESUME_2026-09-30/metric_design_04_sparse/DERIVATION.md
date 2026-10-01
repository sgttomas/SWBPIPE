# V-SPARSE — finite sparse-parity source envelope

**Conditional symbolic envelope; no E_max/admission/implementation acceptance.**
Source40129a225d73860ac2a53da9a2fa73869df668f3. V=VR/src/parity.rs;
S=FK/src/structural/sparse.rs; F=FK/src/structural.rs; D=SD/src/lib.rs;
DS=SD/src/structural.rs. Paths/hashes are in SOURCES.json. This adds source
bindings to source11/vr_caller_07; all earlier sealed packets stay unchanged.

## Reached branch and descriptors

V200-247 calls binary64_model, contributions, allowances, unscaled assembly,
SparseStructuralSystem::new, solve_sparse_structural and class, then returns
because dense=false. No dense branch executes. Actual axes/directional springs
and every existing case remain in the domain: a directional spring returns
its named adapter error at V78, not a fabricated zero axis-spring count.

SparseAssemblyOptions::new fixes UNSCALED (S484-487); users/blocks are empty.
V92 sets every prescribed value to+0.0. DS104 -> S1275 fixes ForceBinding::Legacy;
prepared.formation=None at S1458. Thus this route does not execute force-scaling
copies, load-fidelity or formation-check helpers. Coupling-index vectors still
exist; zero prescribed products do not erase their allocation. The ordinary
contribution and intended-action audits both run when reached.

Use N nodes,m members,s axis springs,r raw constraints,n=6N,f actual free DOFs.
Do not compute f=n-r until uniqueness is validated; f<=n is a safe earlier bound.
Define C=144m+s contribution entries, z global structural stored entries,
zf prepared stored free entries, L stored lower entries including the diagonal,
e nonzero strict-lower prepared entries, d_i their undirected degree,
hs=sum_i(i-first_i+1) under SD's actual order. Use t_e for the count of original
contributions at global entry e (zero-valued contributions included), T=max_row
sum_e t_e, and k_i stored prescribed couplings in global row i.

Source-only fallbacks, checked arithmetically without building any graph:
`z<=min(n*n,C)`, `zf<=min(f*f,z)`, `L<=zf`, `2e<=zf`, `d_i<=f-1`,
`hs<=f(f+1)/2`, `sum t_e=C`, `T<=C`, `k_i<=min(r,n)`.
Validated distinct element endpoints also give max t_e<=m+s and T<=12m+s
without constructing a graph; DESCRIPTORS.md proves these tighter substitutions.
Successful preparation has each diagonal, so tighter `L=(zf+f)/2` and
`sum d_i=2e<=zf-f` may be used after their premises are bound. Structural counts
can come from the existing input/count basis; hs,d_i/e depend on the prepared
binary64 values (including numerical zeros/projection), not W1's retained
structural graph or skyline h. No h value is copied from the retained kernel.
A dense-size algebraic upper is not a dense allocation or a guarantee of admission.
A useful tighter hs remains a named descriptor dependency when the coarse upper
is too large for the existing rule.

**Correction to source01's abbreviated ordering row:** S1257-1264 lower_entries
explicitly includes stored zeros. DS60-63 collects all L entries before D487
filters numerical zeros for adjacency. The entry Vec must use L, not e or a
count of nonzero lower entries. This correction changes no source/test behavior.

## Request operators and outer ownership

E(T,k)=k*sizeof(T). G(T,k)=sizeof(T)*P_T(k), with P(0)=0 and
P(k)=max(mu(T),next_pow2(k)) for fresh pushes/lower-zero borrowed collections.
A(T,k)=0 at k=0, otherwise sizeof(T)*max(mu(T),2k), for a finite fresh generic
collect/extend/resize path. Use source02's exact recurrence if starting capacity
is neither zero nor the named exact constructor. Clone requests logical length,
not old capacity. Dedup/pop/clear do not shrink backing automatically.

O_G(T,k)=0 when k<=mu(T), else G(T,k)/2 is a fresh push-vector active-old upper.
For A paths O_A(T,k)<=E(T,k) at an actual grow, since old capacity<required<=k.
Write epsilon=0 for ordinary requested bytes and1 for growing-realloc move bytes.
Only the buffer currently reallocating receives its O term. Distinct old/new
replacement Vecs are simultaneous owners regardless of epsilon. Source02 governs
tree insertion and stable-sort scratch `Sort(T,k)=0 for k<2`, otherwise
`sizeof(T)*max(k,48)`. Integer/tuple-key comparators allocate no children.
The installed sort_unstable/by_key API explicitly allocates no scratch; its
relevant source/doc excerpt is SORT_UNSTABLE_SOURCE.json.

Public sizeof expressions suffice for FrameNode,FrameElement,Matrix12,
(usize,usize,Matrix12),(usize,usize),(usize,f64),(usize,f64,f64),
StiffnessContribution,SymmetricMatrixEntry,PivotEvidence,ResidualRow,
ContributionRounding and Vec headers. The existing source05 actual nominal
Expansion stride32 is reused conditionally on its final build binding, not
inferred from a mirror. SparsePreparedSystem,ProfileFactor and their wrappers
are unboxed locals: do not charge sizeof(wrapper) as a separate allocation.
All8/4/1 coefficients below use the existing target usize/f64/i32/bool strides;
use public sizeof expressions in an eventual checked implementation.

Let Late be source11's actual surviving Case/Model/Args/hash/runtime, record2,
not_covered_full and OnceLock owners at V500. W1 Outcome/caches/publication/radius
have already dropped and are absent. Late is paid once outside this envelope.

## Adapter, contribution maps and sparse assembly

Binary64Model retained children:

```
B = G(FrameElement,m)+G((usize,f64),s)+8n+G(usize,f)+E((usize,f64),r)
HB <= B+G(FrameNode,N)+8r
      +max(Sort((usize,f64),r), epsilon*O_G(FrameNode,N),
           epsilon*O_G(FrameElement,m), epsilon*O_G((usize,f64),s),
           epsilon*O_G(usize,f))
```

This explicitly pads completed output B early and keeps node/restraint arrays
through their last possible overlap. Nodes/frames are borrowed fallible collects;
free is a fresh filter. Prescribed tuples are exact borrowed maps then stable
sorted. Nodes and restrained arrays drop at adapter return. Adapter error String
construction must overlap the appropriate partial prefix (V-FMT interface below).
Frame constructors/Matrix12 arithmetic/transform_roundoff use fixed arrays and
scalar typed errors; matrices are inline tuple data, not144 separate heap owners.

Contrib=G(StiffnessContribution,C); construction is Contrib+epsilon*O_G(...,C).
Let I(z) be source02's insertion upper for the **actual**
BTreeMap<(usize,usize),(f64,usize)> nodes, including split/root construction.
No key/value child heaps occur. Its private leaf/internal requests are V-JNODE,
not guessed from the public map header. Allowance build peak<=2I(z): `out` and
`magnitude` coexist, with magnitude nodes permitted to remain through consuming
iteration (V134-164). Return owns only I(z); matrices/evidence are stack arrays.

For the connectivity-pattern constructor define pre-dedup node neighbor pushes
b_v=2*incident_member_count(v), sum b_v=4m, and final global row lengths a_i,
sum a_i=z. Optional exact arrays may be replaced by input-only upper counts.

```
Neighbor = E(Vec<usize>,N)+sum_v A(usize,b_v)
Rows     = E(Vec<usize>,n)+sum_i A(usize,a_i)
Pattern  = 8(n+1)+A(usize,z)+8z
HPattern <= Pattern+Neighbor+n+Rows
            +epsilon*max(max_v O_A(usize,b_v),max_i O_A(usize,a_i),O_A(usize,z))
K = Pattern+8z
Form = E((usize,usize,Matrix12),m)+E((usize,usize),m)+8s
HAssembly <= Form+max(HPattern,K)
```

S90-129 retains deduplicated neighbor *capacities*, spring bool[n] and all row
arrays while compressed columns are constructed. The row IntoIter may retain
outer backing and the current row while columns grow. The formula deliberately
pads all row children until compression completes. Transpose is exact z slots.
S615-694 retains formed matrices/connectivity/spring-DOFs until assembly returns;
Pattern's construction locals have ended before values[z] are allocated. K is
the returned SparseStiffness heap. These alternatives do not sum all phase maxima.

s_round and s_count each retain G(...,z), with only one active old buffer during
their interleaved pushes. SparseStructuralSystem::new adds an exact usize[n]
prescribed-position map (S897), which persists through solve/class. Define:

```
W = B+Contrib+I(z)+K+G(f64,z)+G(usize,z)+8n
Hfront = max(HB,
             B+Contrib+epsilon*O_G(StiffnessContribution,C),
             B+Contrib+2I(z), B+Contrib+I(z)+HAssembly,
             W+epsilon*max(O_G(f64,z),O_G(usize,z)))
```

W remains the sparse caller prefix until parity returns, including after solve
returns. It is not added again inside the following helper equations.

## Expansion replacement, preparation and contribution audit

For an Expansion that has received at most k scalar add operations, length<=k
and returned child<=G(f64,k). Every add creates a fresh `next` Vec and only then
assigns self.terms=next (F712-735). Therefore a current add peak is bounded by

```
X(k)=2G(f64,k)+epsilon*O_G(f64,k)
Sums=E(Expansion,z)+sum_e G(f64,t_e)
HSums=Sums+max_e[G(f64,t_e)+epsilon*O_G(f64,t_e)]
Diff=E(Expansion,z)+sum_e G(f64,t_e+1)
HDiff=Diff+max_e[G(f64,t_e+1)+epsilon*O_G(f64,t_e+1)]
Round=G(ContributionRounding,z)+8*sum_e(2t_e+1)
RoundClone=E(ContributionRounding,z)+8*sum_e(2t_e+1)
```

HSums pads the old terms in Sums and adds the distinct new terms; epsilon is
only the new Vec's realloc-old surcharge. This is not equivalent to charging
one realloc on the old Expansion buffer. Deep clone of sums initially copies
logical child lengths exactly; Diff pads those and subsequent replacement
capacities. ContributionRounding rows clone two child slices exactly, and Round
pads those children before their outer Vec push (S1529-1548). RoundClone is a
later independent deep clone, not another source expansion identity.

ExactAccumulator::round helpers use fixed68-limb arrays and scalar iteration
(exact_sum.rs45-48,69-200,303-308): no registered helper heap or per-limb Vec.
Since all prescribed values here are0, S1591 add_product returns immediately
(F743-745); delta's heap stays empty, but the filtered coupling-index Vec stays.

```
HAudit = max(HSums,
             Sums+HDiff,
             Sums+Diff+Round+epsilon*O_G(ContributionRounding,z),
             Sums+Diff+Round+max_i[G((usize,usize),k_i)
                                  +epsilon*O_G((usize,usize),k_i)])
Pcore=4f+8(f+1)+G(usize,zf)+G(f64,zf)+G(usize,zf)+8zf+8f+8f+BASISlen
P=Pcore+Round
```

Pcore's terms are scale_exponents,row_starts,columns,values,source_entries,
transpose,diagonal,rhs and one symmetry_basis String. BASISlen is V's fixed
BASIS.as_bytes().len(), an exact str clone, not an arbitrary format allowance.
P retains only Round after the audit; Sums/Diff/coupling scratch drop. The new
sums in a later intended-action audit are distinct sequential owners.

Preparation validates before arrays (bool[n]); negative-original-diagonal
refusal at S1318 adds global direction8n while partial exponents4f exist.
Otherwise free_position8n survives **through** HAudit until preparation returns.
Let j_i be stored free entries in global free row i. Then:

```
RowPrepare = max_i( G((usize,usize),j_i)
                   +epsilon*max(O_G((usize,usize),j_i),O_G(usize,zf),O_G(f64,zf)),
                   G((usize,f64,f64),k_i)+epsilon*O_G((usize,f64,f64),k_i) )
HPrepare = max(n, 4f+8n, Pcore+8n+max(RowPrepare,HAudit))
```

Rows' filtered entry backing remains through column/value/source-entry pushes;
it drops before prescribed_couplings is built. The two couplings types have
distinct strides. Their unstable sort adds no heap. RHS follows the legacy
zero-product fold, so the unused general exact_scaled_rhs route adds no heap
owner here. Typed failure unwinds a prefix covered by these complete paddings.

## Ordering and SD RCM, including six simultaneous eccentricity buffers

Entries=A(SymmetricMatrixEntry,L); Adj=E(Vec<usize>,f)+sum_i G(usize,d_i).
Do not omit explicit-zero entries from Entries. Raw adjacency has d_i elements;
SD symmetrizes it again, so each neighbor backing grows from2d_i pushes before
dedup, and **retains that capacity** (D508-525):

```
RCMbase=E(Vec<usize>,f)+sum_i G(usize,2d_i)+8f+f+8f
```

The last terms are degrees,visited and exact-reserved returned order. Padding
these completed arrays during earlier neighbor construction is intentional.
Apply source07 QueueRet(c),QueueOld(c),ReachOld(c) for component c<=f:

```
Reach = f+G(usize,c)+QueueRet(c)
        +epsilon*max(ReachOld(c),QueueOld(c))
Ecc   = f+6G(usize,c)+epsilon*O_G(usize,c)
MainQ = QueueRet(c)+epsilon*QueueOld(c)
HRCM  = RCMbase+max(epsilon*max_i O_G(usize,2d_i),
                    max_i Sort(usize,d_i),max_components(Reach,Ecc,MainQ))
```

Six Ecc identities: caller component and saved last_level; callee current_level,
old last_level,next_level and the new exact clone created before assignment at
D618. Fresh next growth alone gets O_G; copying its clone is an independent
allocation already included. Reach begins with vec![seed] capacity1, so its
special ReachOld from source07 must not be replaced by the empty-Vec old formula.
Only one queue/one component's local buffers exist at once; iterations do not
accumulate them. Stable degree-sort scratch is a separate phase. Empty maxima
are0, including f=0/component-free cases.

Profile temporary construction D215-275 validates with bool[f], then retains
ordered_position8f,first8f,row_starts8(f+1),resize-built A(f64,hs), and an exact
order clone8f. Its values must be counted even though ordering discards them.

```
HProfile=max(f,24f+8(f+1)+A(f64,hs)+epsilon*O_A(f64,hs))
HOrder=max(Entries+epsilon*O_A(SymmetricMatrixEntry,L),
           Entries+Adj+epsilon*max_i O_G(usize,d_i),
           Entries+Adj+HRCM,
           Entries+Adj+8f+HProfile)
OrderReturned=16f
```

Original entries/adjacency and the RCM output order stay through profile build.
Only order and first_columns survive DS70-75. Profile values/row_starts/
original_indices, entries and adjacency drop before FK factor construction.
The symbolic hs profile is not a dense n*n matrix and is not W1's h.

## Factor, condition, refinement and successful output

```
Fct=E(Vec<f64>,f)+8hs+16f+G(PivotEvidence,f)
HFactor=max(n, Fct+16f+f+8f+8f+8f+epsilon*O_G(PivotEvidence,f))
```

Fct owns exact individual skyline rows, outer exact f headers, factor first/order
clones and pushed pivots. HFactor adds caller order/first16f, seen[f],position8f,
the factorizer's additional first clone8f and work8f (S1748-1795,F1999-2001).
Partial factor buffers drop before the fallback negative-pair search. The
caller ordering drops when factor_sparse_structural_ldlt returns.

Profile solve F2029-2053 allocates exact x[f] then exact result[f]: `HSolve=16f`,
returned storage8f. This map collect is infallible over a borrowed exact-size
iterator; an outer Result return does not make it fallible collection capacity.
Let PivotCopy=E(PivotEvidence,f), cloned once at S1806 and later moved into report.
The original G(PivotEvidence,f) in Fct remains a distinct live allocation.

Condition extra `HCond=max(n,40f)`: validation's bool[n] is sequential with
numeric work. At the largest inner solve, x,y,signs are alive plus the inner
x/result pair (five8f buffers). The alternate safeguard has x,alt and an inner
pair (four); five iterations are sequential. No factor-work Vec survives here.

Let RR=G(ResidualRow,f), ORR=O_G(ResidualRow,f). The original-residual helper
validates first then builds RR: `HResidual=max(n,RR+epsilon*ORR)`.
Its iterators are unboxed; its fixed ExactAccumulator is inline. All zero
prescriptions make its optional exact-numerator path false, without eliminating
its row outputs. For intended audit let Krow=1+2T:

```
HIntended=max(HSums,
               Sums+RR+max(X(Krow),G(f64,Krow)+epsilon*ORR))
```

Each coefficient Expansion has<=t_e terms. Residual starts with one force add;
each coefficient term can cause two scalar adds through add_product. The old
row residual and its fresh replacement overlap. Residual remains while RR grows,
so RR's realloc-old and the replacement's realloc-old are alternatives, not
concurrent charges. The prepared Round stays separately in P throughout.

Define FinishExtra as the maximum of:

```
n, HCond, HSolve,
8f+8n+HResidual,
8f+8n+RR+G(f64,f)+epsilon*O_G(f64,f),
8f+8n+RR+G(f64,f)+HSolve,
8f+8n+RR+HIntended,
8f+8n+2RR+4f+RoundClone+BASISlen
```

The loop's y8f,full-u8n,original residual RR and pushed correction G(f64,f)
survive correction solve. Returned delta backs IntoIter until its update loop
ends. They drop at each iteration boundary, so three corrections do not require
three accumulated u/residual/correction sets. On success, original/intended row
Vecs and u move into output; they are not cloned. Only scale_exponents,
contribution_rounding and symmetry_basis are copied at that return, alongside
the already present PivotCopy (F1750-1779).

```
HSolver=max(HPrepare,
             P+HOrder, P+HFactor,
             P+Fct+PivotCopy+FinishExtra,
             P+max(n,8f+8n))
Solution=8n+4f+PivotCopy+2RR+RoundClone+BASISlen
```

The final alternative is the negative-pair search after factor failure:
validation bool[n], then free direction8f and mapped global direction8n coexist
(S1947-1970,1918). The returned error retains only its global direction. Early
prepare's direction is already in HPrepare. Numeric range/pivot/input/asymmetry
errors contain only scalars/static strings. The reached SD errors also have no
heap children and map to static StructuralError reasons. No physical mechanism
or numerical-success assertion follows from this memory accounting.

After solve returns, P/Fct/PivotCopy's temporary ownership is gone except the
moved/copied output identities in Solution. During V241 class(), W and either
Solution or a global error direction remain. Class labels for NegativeEnergy/
Mechanism do not serialize their directions. At parity return **all W and solver
output drop**; only Parity.sparse String (or early error String) escapes.

## Composition and remaining cells

With HClass the separately bound class/error formatting extra:

```
H_VSPARSE=max(Hfront, W+HSolver,
               W+max(Solution,8n)+HClass,
               each reached early-error-prefix+its V-FMT extra)
E_future_at_this_phase = Late + H_VSPARSE
```

This includes the retained record2/not_covered_full/OnceLock Late from source11,
not the dead W1 result. The returned class/error String must then enter the
later JSON/emit phase under source11, rather than carrying W past parity return.
No pre-cut/global-prefix, admission or RSS obligation is replaced.

The remaining exact integration cells are:

1. The allowance-map private requested node sizes I(z) remain V-JNODE unless
   an independently accepted later packet binds this exact specialization.
   Public outer map sizeof is not a node request. Other public sizeof terms can
   be evaluated directly in a future authorized estimator; Expansion32 retains
   its existing actual-type basis/final-build reconciliation requirement.
2. Supply existing input/structural descriptors or explicitly choose the checked
   conservative upper per field. L must include stored zeros. hs must be the
   SD numeric-profile descriptor or a proved upper valid for its possible order;
   copied W1 h or an observed factor footprint is not sufficient. No graph was
   built in this tranche. Distribution terms can stay symbolic or use the total
   alternatives in DESCRIPTORS.md; no model was generated.
3. HClass, adapter map_err Debug and `.expect` panic/error formatting remain
   V-FMT/runtime. In particular valid FrameElement construction alone does not
   prove local_stiffness/transform_roundoff cannot later refuse: V109/V142 use
   expect. Their failed prefixes are included here, but their formatting/abort
   machinery is not silently assigned zero. Typed-refusal and successful paths
   have finite buffer envelopes regardless of whether a successful solve occurs.
4. Check library/target/source/normal feature correspondence, checked arithmetic
   and independent owner-union review before translation. The production
   mutation-controls/test branch is not assumed enabled. Source11's other
   V-PRE/IO/serde/parse terms remain separately owned.

No new probe/layout measurement, numerical solve, dense allocation, fixture
restriction, empirical allowance or whole-process acceptance was introduced.
