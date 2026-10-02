# Sparse descriptor interface and finite substitutions

Frozen40129 inputs/source; equations only. No graph, model, matrix, profile,
ordering or solver was constructed. source09 input identities/populations remain
the external basis; source11 fixes the actual dense=false call path.

| Descriptor | Source / usable bound |
|---|---|
| N,m,s,r,n,f | Existing Model lengths; s counts Some(axis) springs; r is raw constraint count; n=6N checked. Use f<=n before valid-map premises; actual binary64 free count is n minus distinct restrained DOFs. A directional spring returns before assembly; its branch stays covered. |
| C | Exactly144m+s on successful contributions, V106-128. Matrices are12x12 inline data and both triangles are appended even when a coefficient is0. |
| z | SparsePattern connectivity stored count; <=min(n*n,C) with checked products. This is structural storage, not numerical nonzeros. |
| a_i,b_v | a_i is each global stored row length (sum z); b_v is pre-dedup neighbor insertion population2*incident members, sum4m. No dedup-based shrink. Optional; total bounds below avoid recomputing these arrays. |
| zf,L | Prepared stored free pattern and its stored lower part, including zeros. zf<=min(f*f,z), L<=zf; after diagonal/symmetry proof L=(zf+f)/2. S1257-1264 explicitly includes zeros. |
| d_i,e | Nonzero strict-lower prepared coefficients create undirected adjacency. sum d_i=2e<=zf (tighter zf-f on successful diagonal basis); d_i<=max(0,f-1). These are not the retained kernel's free-neighbor descriptors. |
| hs | SD skyline from first_columns after its value-sensitive RCM order. hs<=f(f+1)/2, checked. Its temporary contiguous values resize allocation and later exact per-row factor allocations differ in capacity. Use an existing proved hs upper or this algebraic ceiling, not an observed peak or W1 h. |
| t_e,tau,T | t_e counts submitted contributions at entry e, sum t_e=C. Safe tau=max t_e<=C,T=max_row sum t_e<=C. Sharper input-only bounds below require only already-validated distinct element endpoints. |
| j_i,k_i | Stored free / prescribed couplings per global free row. j_i<=f, k_i<=min(r,n), on the validated source. Used to size filtered row buffers before pushes or zero-product folds. |
| map node requests | Actual BTreeMap<(usize,usize),(f64,usize)> leaf/internal sizes, with source02 insertion population bound. This is the existing V-JNODE cell; public map header size is insufficient. |
| public strides | sizeof of the exact public types/tuples listed in DERIVATION. Vec headers inside outer buffers are sizeof(Vec<T>); wrappers on the stack are not separately heap-allocated. |
| Expansion stride | Existing source05 actual nominal FK.Expansion32 under its recorded build basis; reconcile final artifact. No copied private struct or new layout probe. |

Finite aggregate substitution: if k_i>=0,sum_i k_i=K over M slots, both
`sum G(T,k_i)` and `sum A(T,k_i)` are at most
`sizeof(T)*(2K + mu(T)*M)`. This follows P(k)<=max(mu,2k) and
max(mu,2k)<=mu+2k. At k=0 the real request is0; the formula deliberately pads
that slot. Sum operators may therefore be removed without creating a graph.
For usize/f64 mu=4 on the pinned source:

```
sum_v A(usize,b_v) <= 64m+32N
sum_i A(usize,a_i) <= 16z+32n
sum_e G(f64,t_e) <= 16C+32z
sum_e G(f64,t_e+1) <= 16C+48z
sum_i G(usize,d_i) <= 16zf+32f
sum_i G(usize,2d_i) <= 32zf+32f
Round child clones <=8(2C+z)
```

These sums are retained-byte uppers; each growth edge still gets only its one
active-old surcharge. For maximum terms use the declared maximum count, e.g.
max_e G(f64,t_e)<=G(f64,tau), max couplings<=G(tuple,min(r,n)), and c<=f for
one component's RCM scratch. Stable sort uses actual logical length, or a proved
upper on it, while the backing retains its larger pre-dedup capacity.

Sharper tau/T without a graph: FrameElement::new rejects identical node indices
(FK/lib.rs569-572). Its12 global DOFs are distinct, so each matrix contributes
at most one scalar to a given global entry and at most12 to a given global row.
Consequently `tau<=m+s` and `T<=12m+s` on successful adapter/contribution paths.
No numerical coefficient or coalescing outcome is needed. A failed adapter or
expect/panic path uses its separate prefix/format envelope. These improvements
can tighten X(1+2T), HSums and HDiff without deleting zero contributions.

All descriptors are bounded source/control facts. Their numerical substitution
is not a claim that a case solves, a prediction that its profile attains a
ceiling, or a new admission decision. If a coarse valid hs bound is too large,
report that exact usefulness gap; do not silently narrow the supported cases.
