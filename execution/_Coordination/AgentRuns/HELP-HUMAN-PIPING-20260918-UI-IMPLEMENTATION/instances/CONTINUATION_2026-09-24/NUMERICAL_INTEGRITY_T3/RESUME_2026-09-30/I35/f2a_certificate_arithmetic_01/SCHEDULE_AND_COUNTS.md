# I35 — fixed scratch, passes and count interface

This completes the *logical arithmetic workspace* and source-derived operation schedule. It is not P1–P5's aggregate product-memory proof, a Rust layout statement, an adopted budget, or an actual availability result. One certificate case executes serially. Concurrent callers/retained cases multiply or overlap these owners only as P3/P4 explicitly account. There is no implicit shared global workspace.

## 1. Admission descriptor and custody

All counts belong to the same immutable source/final-row binding: N nodes; n free DOFs; c constrained DOFs, n+c=6N; m admitted straight members; b free blocks; v bodies; s actual stations; k springs; a total attributed-support membership terms visited; q immutable final rows; ell individual ledger terms; z pattern-entry visits in the exact data predicate; rho source/map entries checked; B_bytes total compared source/ledger/id/unit/row-metadata bytes over their declared visits. Derive these from actual raw/canonical C2 populations, not rounded load nets or a guessed density. Distinguish q from kernel layout size qK. Include qK in rho when checking all retained rows. A future B2 census appends its actual SIF/circular/open row counts; it cannot resurrect retired rows.

Check n+c=6N, 2n, 3b, m+1, q+7, every prefix sum/index/allocation length, all count products below, and each conversion to the qualified target usize/u32 *before* construction. Use checked u128 to evaluate the finite upper expressions, then require representability of the actual u64 accounting and target indices. Count/exponent/span refusal never becomes a zero-count descriptor. No new physical population cutoff is selected; an admitted resource cap remains a later ROOT/P5 choice.

Borrow existing C2 source and row maps, retained solve/cache/state, selected verification scales, body bounds, actual operational section inputs, body extents and final output rows. P2/P3/P4 must count those owners and their lifetimes. Needed borrowed maps include global-DOF/free-position/block/body and DOF-to-certified-row maps, member/node/station/support bijections, exact selected material identity, and final-row recipe/operand binding. A new map that is not already present must be included in P1/P2; calling it borrowed does not make its allocation disappear. Reject a missing map rather than search by coincident values.

The checked case constructor compares full source/ledger bytes once. Later row accesses use the same borrowed owner and checked layout/meta index. It must not invoke the current full-byte `publication_radius` comparison once per matrix term. If an implementation does so, its declared B_bytes increases by the actual visit multiplicity; this schedule's linear byte claim then does not apply.

## 2. Logical fixed scratch allocation

An endpoint has 128 limb bytes + 8 exponent bytes + 1 sign byte = **137 logical bytes**. An interval is two endpoints. These are field-payload sums, not `size_of`, alignment, ABI, stack or Vec capacity claims. Logical index/counter slots below are u64-sized; P5 maps them to actual representations. No endpoint contains a heap pointer or retained expression tree.

Reserve the following constant slots, conservatively allowing simultaneous liveness rather than relying on optimizer reuse:

| Owner | Slots / meaning |
|---|---|
| MemberScratch | 64 endpoints. Slots 0–31 build geometry/material/chord; 32–43 hold the two compact six-value D patterns; 44–50 retain c and A/Z/J endpoints; 51 holds invell; 52–63 are member temporaries. No member cache survives the visit. |
| VectorScratch | 48 endpoints: a 12-vector h, six B*h values, six D*(B*h) values, a 12-vector global contraction, and twelve local end-error values. Reuse h and contraction slots between the two vector families. The local end-error values accumulate both families. |
| RowScratch | 24 endpoints, reused per row: x/R/e and interval endpoints, recipe extrema and at most one live corner, raw endpoints, Hn/HU, denominator and allowance operands. Torsion never holds eight corner records. |
| Primitive value scratch | 16 endpoints for nearest/product residual/step/result and scalar intermediates. Inputs are borrowed from the other slots. This is a conservative register budget for the selected call grammar, not an assertion about generated copies. |
| Exact sum | Two 128-limb magnitudes =256 u64 slots. One fresh logical owner per operation/comparator; discarded on failure. No persistent row sum uses it. |
| Exact-sum subcall scratch | 130 raw-term limbs, 17 scaled-term limbs and 128 net-result limbs; reserved distinctly, including when source branches would reuse them. |
| Integer-core subcall scratch | 128 u64 slots. Covers three 32-limb buffers plus one 16-limb round result, or product/residual/low plus round result; no call recurses. |
| Control scratch | 128 u64 logical slots plus 64 tag bytes. Includes local context/sum metadata, loop/index/exponent temporaries (i128 uses two slots), the checked aggregate counters/status and finite refusal metadata. No dynamically formatted error string is retained here. |
| Pi | Two readonly endpoints =274 logical bytes; source numerators/denominator are the reviewed fixed constants. |

Thus constant arithmetic payload is

    152*137 + (256+130+17+128+128)*8 + 128*8 + 64 + 2*137
    = 27,458 logical bytes.

This deliberately reserves mutually exclusive subcall arrays as well. If the existing Rust call structure retains additional copies, argument temporaries, enum payloads or spill frames, P5 must add them or an implementation must demonstrate the specified workspace placement. The number is a logical upper allocation plan, not measured stack bytes. No automatic claim that the compiler realizes it follows.

Variable arithmetic payload:

- One endpoint array of length `max(2n,c)`. In the bridge pass its first n and next n entries are eta-row and v-row upward sums. After all blocks have tau, reset it in full; the first c entries become constrained global-reaction errors. No live alias survives the reset.
- Three endpoint slots and one tag per block. First two hold eta and v; the third holds tau. After block completion only tau and the checked data/zero/refused tag are read, but all three are reserved. No retained VerificationReport is added.
- Four binary64 slots per body (32v logical bytes). First hold the original normalized maxima. Apply coupling using a four-scalar local copy so both coupled directions use original operands, then overwrite with final coupled/floored scales. The existing extents/floor bits are borrowed and identity checked. Member stress scales are recomputed on demand from actual A_hat/Z_hat.
- A q-bit final coverage bitset, length ceil(q/8). It records each validated immutable final row exactly once, not a private radius per row.
- A final-row grouping index: q u64 row ordinals and m+1 u64 member offsets. Construct with an additional m-u64 fill cursor, then drop that cursor before numerical passes. Nonmember rows are visited by one final q scan using the bitset. This is an index of existing rows; no second publication is created.

Two new-payload phase bounds are therefore:

    MapBuild = 8q + 8(m+1) + 8m + ceil(q/8)
    Arithmetic = 137*(max(2n,c)+3b) + b + 32v
                 + 8q + 8(m+1) + ceil(q/8) + 27,458.

The new workspace plan is max(MapBuild,Arithmetic), *plus* explicitly borrowed-owner overlap, actual layout/capacity/allocator facts and caller storage from P1–P5. Bounds include zero-initialization/reset of these arrays; none can grow after admission. There is no source-coefficient cache, full M/T vector, dense perturbation matrix, per-member saved end-error array, per-row ExactWideSum, or serialized private endpoint.

## 3. Runtime schedule and liveness

These F0–F5 execution phases are distinct from the P1–P5 proof components.

**F0 — association and final coordinates.** Freeze final y/U/row identities after all existing replacements/qualification. Validate the checked case/verification view and row/source maps, build the grouping index, then drop its fill cursors. Scan q rows once to form the four original normalized maxima per body with existing exclusions. Reproduce original-operand coupling (two multiplies/two divides per nonzero-extent body), p512 force/moment floors in the existing position, and actual operational stress-scale rules. Each final row normalization is at most one prescribed binary64 operation. Body extent construction and its owner are existing P2/P4 input facts, not recomputed by a new norm algorithm here.

**F1 — source-premise pass.** Visit every member once; construct and validate its source intervals and coefficient majorants with MemberScratch, then discard them. Validate complete member/DOF/block maps and all source-frame/load/constraint premises. Reproduce data flags from actual individual ledger terms, prescriptions, pattern and matching verification state, storing one tag per block. A cancelling net or missing B cannot create a zero flag. Check all branches before certifying a row. No numeric cache is retained.

**F2 — response pass.** Reconstruct every member's coefficients. For h=s on free DOFs/0 on constrained ones, evaluate Bbar*h, Ddelta*..., Bbar^T*..., scale each free row by s_i and scatter upward into eta-row. Reuse VectorScratch for h=M, with M recomputed from checked x/radius at each of the 12 visits, and accumulate v-row. Reduce maxima by the existing block map, then form beta, alpha, positive denominator and tau once per data block. Refuse at the first failed mandatory block. Process no-data blocks only under the exact zero warrant. After every block is done, clear the entire row array for reaction accumulation. Block tau remains live.

**F3 — action/member-row pass.** Reconstruct member coefficients a third time. Compute DG*(Bbar*T) and Ddelta*(Bbar*M), one family at a time. Accumulate both into the twelve local end-error slots through Hbar^T, and their constrained global-reaction contributions through Bbar^T and the actual global-DOF map. Materialize each needed station error with the exact source fraction/side signs and the corresponding nonnegative majorant weights. Certify this member's actual final rows while its A/Z/J/c and end bounds are live. Recompute n and compare to the frozen input contract, then evaluate B1 recipes/distances/exact predicates. Set a coverage bit only after the full row verdict succeeds. Discard member geometry/end/station temporaries before the next member.

**F4 — direct/support rows.** Traverse spring rows and the actual support membership lists. Compute each k*T on demand; add the appropriate constrained reaction and spring error terms for the source-selected group/component. Keep at most six component errors in RowScratch and use their upward sum for an already mapped magnitude's bridge error. Direct nodal components use T, and mapped nodal magnitudes use their component-error sum. Never replace a certified kernel magnitude's value by a newly computed magnitude. Visit remaining actual rows in the final q scan, refusing unsupported or duplicate association; InputDerived follows its separate source contract. Check final coverage and unchanged G5a/row-set/scale/class facts.

**F5 — return/drop.** Return only the bounded verdict, typed failure location if any, and checked work/count summary to the ordinary transaction owner. Keep the immutable final-row borrow through the verdict's use. Drop all arithmetic workspace before or at the explicitly accounted transaction transition. Every error path has at most the same live owners as its phase; it preserves counters and invalidates the partial verdict. It creates no second solve, extra precision attempt, cached arithmetic success or public radius.

Exactly three coefficient constructions per member are priced in this schedule. Changing to two passes or a persistent coefficient cache requires a revised count/liveness witness. Final scalar output normalization occurs at most twice per row (F0 and certification), not once per corner. Source scales and actual final classes must be ready before any final numeric predicate is accepted.

## 4. Concrete arithmetic counts

Count every nonzero/zero call attempted; input-dependent early returns can reduce spent core work but cannot erase the call or source visit. All counts include failed operations' incurred prefix. An admitted upper bound is a reservation, not a claim it was executed.

One member builder has the following upper operation counts, using separate endpoint directions:

| Work | down/up scalar calls |
|---|---:|
| Geometry D-t, c-t | 4 sub |
| P, c², ri², G, A, I endpoints | 11 mul |
| Q endpoints | 2 add |
| Z endpoints | 2 div |
| Three chord differences and invell | 6 sub + 1 div |
| Material 2(1+nu), E/den | 2 add + 2 div |
| Four source C interval products and four K products | 12 mul |
| Two endpoint differences per C | 8 sub |
| Source/delta coefficients divided by ell_lo | 8 div |
| Exact exponent shifts | at most 15 |

Total builder: **4 add, 18 sub, 23 mul, 13 div =58 directed calls**, plus at most 15 checked exponent shifts, 32 binary64 lifts and 128 direct endpoint/premise comparisons. The four-source-product count deliberately repeats identical circular Iy/Iz work. The geometry schedule is a new outward realization of B1C, so its count is not B1C's exact-integer 5779 loop count and must not be added to that count.

Contractions and rows, beyond three builders per member:

| Work | Upper directed calls / other counts |
|---|---|
| F2 two contractions | 236 mul + 236 add per member; plus <=12 add for M construction =248 add |
| F2 block reduction | <=2n endpoint max comparisons |
| Block certificate | 2 mul +1 sub +1 div; one exact beta shift; <=4 comparisons per block |
| F3 two contraction families | 244 mul +244 add; reserve 24 extra scatter adds and 12 M adds =280 add per member |
| Stations | <=24 mul +24 add +1 sub per station; copies/metadata extra |
| Springs/support membership | <=(k+a) mul and <=(k+a) add; at most k+a exact T shifts |
| Per B1 final row | <=6 component-error adds, 3 source-interval operations, worst torsion's 16 mul+16 div, <=2 raw conversion calls, 4 abs-difference subtractions =**47 directed calls** |
| Final exact tests including small-b search | <=68 fresh exact comparators per row; each <=5 direct/scaled terms |
| Normalization | <=2q prescribed RN64 operations, identity is a checked no-op |
| Ordinary final scale/class/allowance arithmetic | <=32q+32v further scalar f64 operations/comparisons under B1's closed recipes, including five A_f64 operations, threshold, b0/r tests, four-operation stress scale and body coupling; pi/source geometry is excluded |

Thus a simple B1 scalar-call admission upper is

    D = 1182m + 4b + 49s + 2(k+a) + 47q,
    C = 68q,

where D counts directed calls, C the separate exact final comparators. Source visits, direct comparisons/lifts/shifts, byte comparisons, array resets and inherited caller work are additional. q is all actual final rows, so skipped/InputDerived rows only make this conservative. It gives no permission to apply a B1 recipe to a B2 row.

A concrete auxiliary-loop reservation is

    A = 8192*(1+m+n+c+b+v+s+k+a+q+rho) + 4*(B_bytes+ell+z).

Its units are bounded scalar/array loop-body visits in the specified facade grammar, including initialization, grouping, map validation, max/abs/lift/shift/copy/tag checks and final coverage. B_bytes is the actual total length over declared byte visits, not a digest length. A source adapter doing extra sorts/canonicalization/preparation must expose its additional count in P1/P2; it is not licensed to call that A or zero. This conservative reservation is a proposed checked resource interface, not an adopted tariff or measured instructions. Every executed facade loop must name its owning population or fixed bound in the implementation correspondence.

## 5. Primitive cost and counter safety

A fresh directed call has at most five raw additions, five anchor shifts, two round nettings, one side signum, and one <=16-limb small-factor scaling. With <=128 carry visits per add:

    term_limbs <= 5*(18+128)+16 =746
    shift_limbs <=5*256=1280
    net_limbs <=5*128=640
    rounded_limbs <=2*128=256
    total SumWork <4096; max_span<=8128.

The five-term final comparator may use five small-factor scalings: term<=810, shift<=1280, net<=128, round=0, again <4096. All its existing ordinary additions are therefore exact u64 operations in the fresh local. These conservative bounds include the live five-term carry proof; they do not assert safety for an arbitrary reused ExactWideSum.

At width 16, the fresh WidthWork upper vectors are:

| facade call | existing counted core calls | existing core LME expression, for evidence only |
|---|---|---:|
| directed add/sub | round<=2 | <=64 |
| directed multiply | two_product=1, round<=2 | <=384 |
| directed divide | div=1, two_product=1, round<=1 | <=17,794 |
| directed sqrt | sqrt=1, two_product=1, round<=1 | <=18,820 |
| final exact comparator | no rounded core call | 0, with nonzero SumWork/other work separately |

No new LME tariff is invented. The numbers in the last column merely apply the existing width/kind formula to the actual reused core calls. They are **not** automatically charged under the selected kernel 20B/60B policy. The new facade's work must be explicitly admitted and integrated; it is not free because it occurs after a kernel selection.

For a platform-independent finite execution bound, define one microstep as one limb-loop body or fixed scalar action in the inspected grammar (not one CPU instruction). For the inspected source, use the following conservative fixed microstep bound per directed call or exact comparator: the largest loop is sqrt's 1026 rounds, each with <=6 loops of 17 limbs plus its fixed scalar step. Reserve a further 2^14 loop-body/fixed-action visits for all initialization, 32/128/130-limb scans, product/residual, at-most-five raw additions/shifts, two nettings, side test and one step. Allowing 16 elementary fixed actions per visit gives `16*((6*17+1)*1026+2^14)=1,952,992 <2^21`; use the conservative **2^21** microstep cap. The division and other scalar schedules have fewer visits. The implementation must preserve that fixed-action correspondence or revise this bound. With the auxiliary reservation, use

    U = 2^21*(D+C) + A.

Require all count vectors and U representable before work starts. A lower resource allowance can refuse; no allowance is selected here. Track actual primitive attempts, existing fresh WidthWork/SumWork payloads, source/row/byte visits and failure prefix separately. Do not describe U as spent work, seconds, RSS or LME. Aggregate each numeric counter with checked add/multiply before accepting its next state; counter failure latches unusable evidence, invalidates the certificate and cannot reset on fallback or the next case.

I34 proposes E/O/I/OI status and terminal custody for the existing kernel. This facade is compatible with that direction but does not adopt its API, claim its implementation exists, or lift C1's present upstream-no-wrap blocker. Its fresh-local safety proof only covers the new bounded grammar. Final W1 receipts still need qualified kernel work and C1's explicit reconciliation. A numerical predicate pass does not confer exact-charge standing.

## 6. Conditional B2 slots and P1–P5 return

The sibling B2 arithmetic interface was received while this task ran. Without caching, for actual sI SIF, mC circular maximum and mO surviving open-summary rows it requests:

- endpoint square products 4sI+8mC; directed sqrt calls 2sI+4mC;
- SIF multiplies 2sI;
- directed positive divisions 2sI+8mC+8mO;
- additions 2sI+8mC+8mO;
- endpoint maximum comparisons 2mC+2mO;
- abs-range calls 2sI+6mC+6mO, each <=2 abs plus4 comparisons.

Add these calls to D and their auxiliary actions to A only after B2's separate review/source/maximum premises and concrete C2 recipe coverage are integrated. RowScratch24 can stream its two endpoint functions: two input intervals, their four abs bounds, two squares, two sums, one norm interval, one axial interval, one output interval and two final max endpoints need <=24 endpoint slots with square/sum/norm reuse. No source maximum theorem or availability follows from that arithmetic capacity.

| Proof owner | Concrete handoff from I35 | Still required there |
|---|---|---|
| P1 adapter/census | n,c,m,b,v,s,k,a,q,qK,ell,z,rho,B_bytes; row grouping and one-time custody requirements; D/C/A/U formulae | actual raw-to-canonical populations, unsupported/failure paths, bytes and source identities, checked count implementation |
| P2 preparation | exact map-index lengths, fixed scratch, three source passes, no coefficient cache or duplicate source construction | source/map allocation ownership, capacities, existing setup lifetime; no hidden map manufactured after admission |
| P3 retained numeric owners | borrow selected P=2p scale/cache/state/body B; 3b endpoint slots+tags; no report retention; max(2n,c) array reuse | actual cache/clone/error/combination overlap and qualifying the view/data flags and kernel work |
| P4 publication/caller | 32v scale bytes, q-bit coverage, row index, final immutable borrow, exact raw/SI predicates and finite small-b search | actual final envelopes/strings/receipt/canonicalization/rollback/caller overlap, B2/ordinary truth, adopted facade work integration |
| P5 build/profile | logical payload expressions and per-operation loop bounds | real Rust layout/padding/stack/Vec capacity, target and feature-specific source correspondence, build/caller qualification and selected resource allowance |

No compiler, solver, runtime or host qualification was performed. These handoffs let the next participants price and review a concrete arithmetic design instead of a placeholder format.
