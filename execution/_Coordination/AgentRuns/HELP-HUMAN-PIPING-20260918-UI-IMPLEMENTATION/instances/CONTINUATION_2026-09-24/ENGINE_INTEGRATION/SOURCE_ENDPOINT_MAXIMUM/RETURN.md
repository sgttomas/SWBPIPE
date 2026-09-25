# Retained endpoint circular-normal maximum draft

TASK `/root/physics_resume/source_endpoint_maximum` returns to actual parent
`/root/physics_resume` (WORKING_ITEMS), using delegated-harness-native execution.
No descendants were created. This is a bounded implementation draft and author
verification, not independent review, source qualification or a native witness.
Only this evidence/draft directory was written; production and dependency source
remained read-only. No Cargo, rustc, npm or native build was run.

The integration candidate is [endpoint_maximum.rs](endpoint_maximum.rs). It is a
single private module with seven Rust test groups, no dependencies, no public
certificate/Deserialize implementation, and no semantic-table or identity edits.
Parent owns integration after NUM's freeze and independent review of the actual
joined candidate. ROOT's selected basis remains
`retained_source_endpoint_normal_max_v1` under `physics-source-1`, in maximum-row
`metadata.basis` and matching maximum evidence.

## Exact source hookup and prerequisites

Call `endpoint_maximum(As, Z, [[Ni, Myi, Mzi], [Nj, Myj, Mzj]])` only inside the
source-owned producer/finalizer after retained source binding. Each argument is
an immutable `ProjectionBounds::from_retained_parts(p.index(), p.value(),
p.interval())` copied from the same `SelectedSourceRecovery.retained()` instance.
The constructor validates numeric shape only; it is internal transport, never
an authority or authenticity check. Copying public values into it is not a
qualified source route. No numeric shape check substitutes for retained replay.

For each `MemberRecovery`, the exact indices are
`section_functional_indices[0][0,4,5]` and
`section_functional_indices[4][0,4,5]`. In Rust, gather components with
`[0_usize, 4, 5].map(|c| member.section_functional_indices[station][c])` for
stations 0 and 4. Resolve each index through the retained projections and verify
its descriptor is the same case/member's `MemberSection` at fraction 0 or 1,
component 0/4/5, `SectionLocal`, unit N/N·m/N·m. Preserve these six indices and
source/functional-plan identity in the derived-row evidence. Returned endpoint
estimates retain the six indices without reconstructing displacement or forces.

These are already positive-x-cut section actions. Do not sign-flip them again.
The retained implementation constructs section My as `-(Mi_y + Vi_z*x)` and Mz
as `-(Mi_z - Vi_y*x)`, with fraction and actual length as separate exact product
operands; N is the same `-Ni` row throughout. Direct endpoint node-on-element
rows use different signs and are not inputs to this candidate. In particular,
do not substitute the j node-on-element row merely because it is expected to
agree numerically with the retained j section functional.

The owning adapter must establish its complete, identified, signed-permutation,
unreleased straight-frame source gate, order≤2 exact blocks, no span load
producers, constant N and affine My/Mz. This extends to explicit rejection of
nonempty pressure-region inventories, including zero pressure, in the composite
route. An interval overlap check on the two N projections detects a contradiction
but does not prove constant N; the actual retained source construction does.

Use positive finite source As and common circular Z from the actual selected
OD/effective-wall section, with the same case/member/material/geometry identity
as retained assembly. The numeric division uses those represented operands
exactly as its mathematical basis. It does not certify primitive geometry,
material accuracy or the computation that formed As/Z. Parent must establish
the circular section assumption and common Z; passing arbitrary unequal-axis
section properties is outside this helper's contract. Materials may differ
between supported members. No material/component library or code-rule input is
introduced.

## Bound and arithmetic warrant

For t∈[0,1], let m(t)=(1−t)m_i+t m_j and constant N. The field is
`F(t)=|N|/As+||m(t)||₂/Z`. The norm triangle inequality gives
`F(t)≤(1−t)F(0)+tF(1)≤max(F(0),F(1))`; the included endpoints therefore determine
the exact maximum of this admitted field. This warrant is source-family
specific; it does not reuse the general polynomial physics coefficient method.

Each signed action interval first becomes its exact absolute-value interval.
Division by the positive represented As/Z is monotone. For each lower or upper
corner, the norm is monotone in its two nonnegative components. Every division,
square, addition, square root and product is rounded outward with next-down or
next-up, except explicit exact zero/one identities. This reuses the checked
`elastic_extrema` interval approach. It directly composes input projection
uncertainty and arithmetic uncertainty; there is no assumption that a primitive
1e-9 projection automatically qualifies this derivative, and no unexplained
64ε/128ε constant is substituted for the actual bound.

The fixed representative order is:

1. `a=abs(N)/As`, `y=abs(My)/Z`, `z=abs(Mz)/Z`;
2. `m=max(y,z)`; exact zero norm if m=0;
3. `u=y/m`, `v=z/m`, `b=m*sqrt((u*u)+(v*v))`;
4. `q=a+b`, then max of the two endpoint q values, choosing i on equality.

Scaling moments by Z before the norm prevents avoidable overflow of a moment
norm whose stress remains representable. The bound follows the same ordered
scaled arithmetic with outward rounding. It never calls platform hypot. The
arithmetic assumptions are the same as the existing interval implementation:
IEEE binary64 round-to-nearest, gradual underflow, correctly rounded sqrt and no
fast-math/reassociation. Any host that does not satisfy them is not qualified by
this draft.

If endpoint stress enclosures are `[Li,Ui]` and `[Lj,Uj]`, the maximum enclosure
is `[L,U]=[max(Li,Lj),max(Ui,Uj)]`. The representative Q lies in it. The returned
absolute error E rounds `max(Q−L,U−Q)` upward. For L>0, the relative error bound
rounds `E/L` upward. Since the actual maximum M≥L, this proves
`abs(Q−M)/M ≤ E/L ≤ 1e-9`. The test is against the unchanged existing binary64
1e-9 criterion. There is no absolute tolerance floor or a denominator such as
`max(1,L)` that would silently weaken relative accuracy near zero.

Exactly zero U proves zero throughout the span; Q=L=U=E=0 and relative-error
field 0 is a documented convention, not division by zero. A nonzero maximum with
L≤0 cannot establish the relative criterion and is refused. Nonfinite
intermediates or outward bounds are refused. Nonzero Q, U and nonzero L must be
normal for publication. A positive action can be subnormal and scale to a
normal stress; that case is supported. Subnormal norm ratios/squares and
underflowed components remain enclosed rather than being treated as exact zero;
they may be harmless under a normal dominating stress. A nonzero maximum that
underflows to zero or remains subnormal is refused. Diagnostic endpoint
enclosures and absolute-error bounds may be subnormal; no separate relative
accuracy claim is made for each endpoint estimate. Checked-JSON transport and
any additional evidence publication limits remain separate parent checks.

Existing MPa stress and summary rows keep their existing conservative range
guards. This new maximum is Pa and does not enlarge those existing rows' domain.

## Witnesses, ties and constant fields

The returned witness is only the endpoint with greatest computed representative
(i on equality). Its endpoint enclosure accompanies it. It is not an exact
argmax claim unless the independent location outcome warrants that claim.

| Outcome | Warrant and meaning |
| --- | --- |
| `StrictEndpoint(I/J)` | That endpoint's lower bound exceeds the other's upper bound. Convexity makes every interior station strictly lower. This is a station result, not a unique circumferential-fiber claim. |
| `WholeSpanConstant` | Both retained endpoint bending components are singleton exact dyadics and equal with sign. Affine interpolation makes the bending vector constant; the already-proved constant N makes the complete field constant. This needs no user boolean or equality of rounded stresses. |
| `EndpointCandidates { exact_tie_proven: true, interior_equal_possible: false }` | Singleton exact endpoint bending components have equal absolute values componentwise but the vectors differ. Their squared norms are algebraically equal; with the same N and section both endpoints tie. A nonzero vector slope gives a strictly convex squared norm, excluding interior equality. Opposite equal moments are the principal control. |
| `EndpointCandidates { exact_tie_proven: false, … }` | Intervals overlap in stress, so both endpoints remain candidates. False means equality is not proved, not that a tie is disproved. Equal computed floats alone do not establish a tie. |

For unresolved candidates, `interior_equal_possible` stays true if both bending
component intervals overlap across endpoints: a constant vector remains possible
from these bounds. A disjoint component interval proves a nonzero affine vector
slope, hence excludes interior equality with the endpoint maximum. Equal source
functionals with nonsingleton projection intervals are conservatively unresolved;
the draft accepts no magic constant-field flag. Parent may later add an
algebraically checked source-descriptor equality warrant under its same owned
proof, but that extension is not needed to make this result honest.

## Actual checks and representative results

`rustfmt 1.9.0-stable (8bab26f4f6 2026-07-14)` parsed and formatted the module;
the final `rustfmt --check endpoint_maximum.rs` passed. The initial formatting
check requested layout changes; it was a formatting-only failure, then repaired.
No Rust typecheck, unit-test execution, Cargo or native claim is made.

`python3 check_reference.py` passed 19 named controls (13 successful, 6 expected
refusals) and 48 reproducible scale controls, evaluating 524 oracle points.
The implementation's Python arithmetic mirror is checked against an independent
mathematical oracle: exact `Fraction` operands and rational squared comparisons
to `|N|/As+sqrt(My²+Mz²)/Z`. No product output rows or the mirror's rounded square
root serve as the reference. Exact source-action box corners, common-N endpoint
combinations and exact affine interior points are included. This is author
verification with an independent arithmetic oracle, not fresh-agent review.

| Synthetic control | Representative and enclosing interval, Pa | Outcome |
| --- | --- | --- |
| N=−10, As=2, no bending | 5; [4.999999999999999,5.000000000000001] | Constant field |
| N=10, As=2, My=3, Mz=4, Z=2 | 7.5; [7.4999999999999964,7.5000000000000036] | Constant field |
| Bending (3,4) → (−3,−4), Z=1 | 5; [4.999999999999997,5.000000000000003] | Proven endpoint tie; midpoint zero |
| Bending (1,0) → (2,0), Z=1 | 2; [2,2] | Strict endpoint j |
| Overlapping projected My around 3, N=1, Mz=4 | 6; [5.999999999939996,6.000000000060004] | Ambiguous; constant possible |
| Exact affine cancellation (2^53+1)−2^53=1, projected bound ±≈1e−12, As=2 | 0.5; [0.49999999999949996,0.5000000000005002] | Projection uncertainty retained |
| Least subnormal N and same numeric As | 1; [1,1] | Normal stress accepted |
| All zero actions | 0; [0,0] | Proved whole-span zero |

Additional controls include 1e−300 and 1e300 scale normal outputs, norm ratio
underflow, subnormal/underflowed maximum publication refusal, overflow refusal,
contradicted axial intervals, excessive projected uncertainty, and primitive
bounds individually within1e-9 whose composed maximum fails1e-9. Rounded-equal
nonconstant endpoint estimates remain candidates without an exact-tie claim.
Detailed inputs/results are in [REFERENCE_RESULTS.json](REFERENCE_RESULTS.json).
The rerunnable reference source is [check_reference.py](check_reference.py).

## Parent completion obligations

Integrate the private module after NUM's exact API/hash freeze, reserve/charge
its bounded finalization work before executing it under the retained invocation
budget, and preserve those charges on failure. This pure helper deliberately
does not manufacture a new budget or spend from a cloned ledger. Bind six
functional IDs, actual As/Z, member/case/material/geometry, method basis and
these returned bounds/location semantics to the new derived row and evidence.
Bind complete physical evidence before receipt hashing. Apply the unchanged
checked-JSON profile and required-row coverage to the complete envelope.

Run the seven Rust test groups after obtaining the coordinated test lane. A
standalone test wrapper may put this file inside a module because its API is
`pub(super)`. Then obtain fresh independent backcheck and actual both-mode
source/composite controls, including real retained index/descriptor bindings,
case/member/global max and tie completeness, source receipt agreement and all
combined-candidate review/native obligations. This helper covers one admitted
span only; no case/global aggregation or final qualification is claimed here.

Actual file origins, hashes and observed checkout identities are in
[INPUT_ORIGINS.json](INPUT_ORIGINS.json); commands and outputs are in
[CHECKS.json](CHECKS.json). [FINAL_HASHES.json](FINAL_HASHES.json) identifies the
frozen author return. No production source, source dependency, public schema or
semantic table was changed by this child.
