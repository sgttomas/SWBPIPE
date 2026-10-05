# I33 source/private proof seam and bounded accounting inputs

This is a proposed integration shape for independent review, not an API change
or authorization to implement. It leaves PrimitiveSource/SourceParts, the kernel
operator, selected schedule, emitted values and public schemas untouched.

## Available evidence versus missing interface

| Needed fact | Existing source / lifetime | Legitimate use and missing work |
|---|---|---|
| q_K row and radius | RetainedSolve publication and private publication_radius_bits; adaptive.rs:2941–3095,3420–3542 | B1's checked row view can borrow the actual row and matching radius. No free (value,radius) input; no absent=zero. The cross-crate checked view is proposed, not implemented. |
| Source/group/order | RetainedSolve owns prep/group/cache/states; GroupPrep at adaptive.rs:1051–1060 has pattern, free ordering, bodies and blocks | Bind the source, complete DOF maps and actual selected case. The source rows are canonically sorted; facade indices need an explicit bijection. |
| Verification scaling S | The shared build at P=2p contains its factor; GroupCache s256/s512/s1024 at adaptive.rs:3582–3593; factor.rs:624–626 exposes scale() only inside the crate | This scaling is retained through the cache, not merely a transient report field. Borrow the selected verification build's exact i64 exponents; check successful slot, P, stiffness identity and ordering. Never substitute candidate p scaling, a different cached precision, or product engineering units. |
| Inverse bound | VerificationReport blocks, theta, bodies are live-only (verify.rs:597–636); BoundVerification is transient (adaptive.rs:2352–2368) | Per-block B_c/flags are not retained in the full report. Do not retain/reconstruct that report just for this packet. |
| Conservative retained B | summarize rounds each body's B upward (adaptive.rs:2394–2412); RetainedEvidence.certified_bound stores per-body bits (:3415–3417) | The selected live solve's own B_b suffices for beta=2B_b on its data blocks. Validate its body association, policy and selected verification provenance; an unattached public JSON field is not proof. Multiply by 2 exactly in bridge arithmetic, not unchecked f64. This is looser than live per-block B_c, but avoids a new report allocation. |
| Data-block coverage | R7 §5.5 item 7a; bound.rs:127–158 forms flags from actual ledger, prescribed-nonzero facts, pattern and verification state | The per-block report flag is transient. A private view can reproduce exactly that predicate from existing source/ledger and retained verification state. Any no-data shortcut needs the accepted exact homogeneous/unique-solution warrant, not just count or published zeros. Alternatively consume validated flags while live; explicitly account any retained bitset. No arbitrary flag supplied by PP is trusted. |
| Source geometry/material | Normalized PP model, actual DerivedSection effective wall, SourceAnnulus source D/t accessors, selected IsotropicENu pair; PP lib.rs:6507–6563; physics_source.rs:918–977 | PrimitiveSource intentionally has only admitted K properties. Borrow a separately checked PP source binding for D/t/E/nu and actual K properties. This is a private certificate association, not a SourceParts extension. Do not infer D/t or nu from rounded A/I/J/G. |

All vector/matrix norms in RETURN are norms of numerical coordinates in the
existing SI representation and R7's fixed ordering/scaling. S is a positive
diagonal power-of-two map; u=S z and scaled load is S f. Physical translation,
rotation, force and moment units are restored through the same row/DOF maps.
An arbitrary rescaling, rcond estimate, unscaled B, or coincident unit label does
not meet the theorem. The prescribed columns are unscaled inputs to Delta*u_K.

A minimal two-sided private view could be named `CheckedSourceBridge<'case>`:

    Kernel side: owner-bound source/policy/precision identity; actual row views;
                 free ordering, blocks/body map; exact scale exponents at P;
                 associated upward B_b; validated data/zero-block facts.
    Product side: actual captured case/basis; D/t and selected E/nu bits;
                  member/DOF/station/support bijections; actual K property bits;
                  actual final row snapshot and recipe identity.

Only validating constructors create these views. Owner/reference association
prevents later pairing with another solve; explicit identities protect two live
solves with the same values. Keep normalized-source custody, source bytes and
ledger identity separate from material/stiffness equality. A failed association
is a refusal, not a numerical zero. Public visibility required across Rust crates
would not make fields caller-constructible or establish a security boundary.

The product performs B1C geometry and material differences, then sparse bridge
passes while borrowing these views. A small private bridge workspace retains
block tau values only as long as final rows are certified. No source radius is
serialized, no second solve/publication is cloned, and no VerificationReport or
ExactWideSum is retained per row. A final row's mutation, replacement or basis
qualification invalidates its verdict. The final gate remains after all current
row-set changes. This is a custody plan, not existing executable behavior.

Potential write seams for a later specifically authorized slice are FK's narrow
read-only row/scaling/bound view and PP's private normalized-source/member binding
and bridge evaluator. No current source is changed. Verify the actual lifetime
and cache-success invariant before relying on the retained route; if unavailable,
the alternative is a bounded live-only extraction during verification, with new
storage/overlap accounting, never silent report retention.

## Fixed passes and accounting inputs

Let n be free DOFs, c the constrained DOFs, m straight members, b free blocks,
s stations, k spring terms, q final rows, and B_source the compared identity
bytes. Counts come from admitted source and checked maps, not a new physical
cutoff. Let L_B be the selected fixed endpoint storage in bytes and W_B its
integer-limb cap; neither value is selected by I33.

One viable schedule is three member passes (or one pass plus a counted cache):

1. Validate/custody and construct source coefficient/geometry intervals as
   needed. B1C gives its exact two subtractions, one addition and eight products
   per section, with 5779 conservative grade-school limb-product visits for
   that geometry construction alone. Add three exact chord differences, ell
   max/absolute comparisons, selected E/nu denominator and coefficient products,
   signed endpoint differences and checked divisions. Those additions need their
   own counters and size proof; 5779 does not price them.
2. Accumulate eta and v on free rows, then maxima by block. Do not materialize
   an n-by-n matrix. For each vector h, use
   Bbar*h, Dbar_delta*(Bbar*h), Bbar^T*(...), then each outer s_i.
   With 48 Bbar nonzeros and 10 D entries this costs at most
   48+10+48+12=118 scalar multiplications per member per vector. The two vectors
   are h_j=s_j on free DOFs/0 otherwise, and h_j=M_j on all DOFs: **236**
   scalar multiplications per member before source construction. Nonnegative
   additions are bounded by the same term counts plus row accumulation. Form
   beta, alpha and tau once per data block; no refinement loop follows refusal.
3. Reconstruct the same source coefficients (or use the explicit counted cache).
   For source-action error compute Dbar_G*(Bbar*T) and
   Dbar_delta*(Bbar*M). Each six-vector yields both local end bounds through
   Hbar^T and global reaction contributions through Bbar^T. A dense-in-these-
   patterns upper count is 2*(48+10+16+48)=**244** scalar multiplications per
   member. Source-map scattering and additions are extra. Station bounds use
   their fixed signed recipe's absolute coefficients, bounded by 24 additional
   scalar products per station. Global springs need at most one k*T product
   per action term plus attributed accumulation. Bind q final rows and run the
   already scoped B1 endpoint comparisons separately.

Without a coefficient cache, count up to three geometry/material constructions
per member across these passes, not one. Eliminating that repetition requires an
explicitly counted cache or a revised proven streaming schedule.

These are deterministic scalar term-count inputs, not LME tariffs, actual runtime
measurements or universal integer-width proofs. Skips may reduce counts but
cannot be relied on without a checked counter contract. With fixed W_B-limb
schoolbook arithmetic, each multiplication uses at most W_B^2 limb products;
division/rounding and comparisons need their own fixed bounded loops. Never
charge new facade work as zero under the selected 20B/60B kernel work policy.

For example, Pass 2 needs two free-row accumulator arrays (2n endpoints) plus
per-block eta/v/tau storage, with eta/v slots reusable after the test. Pass 3
can reuse the freed arrays or allocate explicitly counted constrained reaction
accumulators (c endpoints). A nodal M or T value can be borrowed/recomputed through
the row map when visiting a member, avoiding another full vector. Count the
choice actually implemented, the B1 source maps, source/case bytes and support
group accumulation. A source-geometry scratch instance plus a fixed collection
of six- and twelve-entry temporary vectors suffices mathematically; their exact
buffer/carry reuse, exponents, numerator/denominator representation, stack/heap
placement, capacities, invalid paths and overlap with caller/receipt/solve/native
buffers are still M1 work. No product allocation or memory bound is proved here.

Each new operation records its actual count/limb visits and spent failure work,
maximum widths/spans and fixed scratch peak against pre-admitted bridge limits.
No unbounded rational expression tree, denominator growth, retry, per-row proof
object cache, or pi/root series is needed. A fixed outward endpoint format keeps
row storage bounded; its exact rounding proof remains a prerequisite. Failure
must preserve the existing ordinary fallback and record the missing W1 proof.

Independent review should first check the actual 48/16/10 patterns, prescribed
columns, data-block/scaling association and exact predicates. It should then
identify one concrete finite arithmetic format and calculate its complete live
storage/work overlap before any implementing reliance or availability claim.
