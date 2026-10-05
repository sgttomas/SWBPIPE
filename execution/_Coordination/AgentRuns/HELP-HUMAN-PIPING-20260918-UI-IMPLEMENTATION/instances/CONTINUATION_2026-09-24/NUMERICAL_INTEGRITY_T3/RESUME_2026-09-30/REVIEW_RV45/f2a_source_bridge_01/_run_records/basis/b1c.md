# I31 B1C — fixed source-geometry enclosure and remaining action bridge

**Finite derivation for RV42 backcheck; no implementation or acceptance.**
Direct native TASK /root/i31_f2a_certificate_b1 under ROOT /root. Receipt:
2026-10-02 20:18:42 UTC; analysis cutoff 20:53:42; deadline 21:03:42.
P = projects/chirality-piping; PP = P/core/product_physics;
FK = P/core/solver/frame_kernel. Maintained source pin:
49034a940f3f8cd3f3da4d4cbc839943b808063d. B1 and RV42 are unchanged.

**Result:** exact positive enclosures for the existing normalized OD/effective-wall
section are derived below, with a fixed 512-bit dyadic enclosure of mathematical
π, a finite arithmetic schedule and 12 passing design control groups.
This completes the missing geometric-operand construction conditionally on the
actual normalized source association. It does **not** establish q_K=q_G or an
error bound between them. Exact-mode public response/source-truth reliance still
needs that separate bridge. No operator, public truth, protected comparison,
domain, output/scaling operation or availability criterion is changed.

## 1. Accepted source layers

The omitted warrant is maintained:
PP/tests/fixtures/pressure_reference/SOURCE_ODWALL_EXPECTATIONS.json:6–8 states
exact normalized binary64 OD/effective wall, with mathematical ro=OD/2 and
ri=ro−wall; rounded reported radii may differ. Its :407–408 preserves nonzero
relative 1e-9, the specified zero scales, and existing runtime boundaries.
It is not a license to use the rounded reported radii as section truth.

PP/tests/pressure_section_geometry.rs:27–44 enforces the protected comparisons.
The no-pressure fixture uses an empty region list (:80–94), and its dedicated
test (:459–469) still checks source-section response and geometry. The checks
at :287–324 use source I for bending displacement/rotation and source J for
torsional rotation. The :326–343 stress reference uses source Z. Thus the exact
profile's warrant is not limited to a post-processing denominator.

| Layer | Ordinary/preview route | Exact-profile route |
|---|---|---|
| Declared K problem, q_K | D1 §4.1.1–2 and FK retained/source.rs:1–21,90–104 use admitted binary64 A,Iy,Iz,J,E,G as exact primitives; exact-coordinate frame/length. PP's legacy derived properties supply these bits. | Same K boundary, but PP lib.rs:6507–6525 supplies rounded SourceAnnulus A/I/J and actual derived G. K certification concerns those admitted primitives, not automatically their source-geometric predecessors. |
| Intended section functional | Existing Euler-Bernoulli preview and section formulas remain (preview_physics.rs:73–80; PP lib.rs:1929–1940). D1 names Z=I/c. I_K/c is a possible clarification, **not selected here**. Consulted ordinary controls do not establish authority to replace every geometric source promise by that definition. | Existing source warrant fixes c_G=D/2, ri_G=c_G−t and real-π A_G/I_G/J_G/Z_G below. Stored-bit singleton A/Z/J or I_K/c as the whole exact-mode reference is inadmissible. |
| Actual producer properties and y | PP lib.rs:9436–9487 evaluates rounded effective wall, d_i, differences of powers with binary64 PI, then A_hat,I_hat,J_hat,Z_hat,c_hat. Stress recovery executes N/A_hat, M/Z_hat and (T*c_hat)/J_hat; MPa emission divides by 1e6. | SourceAnnulus uses Scaled f64-mantissa arithmetic and final accessor conversions (pressure_exact/source_geometry.rs:28–68; pressure_exact.rs:13–141). Z_hat can use Scaled I before its separate I_hat conversion. Existing y stays exactly on this actual side of the certificate. |
| Operational receipt scale | D1 §4.1.6.1 item 7 pins actual A_hat,Z_hat,L_hat,k_a_hat,k_t_hat bits, including selected material basis. | Same rule, with exact-route A/Z evidence cross-check. New geometric endpoints do not replace any scale bits or G5b/c operation. |
| Public source/response claim | Private q_K proof is established by the accepted kernel contract. Whether each preview public functional is precisely that q_K readout must be frozen against its existing promises; no blanket exact-bit stress reinterpretation follows. | The maintained no-pressure displacement/rotation controls and source section profile preserve source-geometric response q_G. q_K-only certificates and new denominator bounds do not close this public claim. |

PP exact_straight_pressure_formulation_basis (:1024–1035) explicitly states
source OD/effective wall defines the section, and common E/nu with derived G.
IsotropicENu (:pressure_exact.rs:235–252) computes actual G_hat using Scaled
2*(1+nu) and division; pressure_material.rs:84–91 installs that G_hat.
The maintained reference carries source G=E/[2(1+nu)]. Therefore the next bridge
must identify any G_hat versus exact E/nu relation too, rather than account only
for A/I/J. This packet derives no material/operator perturbation theorem.

### Effective-wall boundary, preserved

D is the actual normalized outside-diameter bit. t is the actual normalized
**effective-wall bit** that the current producer passes onward:

1. PP normalize_quantity (:8243–8278) uses the accepted units engine on authored
   OD, nominal wall and optional mill tolerance.
2. derive_pipe_section (:9441–9471) validates normalized nominal dimensions;
   with no tolerance, t=t_nom. With tolerance m, t=RN64(t_nom−m), and the current
   finite/nonnegative/positive checks apply.
3. SourceAnnulus receives that D,t, not the raw authored decimal or an exact
   unevaluated nominal-minus-tolerance expression. actual_materials
   (result_export/src/physics_source.rs:918–939) checks the same normalized
   wall-minus-tolerance operation against evidence.
4. SOURCE_ODWALL_EXPECTATIONS has no mill-tolerance cases. It does not establish
   a stronger promise of unrounded t_nom−m. B1C preserves the existing effective
   value boundary; a stronger promise needs its own warrant.

A finite discriminator uses normalized t_nom=1,m=2^-55,D=4: actual t_hat=1,
while exact t_nom−m=1−2^-55. Those define different geometric areas. This is
a boundary illustration, not a product invocation.

Existing admission stays in place: finite positive D/t, valid nominal wall and
tolerance, SourceAnnulus's positive distinct represented radii and finite positive
accessor properties, and W1's existing primitive admissibility. The mathematical
formula below requires 0<t<D/2. It adds no thinness threshold or rounded-radius
substitution. A mathematical enclosure of an abstract input is not permission
to bypass an existing runtime refusal (including rounded-radius collapse).

## 2. Finite proof and fixed enclosure of π

Let N=128 and, for q∈{5,239},

    S_q = sum(k=0..127) (-1)^k / ((2k+1) q^(2k+1))
    E_q = 1 / (257 q^257).

Integrating the finite geometric-series identity for 1/(1+x²) gives

    S_q < atan(1/q) < S_q+E_q.

The remainder is positive because N is even, and bounded by the next integrated
power. No numerical library approximation supplies this inequality.

For α=atan(1/5), β=atan(1/239), exact tangent algebra gives
tan(2α)=5/12, tan(4α)=120/119 and tan(4α−β)=1.
The angle is positive (α>1/5−(1/5)^3/3, β<1/239) and less than 0.8.
Also π/2=2 atan(1)>1 by its integral. Hence 4α−β is in the principal
interval (0,π/2), so π=16α−4β. It follows that

    P_series,- = 16 S_5 − 4(S_239+E_239)
    P_series,+ = 16(S_5+E_5) − 4 S_239
    P_series,- < π < P_series,+.

Integer cross-multiplication checks
16/(257*5^257)+4/(257*239^257) < 2^-600.

Round these proof endpoints outward to multiples of 2^-512:

    p- = floor(2^512 P_series,-) * 2^-512
    p+ = ceil (2^512 P_series,+) * 2^-512.

The exact checker establishes p+−p-=2^-512 and 3<p-<π<p+<4. Their integer
numerators, under the shared denominator 2^512, are:

    lower = 0x3243f6a8885a308d313198a2e03707344a4093822299f31d0082efa98ec4e6c89452821e638d01377be5466cf34e90c6cc0ac29b7c97c50dd3f84d5b5b5470917
    upper = 0x3243f6a8885a308d313198a2e03707344a4093822299f31d0082efa98ec4e6c89452821e638d01377be5466cf34e90c6cc0ac29b7c97c50dd3f84d5b5b5470918

Only these fixed constants are needed by a prospective product certificate;
**no Machin series, unbounded refinement or transcendental call runs at runtime**.
The finite proof uses 128 terms per arctangent and exact rational arithmetic.
Its actual final proof numerator/denominator widths are 2968/2967 bits. These
offline proof widths are not runtime allocation or performance measurements.

## 3. Positive geometric operand enclosures

Decode actual finite D,t exactly, with 0<t<D/2. Define exact dyadic factors

    c=D/2;  ri=c−t;  P=t(D−t);  Q=c²+ri²;  G=P Q.

Then c,ri,P,Q,G are positive. The source section is

    A_G=πP; I_G=πG/4; J_G=πG/2; Z_G=πG/(4c).

Because each coefficient of π is positive, endpoints require no sign-dependent
optimization:

    c in [c,c],                  ri in [ri,ri]
    A_G in [p_lower*P,p_upper*P]
    I_G in [p_lower*G/4,p_upper*G/4]
    J_G in [p_lower*G/2,p_upper*G/2]
    Z_G in [p_lower*G/(4c),p_upper*G/(4c)],

where p_lower=p- and p_upper=p+ from §2.
These are exact rational endpoints, not binary64 conversions of source properties.
Identity P=c²−ri² and G=(c²−ri²)(c²+ri²)=c⁴−ri⁴ connects them directly to
the circular annulus section integrals. The same π occurs in all properties;
ignoring correlation in later corner enclosures is conservative.

All four nontrivial property intervals have relative width
(p+−p-)/p- < 2^-512/3. This states enclosure width, not total solution accuracy.
No minimum positive binary64 floor is imposed on a true positive property by
this arithmetic; existing runtime range/admission rules remain separate.

### Binding into B1

Associate the D/t pair and enclosure with the actual invocation, route, case/
material basis, member and source record. Preserve its exact normalizing path.
The actual A_hat/Z_hat/c_hat/J_hat and final y remain unchanged. G5b still uses
actual receipt A_hat/Z_hat and existing SI normalization.

For an action interval Q_action that encloses the **intended action of the
claim**, use B1's four corners for N/A_G and M/Z_G, and its product/division
corners for T*c/J_G. These produce a source-geometric section readout.
For the final actual y and pinned n=N_U(y), retain B1's H_n and H_U endpoint
distances, unchanged bare b_SI, both sharper predicates, both decimal relative
predicates, final row-set classes, p512 floors, G5a and display coordinates.

If Q_action currently encloses only q_K, the resulting statement is only

    f(q_K, source-geometric section),

not f(q_G, source-geometric section). This private mixed readout can support
analysis but cannot be relabelled as the unresolved public source response.
Zero certified action still gives exact zero stress even though π has a positive
interval width. A nonzero or uncertain action does not acquire that exemption.

## 4. Fixed numerical schedule and resource dependencies

This is a bounded schedule proposal, not adoption of B1's proposed arithmetic
cap or a production memory bound. Represent dyadics as integer significand plus
checked exponent, without expanding every value onto one global fixed-point grid.
Retain Z's division by c as a positive rational denominator, not rounded division.

For finite binary64 x, x=m*2^e with m at most 53 bits and -1074≤e≤971.
Using the unnormalized decoded representation:

| Quantity | Conservative integer width | Reason |
|---|---:|---|
| D−t | 2098 bits | Align two binary64 exponents; positive subtraction cannot exceed the larger aligned input. |
| ri=c−t | 2099 bits | c exponent may be -1075; conservative alignment bound. |
| P=t(D−t) | 2151 bits | 53+2098. |
| Q=c²+ri² | 4200 bits | Both values are <2^1023 and multiples of 2^-1075; squares/sum fit the corresponding bounded span (4200 is conservative). |
| G=P Q | 6351 bits | 2151+4200. |
| A endpoint numerator | 2665 bits | π dyadic numerator ≤514 bits, plus P. |
| I/J endpoint numerator | 6865 bits | 514+6351; /4 or /2 adjusts exponent only. |
| Z denominator | 53 bits | Divide by c=m_D*2^(e_D−1); exponent remains separate. |

With this specified representation, the unreduced exponent ranges through Z are
inside [-5782,4443]; use checked arithmetic regardless. No repeated unbounded
alignment loop is needed. Extra optional normalization is unnecessary and must
not be introduced without its own finite count/size proof.

One member's shared-factor schedule is:

1. Form c by an exponent change, and D−t, ri by two exact subtractions.
2. Form P, c², ri² (three integer products), then Q by one exact sum.
3. Form G=P Q (one product).
4. Form two A numerators p±P and two I numerators p±G (four products).
5. J reuses I numerator with exponent +1; Z reuses I numerator over c.
   c/ri are their exact dyadic singletons. No additional numeric divisions.

Thus geometry alone uses two subtractions, one addition, eight products and a
fixed number of exponent/metadata operations; zero numerical refinement steps.
For grade-school 64-bit-limb multiplication, the conservative product count is

    1*33 + 1*1 + 33*33 + 34*66 + 2*(9*34) + 2*(9*100) = 5779.

This is an explicit multiply-loop count, **not an LME tariff or total work bound**:
decoding, shifts, carries, zeroing, comparisons, source binding and B1 final
comparisons still need their actual checked counters. No inherited 20B/60B price
is fabricated for this new arithmetic.

A concrete storage schedule can reuse eight 128-u64 buffers numbered 0..7:
D−t→0, P→1, release 0; ri→0, ri²→2, c²→3; Q→2, release 3;
G→3, release Q in 2; A lower→2, A upper→4; I lower→5, I upper→6.
Buffer 7 remains available for bounded arithmetic scratch; each multiply writes
to a buffer distinct from its input buffers. Scalar c² may instead stay in two
scalar limbs. J/Z borrow
I numerators and c; no extra endpoint arrays are necessary. The stated bound is
8192 logical bytes of limb elements plus two shared nine-limb π constants,
input scalars, exponents/lengths and reference metadata. An implementation must
demonstrate its exact slot reuse and output/carry scratch requirements; this
count does not cover padding, allocator capacity, call frames or caller overlap.

Either stream one member's enclosure while its matching rows are checked, or
count an explicit bounded per-member cache. Do not silently multiply scratch
by every row/case or retain all rational expression trees. M1 needs actual member/
row counts, cache multiplicity, ownership, checked source-map storage and overlap
with existing solve/publication/receipt/caller buffers. Cross-products in B1
final comparisons can exceed these geometry-only widths; they need their own
checked-size/refusal treatment. No universal 8128-bit recipe-fit claim is made.

Invalid/nonpositive geometric premises, missing D/t custody, arithmetic/counter
overflow, unsupported representation, exhausted admitted work/storage or absent
action bridge are distinct named W1 certificate refusals. They are never a zero
radius, successful proof, relaxed tolerance or silent NotCovered reclassification.
Preserve otherwise publishable ordinary output through the routing contract.

## 5. Exact remaining action/source obligation

The accepted kernel theorem gives |x−q_K|≤R_K for the admitted bit-input operator.
The source reference requires geometric I/J-dependent displacements and rotations,
and geometric A/Z-dependent stress. Changing only output denominators leaves
the stiffness-dependent action/displacement difference untreated.

The finite arithmetic control illustrates the distinction without constructing or
executing a product model: positive geometric stiffness coefficients 3π and 11π
give the exact partition 3/14. Independently rounding those coefficients to
binary64 gives the bit-input partition

    884279719003555/4126638688683257,

whose difference from 3/14 is -1/57772941641565598. This proves neither an actual
W1 defect nor actual producer reach; it disproves the general algebraic inference
that source denominator intervals alone make stiffness-dependent q_K=q_G.
The original 3π/5π control happened to preserve the ratio and was replaced by
this discriminating finite coefficient witness without changing any predicate.

For a statically determinate action, a separately proven equilibrium identity
may show q_K=q_G under its exact load/frame/source premises. The current review
and this packet establish no general recognizer or such exemption. Even one
determinate cantilever's displacement depends on I/J; an action equality would
not remove its response bridge.

**Smallest next proof question for ROOT to commission:** on the unchanged W1a
source and admitted property/material maps, can a bounded private source-to-K
bridge enclose the source-geometric displacement, rotation and required action
rows while retaining the existing final predicates and runtime/output rules?
It must bind actual D/t→A/I/J errors, the selected exact-profile E/nu→G boundary
where applicable, common coordinates/frame/loads/supports, and each recovered
row functional. It must state exactly which rows can use an independently proven
static identity and which require operator-dependent control. It must return a
finite success/refusal and cost/storage interface before implementation reliance.
B1C does not derive this bridge or modify SourceParts, the operator or schedule.

Ordinary-route truth requires its own explicit warrant reconciliation before a
candidate I_K/c definition is selected. The consulted preview limitations and
mill-tolerance test (PP lib.rs:18605–18650) confirm formulas and current rounded
effective-wall construction, not a universal public exact-I_K readout contract.
No new ideal-versus-represented owner choice is needed for exact mode: its existing
source reference already decides that layer. The missing work is proof, not
permission to replace the reference.

## 6. Checks and stopped return

_run_records/exact_geometry_checks.py is standard-library exact design arithmetic.
It proves the fixed π bracket and checks ordinary/thin/large source-fixture
geometry, four abstract arithmetic boundaries, effective-wall rounding,
a stored-area singleton discriminator, B1 signed quotient/torsion propagation,
zero action, invalid annulus premises and the action-bridge discriminator.
All 12 control groups pass. “Ordinary” here is the fixture's case label, not
evidence about the ordinary product route. The fixture's finite π approximation is
checked against its stated error warrant; it is not called exact π or substituted
as the new theorem. No fixture, expectation, 1e-9 threshold or producer byte changed.

Abstract subnormal/extreme inputs are expressly not product-admission witnesses.
No compiler, solver, model/product runtime, new host tool, maintained edit,
Git/index/API write or delegation occurred. Prior B1 and sealed RV42 remain
untouched. Minimal origins, failed-control history, arithmetic and informational
inventory are preserved. Stop here for RV42 backcheck; no SIF/hypot/span work,
implementation, new public truth or availability claim follows.
