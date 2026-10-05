# I31 B1 — identity-bound certificate and finite recipe derivation

**Revisable derivation; no implementation or acceptance seal.** Direct native TASK
/root/i31_f2a_certificate_b1 under ROOT HELP_HUMAN /root. Receipt
2026-10-02 19:49:05 UTC; new-analysis cutoff 20:39:05; return deadline 20:49:05.
P = projects/chirality-piping; FK = P/core/solver/frame_kernel;
PP = P/core/product_physics. Source pin:
49034a940f3f8cd3f3da4d4cbc839943b808063d. Inspected NUM HEAD:
6a964324bcdb33cccc722ce00f6d8f4be390d592. Origins/checks are in _run_records.
No compiler, solver, product/model runtime, host-tool development, Git/index/API
write, maintained edit or delegation occurred.

## Result and remaining boundary

B1 supplies a provenance-checked view specification, a source/operand map,
and finite certificates for identity/sign rows, unit conversion, N/A, M/Z and
the actual T·radius/J recipe. The inequalities are proved below for the stated
operand enclosures. Seventeen finite arithmetic control groups pass independently
of product arithmetic. They establish neither actual W1 selection nor availability.

The accepted mechanics truth and receipt coordinates are identifiable. A
blanket rule treating every rounded stress/geometry operand as exact is not.
**T-Z and T-G below remain open before source-truth reliance.** Also open are
the implementing product-to-kernel maps, allocation composition and fresh
independent math/source review. These gaps are not resolved by a passing abstract
example. The accepted R7/A1 kernel theorem is a premise, not re-proved here.

## 1. What is exact, and what remains undecided

“Exact” names a quantity and a contract, not an entire route.

- **K truth — accepted:** D1 §4.1.1–2 defines a mechanics problem with binary64
  node coordinates, y_reference, E,G,A,Iy,Iz,J, spring stiffness, prescriptions,
  station fractions and individual nodal-load terms lifted exactly. Its frame
  and physical chord length converge to the exact geometry of those coordinates.
  FK source.rs:1–21,90–169 implements that input boundary. An accepted radius
  bounds q* of this declared bit-input mechanics problem. It does not prove that
  admitted A/I/J equals a real-π annulus reconstructed from authored dimensions.
- **Product provenance — required:** capture → actual normalized model → actual
  selected material basis → actual built section → SourceParts must be recorded
  without substituting a base material, another case or reconstructed rounded K.
  PP normalizes quantities through its units engine (:8243–8278); direct and
  temperature-interpolated materials differ (:6500–6563,:9210–9238). Final admitted
  E/G bits are exact K operands even when upstream construction rounded. Source
  unit re-entry creates a new admitted source if these bits differ.
- **Operational receipt truth — accepted:** D1 §4.1.6.1 item 7 pins A,Z,L,k_a,k_t
  “exactly as the product formed them” for scale reconstruction and source
  association. D2 §4.9.3 cross-checks A/Z against exact-route section evidence.
  These bits define scale arithmetic. A1 fixes y,U,n,b_SI and final predicates.
  None alone establishes zero error relative to a separately intended geometric
  denominator.
- **T-A — explicit boundary:** N*/A_K is a well-defined stress of the admitted K
  area. A_K may be a singleton when this is the adopted row truth and the actual
  member A bits match the source. If a row promises another A*, supply a strictly
  positive [A-,A+] containing it. Do not relabel an annulus promise as N*/A_K.
- **T-Z — open source-truth cell:** no blanket warrant was found that stored Z_hat
  is exact intended Z*. Legacy PP computes RN(I_hat/RN(OD/2)); exact mode obtains
  Z_hat through Scaled operations. The proof accepts either a specifically
  warranted singleton Z_hat or a justified interval for Z*, e.g. I_K/c* if that
  definition is adopted. These are different claims. Receipt scale arithmetic
  still uses actual Z_hat; no public scale change follows.
- **T-G — open geometric-operand cell:** torsion radius is produced as OD/2; its
  intended c* must be identified. For the exact half of admitted normalized OD,
  use exact rational OD/2 or directed endpoints, not an unchecked rounded half.
  J_K is an admitted K primitive; another geometric J* needs its own enclosure.
  A real-annulus J=2I does not make rounded Z_hat satisfy J/(2c)=Z_hat.
  Physical member length is not promoted to an exact bit operand.
- Redefining the mechanical operator itself using ideal A*/I*/J* changes q*,
  not just its stress denominator. Output operand intervals cannot repair that
  change of mechanics truth. Such a change is outside B1.

SOURCE_OPERANDS.md records the actual paths and B1 row associations. The theorem
proceeds with declared, justified operand intervals. An unresolved operand must
not be filled with a singleton for convenience.

## 2. Cross-crate view: ownership is not provenance

FK adaptive.rs:3460–3542 already checks source bytes, selected precision/policy,
layout shape, QuantityMeta, class/absence and radius ceiling. Its private SiRadius
contains no value and is insufficient alone as the cross-crate interface.
Preserve those checks and expose only this narrow producer-integration shape
(pseudocode, not compiled API):

    RetainedSolve::checked_si_row(index, expected_meta,
                                  expected_source_bytes, expected_precision)
        -> Result<SiRowView<'solve>, CertificateIssue>

    SiRowView<'solve> { private owner: &'solve RetainedSolve,
                       private index: usize }
    // Read-only access to the owner's actual row/value, QuantityMeta,
    // source/ledger identity, selected precision/policy, matching kind scale,
    // and RadiusState = Verified(finite private radius) |
    //                   InputDerived | Unpublishable(actual range outcome).

Fields and constructors stay private to FK; a narrow pub re-export is necessary
across crates. No Serialize/Deserialize, free-standing public radius record,
VerificationReport retention, per-row ExactWideSum, second Publication or source
copy is introduced. Rust pub visibility is not a security boundary.

Getters always select row and radius from the same owner/index, never a caller's
value paired with another radius. The lifetime prevents use after owner drop.
It does not distinguish two live solves, cases or bases with equal lifetimes.

PP needs a private checked case binding over the retained solve, actual captured
case/combination, material-basis identity and source-construction map. Its
constructor compares canonical source bytes (including actual load terms),
selected precision and policy to solve/evidence. It binds actual case id, basis,
invocation and complete node/member/station/support bijections. A same-stiffness
digest is insufficient. Equal-valued rows, or byte-identical sources in two
cases, do not authorize changing a product row's case/basis provenance.

The private final-row binder consumes that case binding, view(s), and the
actual immutable final product row. It checks full row id, basis_ref, kind/unit,
entity, component, location/fraction, body and recipe/sign. Every operand must
belong to that binding. No arbitrary (value,radius) input is the certificate
entrypoint. The verdict binds this row snapshot; mutation, replacement,
qualification or reordering invalidates it.

Keep existing kernel checks, explicit finite actual-value checking, and owner
source/ledger/state association. InputDerived and Unpublishable remain distinct
states bound to actual rows. Sentinel +infinity is absence, never a numeric
bound. Verified +0 is a real zero radius. Negative zero, negative, NaN/infinite,
wrong-class and absent verified radii refuse. InputDerived uses its separate
source contract, never an invented zero solve radius.

A combination owns a fresh solve/certificate; no operand radius is its result
radius by index. Long identity bytes may be compared once by the checked case
binding and rows then bound by owner/reference and metadata. If implementation
compares full source bytes per row, account the actual O(Q·B_source) cost.

**Unrun API controls:** foreign source; same-stiffness/different ledger;
same-source/different case or basis; wrong p/policy/body/kind/index/meta;
class/sentinel mismatch; duplicate final id; altered y/U; support/station alias;
missing operand; combination versus operand; partial draft drop; moves/clones;
and no radius serialization. These are future implementation tests.

## 3. Finite mathematical certificates

x,y,n are actual finite binary64 values decoded exactly. Calculations are exact
rational expressions or proved outward endpoints. H and allowances are never
nearest-rounded before comparison.

For |x-q*|≤r_x from accepted A1, use

    R_x = r_x                                             (absolute kernel row)
    R_x = min(r_x,A_exact(x,S_x),A_f64(x,S_x),|x|/10^9)      (relative kernel row).

Each minimum operand is an independently certified upper bound for the same
row/source. A decimal allowance stays rational. S_x is the actual kernel scale,
not final product S_n. R_x=0 requires exact certified zero.

**Identity/sign.** For σ∈{−1,+1}, put X=σx and I=[X−R_x,X+R_x]. Negation adds no
arithmetic error. An adopted support law Σ c_i q_i*, c_i∈{−1,+1}, can use exact
X=Σ c_i x_i and R=Σ R_i over its identified, checked finite source list.
This lemma does not infer attribution or authorize norm recomputation. A kernel
displacement/support magnitude maps directly from its own certified magnitude row.

For the closed raw unit U let a>0 map raw truth to SI:

| U | a | Pinned n=N_U(y) |
|---|---:|---|
| m, rad, N, N*m, Pa | 1 | RN64(y·1) |
| mm | 1/1000 | RN64(y/1000) |
| kN, kN*m | 1000 | RN64(y·1000) |
| MPa | 1000000 | RN64(y·1000000) |

Use pinned spellings; this adds no aliases or units. For direct/sign rows,

    e_pub=|a y−X|, e_norm=|n−a y|,
    H_U=(e_pub+R)/a,       H_n=|n−X|+R.

H_n≤e_norm+e_pub+R is also sound. The tighter expression combines signed
discrepancies before absolute value; it does not erase either discrepancy.
The mm control has n=x, e_pub=e_norm=1/180143985094819840 m, and raw error
25/4503599627370496 mm.

**Axial and bending.** Let Q=[q-,q+] enclose the signed section action and
D=[d-,d+] enclose warranted A or Z, with 0<d-≤d+. For stress q*/d*,

    C={q-/d-, q-/d+, q+/d-, q+/d+}; I=[min C,max C].

For fixed positive d, q/d increases in q. For fixed q it is monotone in d,
with direction determined by q's sign. Thus four corners enclose all values,
including negative and zero-crossing intervals. A singleton denominator is a
special case. Apply separately to N/A, M_y/Z_y and M_z/Z_z; equal stored
circular moduli do not eliminate per-component provenance checks.

**Torsion.** For T=[t-,t+], C=[c-,c+], J=[j-,j+] with c-,j->0,

    I=[min(t c/j),max(t c/j)] over all 2×2×2 endpoint triples.

Separate monotonicity proves enclosure. Discarding operand correlations only
enlarges it. Actual y still uses RN64(RN64(T_hat·c_hat)/J_hat), then its output
conversion. Never replace it with T/(2Z_hat). The exact arithmetic witness
T=1,I=3,c=11,J=6,Z_hat=RN64(3/11) yields actual bits 3ffd555555555555 versus
shortcut bits 3ffd555555555556.

**Final value for any B1 recipe.** For justified SI enclosure I=[l,u],

    H_n=max(|n−l|,|n−u|),
    H_U=max(|y−l/a|,|y−u/a|).

For every intended s*∈I these bound |n−s*| and |y−s*/a|. They include every
difference between intended recipe and actual y: producer division/multiply,
MPa publication and SI normalization. “One ulp” cannot replace these distances
or establish zero error.

B1 station truth comes from the matching kernel StationAction and exact admitted
fraction/j-side convention. We do not rebuild q* using the product's rounded
physical length. Product statics affects actual y and therefore the measured
distance. If it names another functional, binding refuses; closeness is not identity.

## 4. Unchanged final classes, bounds and coordinates

Apply closed class/entity rules to all actual final rows after preview/support
replacement, case qualification, combination rendering and summary selection.
Rebuild normalized SI maxima with InputDerived/O9 exclusions, exact max |n|,
original-operand coupling and prescribed body-extent bits. Then apply selected
p512 force/moment floors. A required unpublishable row refuses selection rather
than being omitted to improve a scale. Require total body/kind/row coverage.
A changed final row set cannot inherit kernel scales/classes.

Keep E, coupled ê and Φ=RU64(2^-438 ê) bits and matching source/body/p512 identity.
Apply Φ only to force/moment, only at p=512, after coupling and before stress
scales. B1 stress keeps the existing operational
RN64(RN64(fo/A_hat)+RN64(1·RN64(mo/Z_hat))) using actual receipt A_hat/Z_hat.
The scale is not a proof of recipe error.

For final scale S, h=2^-1074, ε=2^-64, u64=2^-53, use unchanged threshold
RN64(2^-34 S), small-scale forced-absolute rule S<2^-988, and zero-row rule:

    b0=RU64(ε S)
    b_SI=0                                         if S=0
    b_SI=RU64(b0+RU64(u64 |n|)+h)                  if 0<S<2^-988
    b_SI=b0                                        otherwise.

Absolute requires H_n≤b_SI. Relative requires all four:

    H_n≤A_exact(n,S), H_n≤A_f64(n,S),
    10^9 H_n≤|n|,   10^9 H_U≤|y|.

A_exact=ε max(|n|,S)(1+2^-21)+u64|n|+h.
A_f64 is the exact decode of the addendum's binary64 sequence:
a0=RN64(ε max), a1=RN64(a0(1+2^-21)), u0=RN64(u64|n|),
u1=RN64(u0+h), a2=RN64(a1+u1). No reassociation/FMA; all intermediates
must be finite/valid. The controls exhibit both allowance rounding directions.
Run unchanged G5a consistency checks too; they cannot replace this certificate.
b=0 needs I={n} or an equally strong exact proof, not observed zero or underflow.

Absolute S-I/UI uses (n,SI_unit,b_SI). Form adopted outward SI endpoints for b>0,
or exact SI point n for b=0; convert endpoints outward to rule/display units.
An SI point can become a raw-unit interval. A symmetric display about y requires
at least RU64((b_SI+|n−a y|)/a), labelled U as derived display data. Never show
raw y±b_SI labelled U. Relative/InputDerived point binding stays on raw y,U.
B1 implements no S-I code.

## 5. Bounded arithmetic, resource and refusal interface

A finite private evaluator suffices: two direct endpoints, four quotient
corners, eight torsion corners. No generic interval engine, root finding, sqrt,
refinement, adaptive subdivision or precision escalation is needed in B1.
Keep ratios as signed numerator/positive denominator expressions. Cross-multiply
exactly for comparison, including decimal 1/10^9; do not approximate division.

**Proposed arithmetic fence for review:** each integer magnitude/significand and
alignment span ≤8128 bits, 128-u64 fixed storage with 64 carry bits; separately
bound numerator/denominator and check exponent/index/count arithmetic. This
does not enlarge FK's existing span limit or expose its private helpers.
Check cross-product and shift size before evaluation. A cap hit is named facade
arithmetic refusal, never truncation or arbitrary-precision heap fallback.
ExactAccumulator's 68 limbs/quantum 2^-2148 support binary64 sums/products, not
arbitrary three-factor rational expressions; its public API is not proof of
sufficiency for all of B1.

No GCD loop is necessary: bounded unreduced ratios and fixed schedules suffice.
Any optional normalization needs a step cap. Numerical refinement iterations for
the proposed exact-comparison route are **zero**. If storing directed binary64
endpoints, use a proved fixed integer quotient/rounding method or finite bit
search (≤63 comparisons over positive finite binary64 patterns), never an
unbounded nextafter loop. Overflow refuses. Keeping exact rational comparisons
avoids extra endpoint-rounding availability loss.

Min/max over k candidates uses at most 2(k−1) exact comparisons; sorting is
unnecessary. Fixed passes over Q final rows, M sections, B bodies and identified
support terms have checked source-derived counts. Malformed/duplicate maps
refuse before numerical evaluation. No unbounded facade retry or kernel re-solve.

Return a separate checked facade-arithmetic account, bound to case/combination
and invocation: visited rows; recipe/corner counts; exact add/sub/multiply/
compare and limb visits; maximum numerator/denominator/alignment size; optional
quotient-search steps; source/map comparison bytes; peak scratch slots; spent
failed operation and reason. Freeze the schedule, scratch-slot ceiling and
per-operation bounds before admission and implementation.

No new arithmetic LME tariff is accepted. **20B/60B is selected kernel work
policy**, not authority to count facade work as zero or invent an LME price.
Coordinate bounded facade admission/accounting with I30/I29 and reviewed M1.
Memory inputs include actual Q/M/B/support-term/id/source-byte counts, borrowed
existing solve/publication/radii, checked maps, any retained final verdicts and
a fixed number of bounded temporary rationals. Do not retain per-row rationals,
clone solves or retain reports. Actual fixed scratch-slot count, type sizes,
stack/heap placement, capacities and overlap with receipt/caller storage remain
M1 inputs; neither 8Q nor provisional 3.75 GiB proves them.

First deterministic final-row/recipe/operand failure names: identity/coverage,
absence, invalid encoding, nonpositive/zero-containing denominator, range/span/
exponent/count/work, unresolved truth operand, or failed numeric predicate.
Preserve spent kernel/facade evidence. A covered failed recipe declines selected
publication through I30's fallback transaction; it never becomes NotCovered.
Existing outside-table/domain NotCovered semantics remain. W1 failure cannot
block otherwise publishable ordinary output merely because W1 was attempted.

## 6. Finite checks and next boundary

_run_records/exact_checks.py uses only integers and Python Fraction, with an
independent nearest-even/directed binary64 reference. Seventeen control groups
cover ties/subnormal RU; mm/kN/MPa offsets and round trips; sign/sum; signed and
zero-crossing ratios; uncertain denominator and exact zero; denominator refusal;
actual torsion versus shortcut; torsion corners; zero/small/threshold/p512 bound
behavior; both sharper allowances; changed row-set classification; and range/
span refusal. Finite samples check witnesses; §3 proves continuous enclosure.

The first arithmetic run failed because a 256-near-1 witness set lacked both
allowance-rounding directions. A fixed 1024-point mantissa-spread set replaced
that inadequate vector set, preserving every predicate; rerun passed. The
execution record retains the failed assertion. No product/source criterion was
changed and no solver reach, throughput or availability is inferred.

Named next packet proposal: **F2a-CERT-B2 — SIF/hypot and span-maximum closure**.
Dependencies: frozen B1 identity/operand bindings; T-Z/T-G and any T-A promise;
actual component→member/end and SIF association; directed norm/sqrt with bounded
steps; endpoint/station signs and exact geometry; intended unloaded-member
maximum and actual emitted maximum/summary mapping; separate ordinary rounded
length and Bernstein-control formation versus the existing supplied-coefficient
maximum certificate. Preserve k_i, 2sqrt(2),4 and pressure exclusions.
No B2 proof, dispatch or implementation is undertaken here.

Fresh independent math/source review must precede reliance: verify singleton/
enclosure warrants, foreign-case protections, signed recipes and comparisons;
carry unresolved type/resource facts to M1. B1 stops without maintained code.

