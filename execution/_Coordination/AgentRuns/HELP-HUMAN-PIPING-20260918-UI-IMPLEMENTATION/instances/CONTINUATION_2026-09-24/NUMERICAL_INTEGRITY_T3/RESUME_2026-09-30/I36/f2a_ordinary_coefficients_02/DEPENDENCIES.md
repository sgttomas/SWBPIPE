# I36 — ordinary adapter, finite work and storage inputs

This supplements I35/M1's accounting; it does not adopt an arithmetic format,
resource allowance, LME tariff, Rust layout or actual availability. All work,
including failed prefixes, belongs to the same immutable source/final-row binding.

## Private source-view delta

A closed ordinary material variant must distinguish:

| Variant | Borrowed proof inputs and required identity |
|---|---|
| Base | actual invocation material population (including request-material selection), normalized base E/G, actual resolved/built material and K E/G bits; no selected point |
| ExactPoint | same, plus actual case exact-id selector and matched point identity; normalized point E/G, no base fallback; actual resolver validity |
| Interpolated | actual case temperature selector; normalized T; selected adjacent point identities and normalized Tlo/Thi; normalized Elo/Ehi/Glo/Ghi; actual E_hat/G_hat and selected resolver validity; full point population/ordering witness needed to prove adjacency and absence of duplicate temperatures |

Common fields: ordinary route discriminator, captured invocation/case/basis
identity, normalized source owner, member id and material ref, actual D and
DerivedSection effective-wall bit, actual A/Iy/Iz/J/Z/c bits, K-source identity,
and member/node/DOF/station/support/row bijections. Bit identity checks tie actual
resolved E/G and built A/I/J to CK. The binding also preserves alpha slot and
selection validity; this numerical proof cannot accept a selection the ordinary
resolver rejects. The normalized population and selected material objects must
be borrowed from the same run or independently validated by their full identity.
Provenance strings and identical value bits alone cannot join two cases/bases.

The existing resolver sorts a temporary list (`lib.rs:9100–9115`), and its pair
references are local; a future certificate cannot pretend those are retained
public records. Prefer recording/borrowing stable selected point ordinals in a
checked internal selection view during the actual resolver/build. Alternatively
prove the pair from immutable normalized inputs with a bounded scan that checks
all point temperatures and the actual selection rules. Any new map, retained
ordinal array, source sort or validation scan needs its actual allocation/work
in P1/P2. No new sort or point-population copy is priced as free here.

If the same basis serves several members, source selection may be validated once
per distinct actual basis only with an explicit case/material-to-basis map. There
is no such sharing assumed in the member numerical counts below. Long identities
are compared once in the checked case/source constructor where possible; account
all repeated byte visits actually used. Same-stiffness is not same source/ledger.

After this view is validated, the certificate can use read-only actual values;
public code outside the validating constructor cannot pair arbitrary interval
operands with an unrelated RetainedSolve. The existing checked kernel row,
verification factor scale, upward body bound and data/zero facts remain required.
Keep final row identity/unit/value/recipe frozen through verdict consumption.

## Finite material-operation schedule

One interpolated material construction, with four-product numerators streamed
one property at a time:

| Work | Upper count |
|---|---:|
| exact binary64 lifts: Tlo,T,Thi,Elo,Ehi,Glo,Ghi,E_hat,G_hat | 9 |
| downward/upward h=Thi−Tlo | 2 directed subtractions |
| four exact products for E and four for G | 8 directed multiplications (exact by 106-bit bound) |
| downward/upward four-term numerator rounding for E and G | 4 `round_toward` entries on fresh four-term sums |
| source positivity sign checks, before lower rounding | 2 exact `signum` calls, included in the round-entry bound below |
| two endpoint divisions per property | 4 directed divisions |
| H_E/H_G inclusion of actual resolved values | 4 endpoint min/max comparisons |
| source/material/denominator/finite checks and identity checks | explicit auxiliary work, not hidden in the calls above |

The numerical total is **14 directed binary calls plus 4 four-term rounding
entries =18 charged scalar entries**. Base/point variants need lifts, source
identity and positivity comparisons but no interpolation arithmetic. Using 18
per member construction is a conservative common upper, not spent work.

### Four-term rounding entry: distinguish the longer sum grammar

`directed::round_toward` already exists. It rounds the exact sum to nearest,
adds the negated nearest result, checks the exact side, clears, and may take one
adjacent step. For this entry the initial fan-in is **four**, not two. Count:

- at most 7 raw additions: four inputs, nearest residual, and two step terms;
- at most 7 anchor shifts, at most 2 exact net/round operations;
- at most 2 sign comparisons (initial positivity on the lower pass, then side);
- one existing small-factor step operation, and all clears/zeroing/copies;
- no further loop, retry, new scalar arithmetic helper, or rational denominator.

With I35's existing conservative 128-limb work bounds, one such entry has
`term_limbs <=7*(18+128)+16=1038`, `shift_limbs <=7*256=1792`,
`net_limbs <=6*128=768`, `rounded_limbs <=2*128=256`, total **3854<4096**.
Its raw term span is at most 4199 before clear and at most 1025 for the step,
as proved in RETURN. This is a fresh-local bound; aggregate counters must still
be checked. I35 must explicitly add this seven-add entry to its call grammar and
count it, rather than citing its earlier five-add premise unchanged. It needs no
new integer width or scratch-array type. Actual helper/caller correspondence and
failure accounting remain review obligations.

### Delta to the existing three-member-pass schedule

I35's proposed exact-profile material row used `2 add +2 div=4` scalar calls per
builder. Replacing that row with the ordinary interpolation row gives **+14
entries per construction**, or **+42m** under its three constructions per member.
Do not add B1C's exact-integer 5779 count to I35's outward geometry schedule.
The four source coefficient intervals, four CK products, eight endpoint
differences and existing I33 contractions retain I35's counts.

In F3 only, the represented Z cover needs **2 additional directed divisions per
member** for I_K/(D/2), plus two endpoint comparisons with Z_hat. Preserve the
result for that member's rows, then discard it. If separate bending-axis source
association ever permits distinct Iy/Iz, use separate covers and count four
divisions; the current ordinary circular mapping must validate identical Iy/Iz.

For a B1 stress row, separately evaluate represented and source recipes, then
hull before raw conversion and final distance tests. Beyond I35's existing
source recipe upper, reserve **2 calls for Q_K endpoints plus at most 32 calls
for the represented torsion recipe =34 per B1 stress row q_sigma**. N/A or M/Z
use fewer; the torsion upper is deliberately common. SI hull formation is two
comparisons. Final raw conversion, Hn/HU, final exact comparators and unchanged
scale/class checks still execute once on the hull. Direct component/mapped norm
rows need no second action interval because Q_K is contained in Q_S; their
observable checks remain separately required.

Thus, for the B1 portion only, an ordinary common upper against I35's displayed
D is **D +44m+34q_sigma**; when substitution is performed,
`D_ordinary =1226m+4b+49s+2(k+a)+47q+34q_sigma`.
The 4-round-entry type must be present in that count, with its own work vector.
B2 rows are excluded from q_sigma: B2/I35 must price any second nonlinear recipe
and hull under their reviewed source/represented readout plan. This packet does
not assert that the old 47q bound covers B2 or the observable reader operations.

Conservatively add at most **27m** interpolation-source lifts across three
builders without claiming I35's existing 32-lift builder reserve already covers
them. Actual A_hat/J_hat/c_hat can be lifted on demand for represented B1 rows
(<=3q_sigma extra lifts), with Z_hat and I_K lifted once in F3 (<=2m). Min/max,
finite/positivity checks, source id/byte checks, selection scans and zeroing stay
explicit auxiliary populations. An implementation may reduce these upper counts
only with its actual reuse/custody correspondence.

## Logical scratch and lifetime inputs

One material scratch plan uses **21 endpoint slots**, in sequence:

- 0–8: nine decoded source/actual values;
- 9–12: four exact products, reused between E and G;
- 13–14: h endpoints;
- 15–16: current numerator endpoints;
- 17–18: H_E; 19–20: H_G.

A single existing ExactWideSum and I35's primitive/subcall scratch are active at
a time. No four-term-sum clone or second simultaneous accumulator is needed:
rebuild the upper sum from the four saved products. Consume/check/copy its work
before reuse/drop, including failed prefixes; `clear` preserves counters and
must not silently start a new accounting scope.

After material construction, release slots 0–16; only four H_E/H_G endpoints
remain while geometry and coefficients are formed. A later implementation may
overlay these 21 slots with I35 MemberScratch 0–31, but must map the geometry/
chord liveness with the retained four endpoints explicitly. This packet does
not assume the compiler or an unspecified slot allocation makes that overlap
safe. If not overlaid, reserve 21 extra endpoint slots (2,877 field-payload bytes
under I35's 137-byte accounting) in M1; these are logical bytes, not Rust layout.

F3 additionally retains two represented-Z endpoints for the current member's
rows. Actual A/J/c can be borrowed as binary64 and lifted in the existing row
scratch at use. During a dual recipe, retain two endpoints of the first recipe
while streaming the second, then reduce their hull in place; no corner array,
per-row interval cache, new publication, serialized interval or source copy.
M1 must account those **two member endpoints plus two row endpoints** unless a
concrete reuse map proves they fit existing reserved slots. A conservative extra
constant reserve is therefore **25 endpoints =3,425 logical payload bytes**, not
an adoption of a byte allowance. It does not multiply by m or q in this streamed
schedule. Concurrent cases/callers multiply/overlap owners only as P3/P4 proves.

Source selection maps and retained ordinal/identity records are separate from
these arithmetic endpoints and remain source-population-sized. Actual Rust
alignment, capacities, temporaries/copies, early-error liveness, cache/caller
retention and aggregate host memory remain P1–P5 facts to qualify.

## Refusal and integration handoff

Private refusals distinguish source/selection/row mismatch; nonfinite source;
nonpositive exact P_E/P_G; invalid positive geometry/denominator; missing K row,
scale/B/data facts; arithmetic/count/range failure; strict-alpha failure; and
failed final predicate or observable contract. They preserve ordinary behavior
and spent work, and do not claim singularity or change a covered row's class.

ROOT owns I35/M1 integration and RV47/math backcheck. The polynomial numerator
uses only existing arithmetic; the new work is a closed source expression,
source-selection view, its count/lifetime plumbing and the reviewed dual
readout composition. No solver, public source promise, output bits or protected
comparison is changed by these dependencies.
