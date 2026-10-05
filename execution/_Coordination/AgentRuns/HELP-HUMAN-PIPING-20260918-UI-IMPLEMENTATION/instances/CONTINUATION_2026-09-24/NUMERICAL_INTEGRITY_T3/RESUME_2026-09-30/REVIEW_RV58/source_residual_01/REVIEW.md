# RV58 — bounded independent implementation review

**CLEAR for conditional component fan-in. No unresolved actionable finding.**
This verdict covers the complete frozen diff from
`157a64b0b8bb4ad4b3170a7f9b249d218c17d529` to
`6ba653451f9fd27cdb852b7974753a3921f1c483`, with the selected I43/RV57 theorem
and the I44 scope completion. It is not product acceptance, source-law adoption,
resource qualification, general availability, or merge clearance.

TASK `/root/rv58_source_residual`, fresh independent reviewer under ROOT
`/root`, executed through delegated-harness-native. No descendants; no source
or explicit Git/index/API mutation. The reviewer did not implement I44. Same-model independence
is not model diversity. Actual receipt was 2026-10-03 01:55:35 UTC. Runtime was
expressly transferred at dispatch and released at 02:01:30.815397 UTC.

## Findings

None. No confirmed correctness, scope, numerical-contract, custody, or error-path
defect was found in the bounded candidate. The limits below remain explicit
prerequisites for broader reliance rather than newly discovered code defects.

## Mathematical and implementation trace

Paths below are relative to
`projects/chirality-piping/core/solver/frame_kernel/`.

| Reliance | Inspected implementation and result |
|---|---|
| Immutable source, ledger, P, maps, S and B | `adaptive.rs:5380` existing view validates the actual owner/source, publication and ledger identity, free order, closed potential blocks, verification cache/state, radii and body bounds. New `adaptive.rs:5726` validates RHS length, matching cached P, profile and pointer association with that view's factor scales. The borrowed factor is neither reconstructed nor replaced. |
| Source coefficient laws and exact frame | `source_residual.rs:224` retains the old route-specific coefficient builder and member pointer/ordinal check. At :275 the source coordinates and y-reference produce signed chord, normalization, projection, cross product and normalization. Every norm denominator requires a positive lower endpoint; no formed frame is treated as exact. |
| Signed arithmetic | Four directed corners in both directions, positive-denominator division, crossing-zero square ranges and the existing directed sqrt are used. Sqrt work/status joins the actual helper context and exact-sum work. The `directed.rs` change is solely parent-internal module visibility. |
| Original perturbation warrant | :462 retains the conservative Bbar/delta/lower-length contraction. :690 onward uses the retained upward beta=2B and strict alpha<1. Alpha parity with the old bridge passes. Exact independent checks also bound the actual scaled perturbation and scaled K inverse. |
| Actual correction | :722 onward forms RN1024(lo+hi)/2, then `adaptive.rs:5656` explicitly rounds each midpoint once to matching P. Exactly one `solve_scaled` application returns a free-order scaled correction. Center formation is RN1024(x+S*z), not a publication update. |
| Arbitrary-center theorem | :530 independently recomputes S(f-G*y) from the source after center formation, including individual free loads, prescribed columns and springs. :774 uses RU(beta*maxabs(rho)/RD(1-alpha)). No factor accuracy or K-only residual is assumed. |
| No-data and constraints | Individual ledger terms, prescribed adjacency and verification state determine data. :582 requires the sufficient anchored uniqueness warrant for no-data blocks. Only their free center and radius become zero. Fully fixed and nonzero constrained loads/prescriptions still pass through recovery. |
| Recovery and identity | :899 computes signed H-transpose and B-transpose actions, original station rules, spring signs, full constrained ledger and support membership. :884 checks original row identity at every set. Source magnitudes use range squares and directed sqrt. |
| Checked extraction | `ResidualSpent.result` at :620 and `SourceCorrectionSpent.into_parts` at adaptive.rs:5637 hide successful results behind joined work status. Original numeric errors survive unavailable accounting. Storage, count-range and identity refusal remain separate. |
| Publication and source fence | Original adaptive bytes remain an exact prefix. Factor arithmetic, source constructors, original bridge/helpers/oracles and recovery are byte-unchanged. New proof scratch and source enclosures do not replace publication or repurpose `source_error`. |

For arbitrary finite y with exact prescriptions, let A=SG_FF S and
rho contain S(f_F-G_F,all*y_all). The retained inverse transfer gives
||A^-1||inf <= beta/(1-alpha). Therefore S^-1(u_G-y) is bounded by
epsilon=RU(beta*maxabs(rho)/RD(1-alpha)); directed recovery over
y +/- S*epsilon contains the source rows. This argument covers midpoint,
P conversion, factor, scaling and center-addition error without assuming their
accuracy. It requires the same source/ledger/pattern/order/scales/bound premises
that the implementation checks. Coefficient-positive source laws preserve the
common frame energy nullspace for the sufficient no-data uniqueness proof.

## Independent execution and exact evidence

All seven Cargo commands used absolute manifests, --locked --offline, four build
jobs, two test threads, a 1200-second command wall and isolated absolute targets
outside both checkouts under WT/targets. All exited zero and all eight frozen
source hashes matched before/after. No compiler or test subprocess remains.

| Reviewer check | Observed result |
|---|---|
| Frozen source-residual controls, debug / optimized | 9/9 and 9/9 |
| Existing source bridge / member arithmetic / sqrt compatibility | 12/12, 9/9, 8/8 |
| Existing S11 scanner and inventory | 3/3 |
| Independent complete native predicates on zero and loaded, both builds | 52/52 for each specimen in each build |
| Independent initial and final source residuals | 24 components per build |
| Actual midpoint/P cast and center equations | 6 casts and 6 center equations per loaded build |
| Independent generated-reference families | 12/12; regeneration byte-identical |
| Reviewer 3D bending/torsion native factor center | 52/52 source rows; six residual components, inverse/perturbation/radius checks pass |
| Reviewer deliberately poor center on the same 3D source | 52/52 source rows; six residual components and radius theorem checks pass |

The independent checker imports no author or production evaluator. It decodes
binary64 fields using integers, brackets pi with Machin's
16 atan(1/5)-4 atan(1/239), uses rational difference-of-powers annulus formulas,
closed cantilever mechanics and a separate six-by-six rational inverse, and checks
magnitudes by exact squared inequalities. Its pi width is below 2^-1865. It checks
all row identities and source containment separately from the unchanged native
predicates. Relative gates include H<=A_exact, the independently reproduced
five-operation A_f64, and 10^9 H<=|x| for SI and raw=SI.

The separate reviewer fixture uses chord (1,2,2), length 3, y-reference
(2,-2,1), global force (3,9,-6) and moment (-2,-1,11). Its exact orthonormal axes
have thirds and exercise all three spatial directions with signed bending and
torsion. The production predicate helper reports 52/52 at the actual factor
center and 7/52 at the deliberately poor center. The independent checker claims
source containment and theorem/residual validation for these two added cases;
it does not independently claim their full native predicate count because that
fixture log intentionally does not export those gate operands.

The frozen author controls additionally cover prescribed columns, fully fixed
prescribed strain plus constrained load, cancelling individual terms, mixed
no-data bodies, positive springs/supports, near-parallel fixed-width refusal,
foreign source/member/scales, wrong P/cache/ledger/order/row, unsupported
directional springs/combinations, alpha>=1 and missing uniqueness. These controls
were read as well as rerun. The K-only residual counter-control independently
returns a nonzero exact source residual 1/64.

Actual cast refusal records two Round operations before any factor call.
Actual factor exponent refusal retains 15 Mul, 15 Sub and 5 Div. The simultaneous
accounting-loss case is explicitly a synthetic private seed of prior homogeneous
work; the failed factor itself executes and its actual context collection causes
overflow. No naturally reachable corruption or accounting loss is inferred from
that seeded control.

## Counts, storage and failure scope

The implementation and ownership receipt agree on four coefficient passes and
three frame builds per member. Named entries/visits, cast and point/factor work,
sqrt/helper exact-sum work, view f64 work and capacities remain distinct; status
joins do not manufacture a heterogeneous tariff. Actual producing contexts are
collected before every fallible exit. Comparisons, bit work, searches and internal
factor indexing remain identified auxiliary obligations, not zero-priced work.

New production vectors preflight Layout and use fallible reserve. Center/eta,
initial residual, alpha/epsilon, midpoint, matching-width RHS, factor x/out,
widened correction, final residual, row output and constrained-reaction owners
have distinct lifetimes. The nine-slot capacity receipt is fixed. No member-sized
coefficient/frame/model/publication cache is added. The original factor's
infallible collect/vec and internal x capacity remain unqualified; successful
small witnesses do not qualify their allocation failure or large-source memory.
Test-only traces and baseline diagnostic overlap are explicitly separate.
Source-sized nested station/support searches are finite but not covered by a
claimed complete visit budget or byte profile.

The new field guards cfg(all(test)) and cfg(test) are semantically equivalent.
The old S11 scanner still has its documented inability to strip conditional
struct fields; the new spelling leaves these diagnostic fields visible to the
scanner. Scanner functions, assertions and existing dispositions are byte-identical.
The candidate adds only the new file and four zero-site dispositions. Reviewer S11
passes on the actual candidate, so no scanner/protection weakening is inferred.

The preserved author's fixed-action failure was an interval-nesting comparison
against a coarser new reference. The new reference precision is 768 bits, while
the production pi constants and all protected old tests/oracles/tolerances remain
unchanged. Independent validation of every generated endpoint against a separate,
tighter Machin enclosure confirms this is a valid refinement. Initial narrow
A_f64-only diagnostics are preserved; final logs and the review's exact checks
cover A_exact and decimal gates too.

## Provenance and bounded disposition

The 58 inventoried I44 payloads, inventory and seal match the dispatched hashes.
The eight source files and 60 packet paths are the entire candidate diff.
The scope validator passes. Selected proposal, review, brief and ROOT selection
were checked against their exact Git pins; SOURCE remained clean at the frozen
candidate. ORIGINS, BASIS_PINS, READ_SCOPES, SOURCE_BEFORE/AFTER, raw check commands
and outputs preserve the review basis.

Procedural deviation: the first discovery command ran `git status --short` in
the chat's separate 92ea checkout without GIT_OPTIONAL_LOCKS=0. It returned clean;
whether Git refreshed that checkout's index metadata was not established. All
subsequent Git calls, including every SOURCE/NUM Git read and scope subprocess,
explicitly disabled optional locks. No candidate source or index mutation was
observed. This limits the blanket claim of zero incidental index writes; it does
not change the frozen source review or its tests.

ROOT may integrate this component review. Actual factor evidence remains
p128/P256; P512/P1024 are compiled generic branches without new full-native
witnesses. No complete facade price/profile, old factor allocation qualification,
PP normalized/material/final-row custody, readers, UI/practitioner workflow,
protected sweep, general availability, acceptance, merge or release is claimed.
Those owning gates remain outside this assignment.
