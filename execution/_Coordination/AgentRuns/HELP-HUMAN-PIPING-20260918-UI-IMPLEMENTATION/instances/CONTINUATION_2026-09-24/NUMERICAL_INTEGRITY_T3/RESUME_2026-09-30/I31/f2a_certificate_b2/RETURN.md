# I31 B2 — finite nonlinear stress and unloaded-span certificates

**Revisable conditional derivation for independent review; no implementation.**
Direct native TASK /root/i31_f2a_certificate_b1 under ROOT /root.
Receipt 2026-10-02 21:20:47 UTC; new-analysis cutoff 21:55:47; deadline 22:05:47.
P = projects/chirality-piping; PP = P/core/product_physics;
FK = P/core/solver/frame_kernel. Source pin:
49034a940f3f8cd3f3da4d4cbc839943b808063d. Brief/disposition and inspected HEAD:
d85405edcdbb894544a64051faa768db8951060c.

B1/B1C, RV42 and the independently reviewed I33 source bridge remain unchanged.
ROOT selected I33 at 9f1ef2693d, reviewed at d05bb825ff, as a conditional
mathematical basis. Its source/map/frame/load/data/scaling/strict-perturbation
premises remain mandatory. No numerical agreement waives those premises.

**Result:** the three remaining formulas have finite enclosure proofs. Under
strict unloaded straight-member W1a assumptions, circular and open-formula span
maxima equal maxima of two endpoint functions. SIF needs one associated endpoint
hypot. This is a source-functional theorem, not a reinterpretation of rounded
Bernstein coefficients. Fourteen exact abstract control groups pass. Actual
source maps, ordinary-route warrants, I35 arithmetic and implementation/resource
qualification remain prerequisites; no product reach or availability is claimed.

## 1. Source and publication table

| Recipe | Source truth and association | Actual producer y and final seam |
|---|---|---|
| Equal-factor intensified bending | q*=i*sqrt(My*²+Mz*²)/Z*, at one identified member end. Positive i is the actual normalized dimensionless user factor; A1/D2 do not authorize a different factor. Exact-profile actions use the checked I33 bridge and B1C source section if this row is actually admitted; ordinary truth awaits I36's warrant. | preview_physics.rs:628–667 evaluates i*(my.hypot(mz)/section_modulus), emits Pa and component entity, with member/end/source refs. No flexibility, axial or torsion term. The legacy component_user_stress_multiplier_review expression at PP lib.rs:11259–11482 is a distinct row, not this recipe. |
| Pressureless circular member maximum | q*=max over t∈[0,1] of |N*(t)|/A* + sqrt(My*(t)²+Mz*(t)²)/Z*, with one constant positive section on that member. §3 proves the endpoint reduction. | PP lib.rs:9583–9665 forms actual rounded length/statics/Bernstein coefficients and calls elastic_extrema. PP :4655 onward and preview_physics.rs:499–520 emit a midpoint of the witness enclosure in Pa. Keep that y; the new proof does not substitute a freshly computed endpoint maximum. |
| Pressureless open-formula summary | q*=max over the complete unloaded member of |N*/A*|+|My*/Z*|+|Mz*/Z*|. It is this open sum, not the circular-section maximum or code stress. | PP lib.rs:11233–11255 evaluates max(abs(base+abs(by)+abs(bz)),abs(base−abs(by)−abs(bz)))/1e6. :9680–9758 may add its rounded quadratic/station extrema; :4651–4749 chooses the member value in MPa. Actual preview render :582–620 retires this row and replaces it with circular maximum. Do not restore it or claim current W1 selection of the historical source-block route. |

The current circular maximum helper explicitly certifies *supplied exact f64
coefficients* and assigns formation/solution error to its caller
(loads/stress_recovery/src/elastic_extrema.rs:1–5). Its struct (:23–38) encloses
the polynomial maximum in [value_lower,upper_bound], and the witness value in
[value_lower,value_upper], with no exact/unique argmax claim. Product y uses
the latter pair. Neither interval is silently reused as a source-functional
certificate. The new interval below independently encloses the intended source
maximum; its distance to actual y accounts for coefficient formation, optimizer
gap, midpoint rounding and output conversion.

### Strict hypotheses, not a zero-net shortcut

D1 §4.2–4.3 admits straight frame members, positive global-axis ground springs,
zero rigid restraints and identified nodal forces/moments in W1a. Element loads
(weight/uniform/generated), thermal/eigen loads, pressure/thrust, constant effort,
curved/user elements, releases, nonzero product prescriptions and nonlinear
supports retain their existing exclusions/phasing. Exact profile requires an
explicitly empty pressure-region list; legacy route requires its admitted
zero-pressure scope. The 0.4.0 path is not added here.

Check actual source families, not an observed zero resultant/intensity.
Opposing unsupported loads do not acquire the unloaded-member proof.
Nodal forces or springs at a model node act at member boundaries, not as an
unrecorded interior distributed load. A source-map mismatch invalidates the
theorem even if numerical endpoints happen to match.

The non-consuming marker/branch association for an SIF row is not proof that
a consuming component is a supported W1a element. Apply the existing family
gate before this recipe. If its accepted eligibility is not established, report
that source-map/eligibility gap rather than expand “components” coverage.
A row kind in the closed table does not itself admit its model.

### SIF custody and signs

PP lib.rs:7914–7952 normalizes the bend/header/branch factors with the dimensionless
normalizer. preview_physics.rs:316–360 then requires a positive finite value.
The association requires the component at the actual endpoint node, excludes
realized curved macro components, and requires mechanics_geometry_only.
A branch factor comes from the explicit header or branch pipe reference; the
actual if/else ordering chooses header first when both name the same pipe.
Preserve that actual association, side role, normalized value, source reference,
member and end. Do not take a neighboring member's Z or parse i back from a label.

Kernel raw end actions are node-on-element. The j-side section cuts are negative
raw i-end and positive raw j-end. FK recover.rs:350–392 gives, in basic order,

    N(t)=Q0, T(t)=Q1,
    My(t)=t*Q5+(t−1)*Q4,
    Mz(t)=t*Q3+(t−1)*Q2.

Thus section endpoints are (-Q4,-Q2) and (Q5,Q3), and moments interpolate
affinely between those two section cuts. This is the intended exact functional;
the source bridge majorizes the corresponding changed source coefficients.
Its exact common frame, member/station maps and fraction premises remain required.

Using raw i-end signs incorrectly can preserve endpoint norms yet change the
interior functional; the finite control demonstrates that. Hence a passing
maximum comparison cannot replace the independent identity/sign checks.

## 2. Exact finite endpoint enclosures

Obtain signed source-action intervals from the selected I33 interface:

    I_q=[x_q−R_q−e_q, x_q+R_q+e_q].

They must enclose the intended action of the row under its actual source warrant.
For exact profile this is q_G, not a convenient q_K readout. I33's source/operator
associations and successful strict test cannot be inferred from a supplied radius.
Ordinary-route intervals must wait for I36's explicit warrant; B2 selects none.

Let A=[A_l,A_u] and Z=[Z_l,Z_u] enclose the member's intended constant section,
with A_l,Z_l>0. B1C supplies the exact-profile source-geometric intervals. Actual
A_hat/Z_hat remain the operational receipt/producer fields.

For a signed interval X=[l,u], define

    abs_lo(X)=0 if l≤0≤u, otherwise min(|l|,|u|)
    abs_hi(X)=max(|l|,|u|).

Every absolute value in X lies between these nonnegative endpoints. With moment
intervals Y and Zm, put

    d_y=abs_lo(Y), e_y=abs_hi(Y); d_z=abs_lo(Zm), e_z=abs_hi(Zm)
    h_l=sqrt_down(d_y²+d_z²)
    h_u=sqrt_up  (e_y²+e_z²).

Square/add operations also round outward in the appropriate direction. Coordinate
monotonicity of a nonnegative Euclidean norm proves this rectangular enclosure;
correlations among actions may be discarded conservatively. The lower endpoint
is not automatically zero merely because *one* moment interval crosses zero.

The required directed square-root contract is
0≤sqrt_down(a)≤sqrt(a)≤sqrt_up(a) for a≥0, with exact [0,0] at a=0.
No negative-radicand epsilon clamp, arbitrary underflow-to-zero equality or
unbounded nextafter search is allowed. I35 owns the concrete format/helper proof.
Its proposed bounded nearest-root/exact-square correction is not reimplemented
or independently approved by this packet.

For the exact positive admitted factor i,

    I_SIF=[ down(i*(h_l/Z_u)), up(i*(h_u/Z_l)) ].

For one circular-stress endpoint, let n_l,n_u be the absolute N interval:

    I_circ=[ down(n_l/A_u + h_l/Z_u),
             up  (n_u/A_l + h_u/Z_l) ].

For one open-formula endpoint,

    I_open=[ down(n_l/A_u + (d_y+d_z)/Z_u),
             up  (n_u/A_l + (e_y+e_z)/Z_l) ].

Every division/add/product uses its indicated outward direction and strictly
positive denominator. Exact arithmetic is an equivalent specification.
These formulas do not claim all rectangle extrema are simultaneously realizable;
over-enclosure is safe and may conservatively refuse.

At zero pressure the source open expression agrees with the actual formula's
intended real operation because, for b≥0,

    max(|a+b|,|a−b|)=|a|+b.

Here a=N/A and b=|My/Z|+|Mz/Z|. A pressure-longitudinal term changes a and lies
outside this packet's premise. Missing optional operands are not proven zeros,
regardless of current producer unwrap_or behavior.

## 3. Why exactly two endpoints suffice for source span maxima

For any fixed admitted source realization, t is the exact member fraction,
0≤t≤1, and physical position is x=t*L_source. The source length is positive,
so this covers the complete member. Its value need not be recomputed to prove
the following reduction: FK's exact fraction functional already supplies it.
The ordinary rounded product L_hat is never substituted for L_source.

The common straight frame is constant, section A,Z is positive and constant,
N is constant, and v(t)=(My(t),Mz(t)) is affine:

    v(t)=(1−t)v(0)+t v(1).

By the triangle inequality and positive scaling,

    ||v(t)||₂ ≤ (1−t)||v(0)||₂+t||v(1)||₂,
    |My(t)|+|Mz(t)| ≤ (1−t)(|My(0)|+|Mz(0)|)
                     +t(|My(1)|+|Mz(1)|).

Adding the constant |N|/A proves convexity of both source stress functions.
Consequently F(t)≤(1−t)F(0)+tF(1)≤max(F(0),F(1)).
Endpoints belong to the domain, so

    max_[0,1] F = max(F(0),F(1)).

This proof holds for every source realization consistent with the section/action
enclosures; it does not assume interval endpoints describe the same realization.
If the two endpoint function values are enclosed by I0=[l0,u0], I1=[l1,u1],
then the intended source maximum is enclosed by

    I_max=[max(l0,l1), max(u0,u1)].

No interior root, optimizer, subdivision, member-length square root or station
sampling is required by this *new certificate*. No endpoint is asserted the
unique maximizer: equality may hold at both ends or across a constant plateau.

The existing producer may still compute interior stations, rounded power/
Bernstein coefficients and its own bounded optimizer. Keep that computation,
its y and its stated witness metadata unchanged. Its actual cost/lifetime must
remain accounted where executed; B2's constant new certificate cost is not
evidence that the producer's existing optimizer is free or was removed.
For member loads, the quadratic example M(t)=4t(1−t) has zero endpoints and
positive interior maximum, so the endpoint reduction is not generalized beyond
the strict source premises.

## 4. Final row, scales, summary and reference rules

Certification uses the actual immutable final row after replacements and id/basis
qualification. With its raw y,U, positive exact unit map a and pinned n=N_U(y),
apply B1 without change:

    H_n=max(|n−I_l|,|n−I_u|)
    H_U=max(|y−I_l/a|,|y−I_u/a|).

SIF/circular actual rows are Pa (a=1); the open summary is MPa (a=10^6).
Keep final normalized row-set/body scales, operational A_hat/Z_hat and the
existing propagation factors:

- SIF: k_i=RU64(k_sqrt2*i), k_sqrt2 bits 0x3ff6a09e667f3bcd;
- circular member maximum: k=2*k_sqrt2, bits 0x4006a09e667f3bcd;
- open summary: k=4.

Endpoint simplification does not lower those registered factors. Keep actual
class thresholds, small-scale A1 row bounds, p512 force/moment floor placement,
and G5a. Absolute requires H_n≤b_SI. Relative requires H_n≤A_exact and
H_n≤A_f64 plus 10^9 H_n≤|n| and 10^9 H_U≤|y|. H_n=0 needs actual exact
enclosure equality; sqrt/zero shortcuts cannot manufacture it. No private
source interval is serialized as a new public radius.

**Rows and route coverage.** Count only rows actually surviving the current
producer/render path. Current preview render replaces open summaries with circular
maxima. Exact profile produces circular maxima. Historical/source-block rows
retain their own route/standing; invocation-level exact-block coexistence prevents
calling that an actual new W1 route. The open-summary theorem closes the bounded
mathematical recipe, not permission to restore retired rows. SIF remains its
one-end/component-associated quantity. Existing combination gates prohibit
intensified combinations and stress maxima (preview_physics.rs:991–1005);
B2 does not publish a combined maximum, even if an algebraic expression exists.

**Witness and ties.** Keep actual maximum-row id, basis_ref, member, component
and qualified source refs. The producer's governing-station evidence is scoped
to its supplied rounded coefficients, including local_fraction and span identity.
The new source endpoint theorem neither certifies that witness as the exact
source argmax nor authorizes replacing it with an endpoint label.

preview_physics.rs:515,684 chooses actual case rows by emitted value, then smaller
member id on equality; lib.rs:3071–3110 requires complete per-case coverage and
selects across cases using actual values, then case id, then location id.
Preserve those exact finite-value/tie rules, value/unit equality to the referenced
row, coverage and final qualification. Overlapping intervals do not authorize
reordering rows or asserting a unique physical governing member.

**D2 headline alias limit.** D2 §4.9.10 binds a headline as its result_ref row,
including that row's class/refusal. Preserve this contract; do not silently add
a new all-case source-maximum accuracy guarantee. A row's certificate does not
automatically certify the aggregate maximum by that row's radius: the finite
control has row A exact at 10 with radius0, row B at9 with radius2 and truth11;
published winner A differs from the true aggregate maximum by1.

If a separately defined consumer claim requires the full source aggregate and
all its constituent source intervals exist, the same max-of-intervals formula
gives [max_i l_i,max_i u_i], with two comparisons per added constituent. That
claim needs its own comparison against its unchanged applicable predicate.
A missing interval, including an ordinary not_required case, cannot be silently
omitted or treated as a zero-width source certificate. This conditional lemma
does not alter D2's alias standing or invent that extra public guarantee.

## 5. Operation/count contract sent to I35

I35 owns the concrete bounded arithmetic format; B2 supplies only operations and
proof postconditions. No alternative format, LME prices or runtime series is
selected here. Count exact/outward failure work as well as successful work.

| One evaluation | Abs-range calls | Squares | Adds | Directed sqrt | Positive divisions | Other products |
|---|---:|---:|---:|---:|---:|---:|
| Moment hypot | 2 | 4 | 2 | 2 | 0 | 0 |
| SIF endpoint | 2 | 4 | 2 | 2 | 2 | 2 (factor i) |
| Circular endpoint | 3 | 4 | 4 | 2 | 4 | 0 |
| Open endpoint | 3 | 0 | 4 | 0 | 4 | 0 |

Each abs-range uses at most two absolute values and four comparisons, excluding
input-shape checks. Each member maximum uses two endpoint evaluations and two
max comparisons. Zero/perfect-square and reused-value shortcuts may save work;
do not rely on those savings without checked accounting.

Let s be actual admitted SIF rows, m_c circular-maximum rows and m_o surviving
open-summary rows. Without cache sharing the upper operation counts are:

    square-products = 4s+8m_c
    sqrt evaluations = 2s+4m_c
    SIF products = 2s
    divisions = 2s+8m_c+8m_o
    additions = 2s+8m_c+8m_o
    endpoint-max comparisons = 2m_c+2m_o
    abs-range calls = 2s+6m_c+6m_o.

Add existing B1 final distances/predicates, section formation, source bridge,
identity/source scans and actual producer work separately. Do not charge them
again under a misleading nonlinear subtotal or count them as free.
s comes from the checked actual association inventory; a simple finite structural
ceiling is 2*M*C for M members and C component records, subject to checked products,
but actual source maps/eligibility still govern. No physical or input-size limit
is selected by that count.

Stream one member's two endpoints and each associated SIF row. Inputs are at most
six signed action intervals, one positive A/Z pair and one factor per active SIF
association; source/bridge objects remain borrowed under their owner. A running
maximum interval needs two stored endpoints, not every station or a subdivision
heap. Norm caching is optional only for identical source/member/end/operand
identity, with its storage/multiplicity explicitly counted. I35/M1 must bind
actual endpoint sizes, sqrt-square-check scratch, directed-rounding temporaries,
counts, failed paths and caller/receipt/solve overlap. No allocator/RSS bound
follows from these scalar counts.

Named refusal conditions: wrong/absent source or row association; missing bridge/
ordinary warrant; unsupported producer/functional; nonpositive section enclosure;
invalid factor/radicand; missing finite certificate; range/span/exponent/counter/
admitted-work exhaustion; nonfinite output/normalization; or failed final predicate.
Do not clamp, widen public bounds, silently relabel covered rows NotCovered,
drop a required member to improve a maximum, add a retry/solve, or block otherwise
publishable ordinary output merely because W1 certification failed.

## 6. Finite checks and open implementation interfaces

_run_records/exact_b2_checks.py uses integer/Fraction arithmetic and an offline
integer-square-root bracket as an independent test oracle, not a runtime format.
Fourteen passing groups cover:

- signed abs ranges, exact zero, perfect-square/irrational/tiny/large root bounds;
- 36 hypot rectangle points; 27 SIF and 243 each circular/open endpoint points;
- 68 unloaded affine endpoint/convexity checks and the raw-i-end sign discriminator;
- 45 pressureless open-expression identities; pressure and quadratic-domain controls;
- supplied-coefficient versus source maximum; max-of-two intervals; headline
  radius and tie distinctions; exact nonlinear zero and three operand refusals.

The continuous proofs are §§2–3, not a claim that sampling establishes them.
No model, solver, product runtime or host tooling was executed.

Remaining concrete interfaces before reliance are the checked source/row/
component/section/bridge joins, I36 ordinary-route warrants, I35 scalar helper
proofs and fixed-count implementation, actual final-row/metadata/alias validation,
M1 storage/cost composition and protected product qualification. These are
implementation/source-association prerequisites, not a missing optimizer theorem.
No new public truth, combination domain, source formula, criterion, or availability
claim is selected. Stop for fresh independent B2 review; prior packets are untouched.

