# RV67 — prepared-producer design review

**Disposition: conditional mathematics verified; three blocking design-readiness
findings. Return to I51 for a bounded proposal completion, not implementation.**
The existing selected dual-cover method remains binding. This review selects no
replacement method and establishes no live producer, availability or public F2a
success. No presently unavoidable owner-held choice was found.

TASK Type 2 `/root/rv67_prepared_producer_design`, parent ROOT HELP_HUMAN `/root`,
executing through delegated-harness-native collaboration; no descendants.
Receipt: 2026-10-03 08:54:19 UTC, Mac.lan. Dispatch basis:
`e85ab6541bcc4ecca339c38ed9961b8fd5b0ceac`. Subject:
`I51/replacement_producer_01` at
`8d025201b8dc4591aa3be7f4534c0cbb932e594c`.

R is this packet's RESUME_2026-09-30 ancestor. PP is
`projects/chirality-piping/core/product_physics`; FK is
`projects/chirality-piping/core/solver/frame_kernel`. Source locations below are
at `8104a4fedd0fd575f20b3d723cc0cdd722d73ba1` unless stated otherwise. The I50
repair at `c79a1c293dbf5581468e839c8545b5a815a32df7`, integrated at
`7e9597bd9cc7b2d2b1868e421caac1d8a920cb1b`, changes the support bitmap accounting
and bounded uniqueness check, not these numerical/source interfaces. The inspected
repair diff is recorded by hash. All eight I51 packet files and three external
bulk entries matched their preserved bytes/hashes; all 43 I51 origin hashes
matched. The prior startup optional-index-refresh uncertainty remains exactly
qualified in I51 EXECUTION/AUDIT; it is not erased by this review.

## Blocking findings

### RV67-1 — explicitly reconcile the changed formation process and p meaning

**Location:** I51 INTERFACE.md:54–61, 87–100, 116; RETURN's public-warrant table.
D1 DESIGN_NUMERICS/DESIGN.md §§4.1.2, 4.1.5, 4.1.6 and 4.1.6.1 specifies
formation/recovery at p, magnitudes formed at p, and derived stresses using the
existing binary64 publication methods on once-rounded actions. I51 instead
projects primary/component-stress rows from a dual-hull midpoint at fixed1024;
its norm readout/projection precision is not explicitly reconciled either.
Higher precision and a sound final error certificate do not themselves satisfy
the earlier process description.

This is numerically consequential on the named witness, not merely wording.
The candidate action T rounds to `3f8d7dbf487fcb92`. Applying the existing
separate binary64 T*c/J and MPa operations with the new properties gives raw
`3efbf3ab943069f5`, normalized `403aa82dcd5efbcc`. Its exact dual-readout error
is about 3.0762436768e-15 Pa, exceeding its own unchanged smaller sharper allowance
2.9624495887e-15 Pa. I51's projected raw `3efbf3ab943069f4`, normalized
`403aa82dcd5efbcb`, has error about 1.5425424107e-15 Pa and passes. Retaining the
old production recipe while implementing only new geometry does not reproduce
the proposed passing candidate.

**Required correction:** specify the exact fixed1024 midpoint, norm, raw-unit and
RN64 sequence, and prospectively reconcile the affected D1 clauses for precisely
this prepared ordinary producer. Native p must remain the actual selected
128/256/512 precision, native verification precision 2p, with its unchanged
schedule, R7/A1 admission, actual stop evidence and new-source ownership. A
separate proof/projection precision is not native p and does not trigger p512
floors. State explicitly which final rows are newly projected and what their
certificate proves; never label the native stop list as observed convergence of
those different final values. Resolve the product stop-rule/process description
as part of the selected design, with corresponding method/replay/reader
consequences. Alternatively retain the existing production rules and supply a
new complete passing witness. The current packet leaves that choice implicit.

ROOT can select a justified numerical production amendment under delegated
correctness authority; this finding does not automatically require human approval.
No protected sharper/decimal comparator, floor, scale construction, native
admission or public accuracy criterion may be relaxed to do so. See the concrete
minimum contract in DESIGN_FENCE.md §1.

### RV67-2 — close the preparation and dual-readout API before implementation

**Location:** I51 INTERFACE.md:16–53 and 87–94, especially “Expose narrow
owner-bound preparation/projection data needed by PP.”

The existing numeric geometry helper accepts supplied `MemberOperands` and
returns private `MemberEnclosures`; it is not a producer of authenticated prepared
members. PP currently copies old `BuiltModel` section bits into both source and
facts (retained_product.rs:1158–1273). The existing cross-crate entry
`RecordedInvocation::certify_product_case` (origins.rs:786–803) accepts already
final borrowed rows; final_case.rs:858–934 itself runs the source residual. No
specified API yet prepares rounded properties, projects rows, and reuses the
same two residual results for the final certificate. Adding a projector and
then calling that unchanged entry can recompute residuals/corrections, invalidate
the stated at-most-two correction bound, or use different evidence than the
projection did.

**Required correction:** define the constructor inputs, private fields/getters,
Spent/result/error shape, owner validation and consuming/borrowing relationships
for (a) geometry preparation, (b) PP's new prepared case, and (c) one owner-bound
pair of closed residual lanes used through projection and final checking.
Distinguish a scalar rounding fact from PP invocation/material provenance.
Old-to-old built checks remain; new-to-preparation and new-source/facts/operational
checks are additional and explicit. Closed AdmittedK uses exact products and
zero coefficient differences; AnnularSource uses source coefficients throughout
residual and recovery. Neither lane admits a public arbitrary operator, interval,
value/radius constructor or unowned factor. Define how first-lane work survives
second-lane failure and enters the same bounded local accounting. DESIGN_FENCE.md
§§2–3 lists the minimum closed seams. File names and intended impacts alone are
not an implementation-ready interface.

### RV67-3 — specify the frozen candidate/observable lifetime and commit boundary

**Location:** I51 INTERFACE.md:62–76 and 125–140.

`ProductFinalRow.value` is `&f64` (final_case.rs:136–143), PP `bind_rows` borrows a
complete MechanicsEnvelope (retained_product.rs:1521), and PP `observables` reads
that envelope's rows, extrema evidence and summary aliases (:1754–1988). The
proposal supplies only Q_final replacement numbers and M maximum records before
asking these existing functions to check a complete immutable final candidate.
It does not define the overlay/final-view contract or when the ordinary envelope
is moved/mutated. Naively borrowing the ordinary rows certifies stale values;
cloning a replacement envelope adds another publication-sized live owner;
mutating the fallback before all gates finish breaks the stated failure path.

The two current ResidualSpent results also retain `view.data`, `alpha` and
`epsilon`, not only Q_native rows (source_residual.rs:600–628). Current access is
by borrow. Dropping only center/rho/correction does not establish the proposed
compact lifetime or release the other retained buffers.

**Required correction:** choose a concrete frozen overlay or a consuming draft
with closed row access, maximum-evidence access and headline access used by
*all* final gates. Define exactly which old metadata/ancillary bytes are borrowed,
which values/evidence are owned, when each lane's auxiliary vectors are dropped,
and how the identical certified candidate is moved into final publication without
an extra copy or fallible partial mutation. All allocations, capacities and
failed prefixes must be retained in Spent, including both lane data bitmaps,
row descriptors, maxima/metadata containers and serialization overlap. The
private component fence must say what can run before public routing/receipt
activation, rather than inviting an implicit transaction redesign. See
DESIGN_FENCE.md §§3–5.

## Independent mathematical result

### Geometry and genuine new K

For actual normalized binary64 D,t, retain c=D/2, r=c−t,
P=t(D−t)=c²−r² and g=P(c²+r²)=c⁴−r⁴. Existing admission and exact 0<t<c make
all factors positive. A=pi P, I=pi g/4, J=pi g/2 and Z=pi g/(4c) follow directly.
Outward positive arithmetic therefore encloses every property. RN-even is
monotone: equal finite RN64 endpoint results prove the uniquely correctly rounded
source property. Existing positive/range/normal-primitive gates must still run;
equal endpoint conversions to zero/overflow are not admission. Unequal endpoints
mean refusal at this fixed width, not a retry or altered geometry boundary.

check.py independently uses pi=4(atan(1/2)+atan(1/3)), with 320 even-series terms
per arctangent and exact remainder bounds. It confirms B1C's 512-bit pi constants,
then verifies both rational geometry and the 19-entry directed1024 geometry
schedule. It reproduces A=`3f7872fa3a37ac13`, I=`3efc52664442210a`,
J=`3f0c52664442210a`, Z=`3f31b37feaa954a6`; c stays `3fb999999999999a`.
Old J is `3f0c52664442210e`, not the new primitive under another label.

The checker rebuilds and exactly matches both captured canonical K4SRC encodings,
including constraints, every load id/term, stations and support groups. Changing
only the four A/Iy/Iz/J fields gives a distinct *analytical* new encoding; its
hash is labelled non-live. Actual future preparation, new PrimitiveSource,
RecordedInvocation, source/facts/operational association and solve are still
required. The same request and selected independent E=200e9/G=80e9 remain. No
fixture search, nominal-wall re-subtraction, E/nu substitution or exact-profile
change is involved. For interpolation, retain I36's selected source law and
resolved E/G cover; scalar section preparation does not authenticate material
selection.

### Two closed residual laws

Let S be the actual verification factor's positive radix scaling, A_K=S K_FF S,
and beta=2B from the associated accepted upward bound. R7's data-block warrant
gives ||A_K^-1||∞≤beta. For lane L, define Delta=L−K and an outward eta bounding
||S Delta_FF S||∞. Then alpha=beta eta<1 proves

    ||(S L_FF S)^-1||∞ ≤ beta/(1−alpha).

For any finite center c with exact prescribed c_C, form a fresh outward scaled
residual rho=S(f_F−L_Fall c). It follows that

    |u_L,i−c_i| ≤ s_i beta ||rho||∞/(1−alpha).

For AdmittedK the four products E_K A_K, G_K J_K, E_K I_zK and E_K I_yK are exact
binary64 products of at most106 significant bits. Same exact frame, spring,
ledger and constraint maps give Delta=0, eta=alpha=0, hence beta||rho|| suffices.
Frame interval width and computed residual need not vanish. No rounded native
f64 radius replaces this tight readout proof; native A1/radius admission remains
an independent prerequisite.

For AnnularSource use B1C geometry and I36 selected independent E/G intervals,
the exact coefficient differences, the same maps/scaling and strict alpha<1.
The optional cached-factor correction merely chooses a better center: the
verification P=2p factor receives scaled residual midpoint, and its output is
mapped back by S. The post-correction residual, evaluated independently, proves
the bound even if the correction is inaccurate. It grants no new factorization,
solve schedule or repeated correction. The current pointer/run/source/cache/
scaling checks in adaptive.rs:5332–5567 and :5725–5760 are essential.

No-data blocks get zero error only from the existing individual-term,
zero-prescription-adjacency, verified-zero and uniqueness premises. Preserve the
current sufficient anchoring rule or prove a separately reviewed replacement;
missing B/cache/radius is a refusal, never zero. Recover every lane with its own
L-dependent end/station/reaction functional and stress denominators. Directly
applying that functional to the state enclosure accounts for both response and
functional change; using K's functional for the source lane would not. Four
finite two-by-two controls illustrate both residual laws and arbitrary centers;
they are not a new runtime engine or proof of live widths.

### Complete named analytical witness

Static equilibrium gives theta0=(alpha/144,2alpha/1e6,2alpha/1e6),
u1=theta0 cross (1,2,2), theta1=theta0+T(1,2,2)/(GJ), T=3alpha.
This is a compatible rigid rotation plus pure twist. Root spring moments balance
the applied tip moments; all other actions vanish. Positive member coefficients,
three rotational springs and fixed root translations make zero energy imply
zero displacement, so the solution is unique. End-i nodal torsion is −T, while
j-side section stresses have +T c/J at every site. This is an analytical
verification, not permission for a fixture recognizer in production.

An independent integer quotient/remainder RN64 implementation reproduces every
mechanical raw value, normalized value, class and scale in I51: 97 mechanical
rows per mode, 98/99 total, with 69 Absolute, 25 Relative and 3 InputDerived.
The independently bounded norms and geometry produce the same bits even when the
analytical midpoint/raw conversion is explicitly rounded at fixed1024. That
last check concerns these analytical intervals, not future residual widths.
All ancillary rows preserve their captured bits and are excluded from mechanical
scales. Every complete-row primary maximum, original-operand coupling and new
operational A/Z stress scale matches. Both sharper predicates and raw/SI decimal
predicates hold; the smallest fractional sharper margin is 0.38994634518.
Enlarging each non-input SI interval by 2^-100 still passes, as a labelled slack
control only. Eight support component/norm equalities hold in each mode.

The circular normal-stress coefficients are zero here, giving analytical
coefficient enclosure [0,0], midpoint +0 and a valid zero stress alias; torsional
shear is separate. Production still must regenerate actual coefficients from
its final candidate actions and prove the separate physical maximum cover.
Preserve complete-domain/tie/location/metadata evidence and headline aliases;
never simply copy the old ordinary extrema. Actual source/input-derived mapping,
nonquantity custody and support attribution remain required at implementation.

Restoring old J gives a strictly positive no-common-center sharper obstruction
at the recomputed candidate stress scale: lower margin 8.5166314238e-15 Pa.
The new geometry changes the obstructing premise; it does not disprove RV66.
The analytical G5a necessary coupled-upper demands independently match
53050.36890729408 N and 96.11431291987076 N*m (the gate allows equality).
Only actual new-K verification-derived resolution may satisfy G5a. Native p is
unknown for that future solve; if p512 is selected, insert its actual floor bits
in the established post-coupling position and recompute all scales/classes/gates.
The p128/256 analytical values are not transferable evidence for that run.

## Warrant, resources and disposition

Directly inspected owner physics delegation, CORRECTNESS_ACTIVATION,
T3 OWNER_DIRECTION, ROOT_SELECTION_DESIGNS and adopted ROOT_RULINGS clauses
5048–5075, 5102–5123, 5183–5209, 5227–5248 and 5340–5352 support continued bounded
numerical design. They protect independent selected E/G, actual effective wall,
exact-profile source truth, private dual cover and all protected criteria.
They do not freeze legacy section-rounding bits as the only possible future K.
Keeping both newly admitted K and source readouts therefore requires no new
exclusive public-meaning choice. It also does not make the public row “a union.”
The formation/process amendment in RV67-1 must be explicit before selection.

The 19 directed geometry entries are correct for the factored helper schedule:
4 subtractions +11 multiplications +2 additions +2 divisions. This excludes lifts,
shifts, checks, conversions, material work and coefficient work. The proposed
32 Endpoint scratch limit is **unselected and unqualified** until a named live
layout includes helper temporaries, by-value copies and scalar context/sum
scratch. Seven f64 fields are payload, not total per-member storage. No logical
count is a sizeof/RSS result or a new tariff. Both lanes, fallible prefixes,
ordinary fallback, preparation, source/solve, staged values, metadata and caller
serialization must be accounted as DESIGN_FENCE requires.

No currently necessary owner question is returned. An exclusive K/source
reinterpretation, a weakened protected criterion, increased ceiling or acceptance
of required refusal would still need its existing owning decision. This review
permits none. I51 can complete these three precise design gaps and return for
backcheck; ROOT then decides selection. Live new-K p/residual widths, G5a,
complete observables, resource/custody qualification, actual public receipts and
readers, both-entry protections, exact-block coexistence, pressure/no-pressure
controls and native successor Current evidence remain later gates.
