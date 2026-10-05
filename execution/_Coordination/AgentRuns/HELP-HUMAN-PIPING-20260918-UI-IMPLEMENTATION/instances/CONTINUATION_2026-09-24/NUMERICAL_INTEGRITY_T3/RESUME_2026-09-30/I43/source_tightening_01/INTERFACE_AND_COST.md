# I43 — proposed private seams and finite consequences

Proposal only. Existing no-new-solve counts/storage do not cover this route.
No tariff, permit value, M1 admission, facade LME price or kernel charge is
selected. Keep all actual spent work, failed prefixes and original numeric
causes. Source-sized quantities below are from the admitted owner: N free
DOFs, C constrained DOFs, b blocks, m members, s station visits, a support-term
occurrences, k springs and L ledger-term occurrences.

## Same-owner factor application

The actual source seam is retained/factor.rs:640–668 `solve_scaled`. It borrows
immutable RetainedFactor, accepts a local WideContext at the factor's width,
and returns a new vector. Its scales and ordering are the same factor's private
state. The current SourceBridgeView already binds the matching successful
P=2p cache/state and free ordering, but does not expose a correction call.

Add an owner-private entry taking that validated view and its internally
constructed source-residual RHS. Select exactly the matching P256/P512/P1024
cache slot. Never rebuild/import/clone a factor, substitute candidate p, call
condition estimation, or expose a public caller-supplied center/S/B/report as
proof. Explicitly validate RHS length N and exact checked association before
indexing. Cast the chosen finite p1024 midpoint to P once with a specified
rounding rule. The midpoint and cast are proposal choices, not a claim of
residual enclosure. Source residual re-evaluation covers their error. Form the
midpoint by one counted endpoint addition and exact half shift per component,
then the one specified P-rounding. Center formation adds one counted p1024
point addition plus exact radix shift per free component. Conversion, range
checks and every limb/copy traversal are additional explicit finite visits;
they do not disappear into the solve call count.

Allocate/own a fresh local WideContext at P outside a Result-producing closure
that makes exactly one `solve_scaled` call. On every return, including arithmetic
failure after a partial triangular sweep, collect **that same context's actual
AttemptWork/status** before propagating its result. This call itself has no
ExactWideSum; source interval entries own/collect their separate actual sums.
A wrapper that uses `?` before collecting the local context is invalid. Join
non-E work before extracting any successful vector. Preserve simultaneous
numeric and accounting failure independently. Nothing is retrospectively
reconstructed from a theoretical operation count or counted twice.

Let ell=sum_i(i-first[i]) for the retained skyline profile. `solve_scaled`
performs ell forward multiply/subtract pairs, N diagonal divisions, ell backward
multiply/subtract pairs, then an N permutation. Therefore its scalar call upper
(and full successful loop count) is **2ell Mul + 2ell Sub + N Div**. Those are
WideContext calls at P, not p1024 facade DM/DS/DD entries and not an LME tariff.
Each operation can fail with a shorter recorded prefix. The context's actual
rounded arithmetic is accepted only as a trial-center generator; the later
source residual proves accuracy. The repeated `get`/profile/order/index reads,
initialization, allocation and final permutation are separately counted visits.
Checked ell/count/layout calculations precede execution.

Current solve_scaled constructs an N-element permuted x by collect; holds x
through both sweeps/division; allocates N-element out while x is still live;
and permutes into out. Its borrowed RHS is a third N-element owner in the
proposed call. Existing allocation failure behavior and capacity/profile must
be qualified or adapted without changing the arithmetic loop, rather than
assuming the two allocations are free or fallible. No solve call was executed
in I43; no claim about its actual resulting correction, prefix or allocation
capacity has been measured.

## Fixed signed arithmetic and source passes

Source coefficients retain I41's actual route-specific member entries. The
current positive `NumericWork::mul` helper is not a signed interval multiply.
Add a signed pair operation using all four corners, lower/down and upper/up,
with fresh actual producing contexts/sums. A conservative primitive grammar is:

- Interval add/sub: 2 DA/DS calls.
- General signed product: at most 8 DM calls (four corners per direction).
- Signed quotient with strictly positive denominator interval: at most 8 DD.
- Range square: 2 DM after exact abs-range decisions, lower zero if crossing.
- Interval square root: 2 calls to existing `directed::certificate::sqrt_endpoint`.

The sqrt helper already exports its producing context/sum work. Collect it
once on every exit and add a checked **DQ** entry category; the existing six-entry
NumericWork vector does not already account for DQ. No host sqrt/hypot enters
this private arithmetic. Each primitive's existing fixed p1024 span/exponent,
16-limb and inner-loop limits remain; failure is a certificate refusal.

A concrete common exact-frame enclosure uses:

1. Three signed coordinate differences, then norm/normalization for ex and L.
2. Dot(yref,ex), signed subtraction yref-projection*ex, norm/normalize for ey.
3. Signed cross(ex,ey), norm/normalize for ez, matching the source law.
4. 1/L and six products ey/L, ez/L; B entries are signed copies from the axes
   and these products; H entries are signed copies of 1/L and one.

For norm3 use three range squares, two interval adds, one interval sqrt, then
three interval divisions for normalization. All three lower norms must be
positive. This fixed frame schedule uses 9 interval subtractions, 18 general
interval products, 8 interval adds, 9 range squares, 3 interval square roots
and 10 interval divisions, i.e. conservative scalar upper
**DA16, DS18, DM162, DD80, DQ6**, plus fixed lifts/shifts/comparisons and frame
index/field visits. Four additional interval divisions put the source
constitutive numerators over the actual enclosed L. Conservative corner counts
are safety uppers only; actual producing work is charged. Exact axis values
can round exactly through this same method without an axis-specific branch.

Keep F1 law validation. The minimal direct schedule with uncached member data
then has four coefficient builds/member: F1, eta+initial source residual,
recomputed source residual, final source recovery. The signed frame is built
in the latter three. This differs from I42/I35's three coefficient builds and
must replace their count contract before reliance. No m-sized coefficient or
frame cache is assumed.

The initial and final residual passes each use fixed B(48), D(10), B^T(48)
signed contractions: at most 106 interval products and 106 interval additions
per member, plus at most 12 signed scatter additions. The eta part can retain
I42's original nonnegative scale-vector contraction; original v is unnecessary.
No-data only proves free motion zero. Even if no factor call is needed,
source recovery retains actual constrained prescriptions and every constrained
load offset; a fully fixed body is not a zero-action exemption.
Load accumulation visits each actual individual ledger term (not only netload);
springs add exact common-law terms. Scaling and max reductions visit N and b.
The independent residual is recomputed from the chosen y, not from saved rounded
K equations or a proof-by-diagnostic string.

Final member recovery shares B*y and D*B*y between H^T (16) and B^T (48), giving
at most 122 signed interval products and 122 additions, plus global reaction
scatters. Gather y +/- s*epsilon by actual free/block maps or exact prescription.
Station moments keep two signed products and one addition each; compute t-1
once per station. Axial/shear/torsion use their proper end rows. Constrained
load subtraction, springs and support-term mappings are additional explicit
L/k/a occurrences. Each three-component magnitude uses at most DM6, DA4, DQ2
plus abs-range comparisons. Existing final native predicate counts and later
PP raw-SI/observable gates remain additional and unchanged.

Signed dot products must cancel before any absolute maximum. Directed interval
addition can widen them but never permits an inward endpoint. Keep lower and
upper contexts distinct with truthful prefix collection. There is no runtime
pi series, adaptive frame refinement, correction iteration or precision retry.

## Storage owners and overlap

Use one new free-position point-center array of N p1024 endpoints. Exact
prescriptions are borrowed, not copied into a second model. Residual lower/upper
arrays require 2N endpoints; old eta needs N only until alpha is finalized.
Alpha/epsilon need 2b endpoints. Drop eta before correction; convert the initial
residual to one N-element P-width RHS and drop its two endpoint arrays before
calling the factor. During solve, RHS+x+out can overlap: **3N P-width values**,
plus their headers/context and the borrowed old factor/solve owners. After the
call, drop RHS; converting out into the point center overlaps N P-width and
N p1024 values. Source residual recomputation overlaps center N and residual
2N endpoints. Recovery can reuse the residual storage for 2C reaction endpoints.

Thus a proposed streamed arithmetic-array upper is
**N + 2*max(N,C) + 2b p1024 endpoints**, in phases separate from the 3N P-width
solve peak, with the stated conversion overlap. It is a logical owner schedule,
not measured allocator capacity, a compiler overlay or complete memory profile.
A diagnostic that collects q result intervals adds 2q endpoint capacity; a
baseline comparison may overlap the old I42 diagnostic owners. Do not copy that
overlap into a claimed streaming facade bound.

Frame storage can use nine axis intervals plus invL and six transverse/L
intervals (32 endpoint slots), with B/H entries generated by fixed index/sign
rules; no dense 12x12 production matrix is needed. Norm/projection/cross/frame
constructor temporaries, I41 coefficient scratch, signed-corner temporaries,
12 gathered displacement intervals, six basic-action intervals and local/global
end intervals all have fixed liveness that the next code must enumerate. I35's
177-slot reservation cannot be inherited without that new enumeration.
Existing sum/core scratch, typed return/counter/error/context headers, casts,
source-owner/law borrows, row mappings, caller/receipt overlap and failure-path
allocations remain explicit profile obligations. No total byte fit is claimed.

## Smallest implementation and witness manifest after design review

- New private `product_certificate/source_residual.rs`: fixed signed frame,
  source residual, any-center theorem arithmetic, direct source recovery and
  distinct source-enclosure result/work types. Keep original bridge as frozen
  comparison during development.
- `adaptive.rs`: one narrow checked SourceBridgeView correction seam, actual
  matching cache dispatch, conversion/allocation preflight and same-context
  spent return. Call existing factor.rs solve_scaled without changing its
  factorization or primary schedule. Any allocation-wrapper change receives
  its own explicit hunk and parity/failure review.
- `product_certificate.rs`: narrow registration plus signed/DQ collector seams
  as needed; do not repurpose NativeSourceError.source_error or change public
  retained_api/source structs/serialization/receipt fields.
- New focused source-residual test/oracle files and minimal registration. Freeze
  raw actual RHS, returned correction, center, rho, alpha/beta/epsilon, rows,
  classes/bounds and all work/status. Verify bits unchanged before/after.

First native controls: captured zero/loaded, deliberate wrong center, failed
factor/conversion/context prefix, foreign source/factor ordering/P, nonzero
prescribed columns, cancelling individual ledger terms, mixed no-data proof and fully fixed nonzero constrained loads/prescriptions,
near-parallel frame refusal, missing positivity/B/uniqueness, unsupported rows,
and simultaneous numeric/accounting failure. The independent source oracle
must check the **actual** correction/residual intervals, not the constructed
I43 center. Full source/PP origins and resource permits stay outside this native
slice. A later protection/admission/availability claim requires its owning
checks and decisions; this manifest grants none.
