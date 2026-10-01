# V-EXACT — finite source derivation

**Symbolic successful-path envelope derived; no whole-process E_max or numerical
acceptance.** Frozen source40129a225d73860ac2a53da9a2fa73869df668f3. X below
means VR/src/exact.rs; immediate callers are lane.rs, compare.rs and floor.rs.
Original global/stage/admission contracts and all213 cases remain unchanged.

## Descriptor and capacity interface

Each Exact state carries separate upper bounds `(B,l,c,p2,p10)` for mantissa
bits, normalized u32 length, retained u32 capacity and signed exponent intervals.
`l<=ceil(B/32)`; **c must never be replaced by l**. Nat/Exact/tuple wrappers are
inline values; only their Vec buffers are charged. Define R(x)=4*c(x).
An input's R is paid by its caller; H_op below is *additional* peak ownership
inside that operation, including the returned result. Evaluate each equation
in ordinary requested bytes and in move-counter bytes separately.

source02's pinned Rust1.97.1 recurrence is `c'=max(2c,len+additional,mu)` on an
actual grow; mu=4 for u32/u64. Use the source02 mu(U) rule for slice-reference
elements until their public stride U is bound.
For known initial-capacity upper c0 and a bound K on every required length:

```
G_s(c0,K) = c0                         if K=0
          = max(c0,mu(s),2K)           otherwise
A_s(c0,K) = s*G_s(c0,K)                retained bytes
Hgrow_s(c0,K) <= A_s(c0,K) + epsilon*s*K
```

Here epsilon=0 for requested current/peak and1 for the move peak. At a growth
old_capacity < required_length <= K, proving the active-old addition. Only that
currently growing buffer gets the addition. If c0 is only an upper, do NOT use
`K<=c0` to conclude no growth: actual c0 may be smaller. A precise actual
recurrence can tighten this bound. Known clone/reserve/no-growth sites use their
exact constructor below instead of this generic upper.

Fresh push-only capacity P(K)=0 for K=0, otherwise max(4,next_pow2(K)).
Fresh Clone copies **logical length**, requesting4*l bytes. pop/trim preserves
the old backing (X56-60); this includes a zero result with nonzero capacity.
Vec<Exact> containers are not manufactured: these expressions use local values.

## Fixed input descriptors, freshly checked without a solver

DESCRIPTORS.json binds ten committed family files by hash and records all213
cases,27,752 reference rows and5,085 value-control entries. The9,522 distinct
reference/selected-scale/control strings have259 distinct memory shapes
`(digit_count,p10,is_zero)`. Shapes are grouped per case with multiplicity and
first exact row/control location; original values remain in the hashed files.
Scale selection follows cases.rs137-140, including row-level RF-CANCEL overrides.
All control keys resolve to actual rows. No case/model/value was generated.

Every selected string has successful ASCII decimal syntax; parsed explicit
exponent and `explicit_exp10-frac_digits` fit i64. Across this fixed set,
digits<=40, p10 in[-2903,273], chunks=ceil(digits/9)<=5. For parse, B<=4d
since10^d<2^(4d); l<=ceil(d/8). Leading zeroes do not reduce d's capacity bound.
Sign has no extra child. Reference and scale magnitudes are below10^298 by
these input descriptors; this does not prove derived floor/observation values
remain finite.

Finite binary64 input has B<=53,l<=2,c=2,p2 in[-1074,971],p10=0 (X289-300).
Even zero from_f64 retains its original two-u32 allocation after trim.
pow2(k) also starts with c=2 but l=1. from_u128 is test-only in this caller set.

## Primitive allocation and bit rules

| Operation / source | Returned state and additional peak, excluding input owners |
|---|---|
| Decimal parse X265-285,67-90 | d concatenated digits: String retained F(d)=max(8,2d), active old<=d (source09). J=ceil(d/9), chunk-view element stride U=size_of::<&[u8]>(). chunks retained Q=A_U(0,J), construction H_Q=Hgrow_U(0,J); its IntoIter backing survives the complete Nat loop. Nat starts empty, c<=P(ceil(4d/32)), H_N<=4P+epsilon*4ceil(4d/32). Parse peak is max(F(d)+epsilon*d, F(d)+H_Q, F(d)+Q+H_N). Return only the Nat buffer; digits/chunks drop. No clone of input text per chunk. |
| clone/abs/negated/scaled X315-325 | Return c=l(input), same B/l; H=R(return). abs clears sign; negated flips sign; scaled adds k,j to exponent intervals. Zero clone copies length0 and can release the original capacity only when the original later drops. |
| shl(k), X121-143 | Zero branch returns empty. Otherwise w=floor(k/32), r=k%32; exact initial capacity w from vec![0;w], then K=w+l+[r!=0]. Return B'=B+k,l'<=ceil(B'/32), c'<=G_4(w,K); H<=Hgrow_4(w,K). trim preserves this c'. In r=0 branch one extend reserves from w; do not pretend this was a fresh empty push Vec. k=0 is still a new shifted buffer. |
| mul_pow10(j), X146-156 | Fresh clone initially requests4*l(input), then in-place mul_small steps; B'=B+4j, K=ceil(B'/32). For j=0, c'=l,H=4l. Otherwise c'<=G_4(l,K), H<=Hgrow_4(l,K). A known-zero input returns length/capacity0. No allocation per iteration; only growth of this same buffer. |
| Nat add, X158-175 | Exact reserved c'=max(lx,ly)+1; B'<=max(Bx,By)+1; H=4c'. No growth beyond reservation; cancellation/trim does not reduce c'. |
| Nat sub, X178-196 | Exact reserved c'=l(minuend); B'<=B(minuend); H=4c'. Called only after cmp chooses a nonnegative subtraction. Use max(lx,ly) when that branch is unknown. |
| Nat mul, X198-223 | Zero branch empty. Otherwise q=lx+ly+1: exact acc request8q. For a conservative owned-map contract retain acc and a possible output buffer together; returned u32 c'<=max(2q,4), including inherited source bytes if reuse occurs. H<=8q+4max(2q,4)+epsilon*4q. Return B'<=Bx+By,l'<=ceil(B'/32), but trim leaves c'. This deliberately pads generic output growth even if actual TrustedLen construction is exact. |
| Nat cmp/is_zero | No additional buffer; borrowed limb iteration/scalar tests. |

The map rule is source02's explicit input+output/inherited-capacity alternative;
no same-alignment/reuse assumption is needed. On the bound u64-align8/u32-align4
basis in source05 a tighter non-reuse/TrustedLen route is possible, but is not
needed for this conservative formula. The u64 accumulator is not a u32 buffer
and must remain in the mul construction peak. mul_small/add_small carry fits
u64 because limbs are u32 and multipliers<=10^9; high-level Nat arithmetic does
not introduce another big integer owner.

U remains an explicit public size_of expression, not a private layout guess.
If authenticated U=16 is supplied, the fixed d<=40 parse envelope is at most
max(120,80+160+80,80+160+32+20)=320 bytes; returned Nat<=32 bytes. This is source
arithmetic conditional on that stride, not an observed scratch allowance.
The newly bound immediate i64 FromStr path uses scalar/slice iteration and
inline ParseIntError only (core/num/mod.rs1626-1627,1677-1679,1756-1844;
error.rs74-76). It adds no numeric heap. Fixed-input validation excludes the
parse error-format closure on these strings; altered inputs require revalidation.

## Exact composition, preserving temporary ownership

For fixed exponents, alignment uses p2=min(nonzero operand p2), likewise p10.
For interval states a,b, use the interval union for the result's exponents and
safe nonnegative deltas, e.g.
`delta2_a<=max(0,a.p2_hi-min(a.p2_lo,b.p2_lo))`, and similarly decimal deltas.
Known zeros take the explicit zero branch. This overapproximation includes the
special zero exponent choices without casting a negative difference to u64.

For a lift, let S=shl(x,delta2), Q=mul_pow10(S,delta10):

```
HLift(x) = max(Hshl(x), R(S)+Hpow10(S))
HAlign(a,b) = max(HLift(a), R(Qa)+HLift(b))
```

The shifted S temporary is alive while mul_pow10 clones/grows Q. Qa survives
while b is lifted. At return both Qa,Qb coexist; original a,b remain caller-owned.
Bit bounds for lifts are Bx+delta2+4delta10. Let Aligned(a,b) denote Qa,Qb.

```
HCmp(a,b) = HAlign(a,b)
HAdd(a,b) = max(HAlign(a,b), R(Qa)+R(Qb)+4(max(la,lb)+1))
HSub(a,b) = R(Neg(b))+HAdd(a,Neg(b))
HMul(a,b) = HNatMul(mant(a),mant(b))
HMax(a,b) = max(HCmp(a,b), R(Clone(chosen input)))
```

In HAdd the l values are **aligned** logical lengths. Opposite-sign subtraction
can use its smaller exact reservation; the displayed add reservation safely
covers both signs and its output state uses capacity max(la,lb)+1. Both aligned
buffers remain until return. Sub's negated clone coexists with original b and
the complete add. Max returns a clone of the original chosen input, not a large
aligned operand; cmp's aligned buffers have dropped before that clone.
Mul exponent intervals add; add/sub retain alignment's common exponents.
Every length, exponent operation, byte product and capacity addition must be
checked against i64/u64/usize/Layout limits before using an estimate.

## Live expression maxima in the actual predicates

The following equations are phase maxima, applied componentwise to ordinary and
move H. They exclude externally owned input exp E, scale S, and scalar obs O;
add those once at the immediate caller. Symbols name **distinct allocation
identities**, even when logical values are equal. Padding a temporary until its
statement ends is deliberate; no optimizer/lifetime shortening is required.

Tolerance X466-468: A=abs(E), M=max(A,S), T=scaled(M,0,-9):

```
HTol(E,S) = max(R(A)+HMax(A,S), R(A)+R(M)+R(T))
```

A and receiver M may coexist with new T until the return expression ends.
Scalar predicate X471-473: D=sub(O,E), Dabs=abs(D), T=tolerance(E,S):

```
HPred(O,E,S)=max(HSub(O,E), R(D)+R(Dabs),
                R(D)+R(Dabs)+HTol(E,S),
                R(D)+R(Dabs)+R(T)+HCmp(Dabs,T))
```

This retains the sub receiver D while abs and the comparison/tolerance argument
are evaluated. Tolerance's inner temporaries are not retained after it returns.

Magnitude X479-492: Y,Z=from_f64; Y2=Y*Y,Z2=Z*Z, Q=Y2+Z2; T=Tol(E,S);
U=E+T; L=E-T; U2=U*U,L2=L*L. Both Y and Z remain local to function exit.
HMag is16 plus the maximum of:

```
HMul(Y,Y)
R(Y2)+HMul(Z,Z)
R(Y2)+R(Z2)+HAdd(Y2,Z2)
R(Q)+HTol(E,S)
R(Q)+R(T)+HAdd(E,T)
R(Q)+R(T)+R(U)+max(HMul(U,U),R(U2)+HCmp(Q,U2))
R(Q)+R(T)+R(U)+HSub(E,T)
R(Q)+R(T)+R(U)+R(L)+max(HMul(L,L),R(L2)+HCmp(Q,L2))
```

The initial16 also bounds sequential construction of Y and Z. Y2/Z2 die after
Q's statement; U is a local and persists while L is built and compared. Short-
circuit returns reduce peaks; the maximum conservatively permits every branch.

Range X496-504: A=abs(E), Tiny=pow2(-1075), P=pow2(1024), V=pow2(970),
Huge=sub(P,V). R(Tiny)=R(P)=R(V)=8. Then:

```
HRange(E)=max(R(A), R(A)+8+16+HSub(P,V),
             R(A)+8+R(Huge)+max(HCmp(A,Tiny),HCmp(A,Huge)))
```

P/V die after Huge's construction. For a known zero E, early return adds0.
Huge has mantissa2^54-1,p2=970,p10=0; using55 mantissa bits is also safe.

Floor X508-512: A=abs(E), C=max(A,S), F0=from_f64(s_star), F=scaled(F0,-34,0):

```
HFloor(E,S)=max(R(A)+HMax(A,S), R(C)+8+R(F),
               R(C)+R(F)+HCmp(C,F))
```

A dies after C's statement; F0 dies after F's statement. F is an exact dyadic
with B<=53,p2 in[-1108,937]. No rounded multiplication by2^-34 is substituted.

## Immediate caller integration

For each reference/scale shape, parse E then S:
`HInputs=max(HParse(E),R(E)+HParse(S))` and retained inputs R(E)+R(S).
In lane.rs275-309 take the max of HInputs and that retained pair plus:

- HFloor, then one of the scalar branch `8+HPred(O,E,S)`, HMag or no numeric
  helper for StructuralZero/Unavailable; the8 is compare.rs40's fresh from_f64.
- HRange on the successful predicate branch. holds' numeric temporaries have
  already dropped before judge's range call (compare.rs60-67).

Exact helpers from those sequential calls are not summed. Source11 still pays
Outcome, published/body/floor maps, original strings, resolve/observe arrays and
diagnostics; this proposal adds only the named Exact/Nat buffers above them.
E and S remain alive through resolve/class checks and failure formatting until
the row-loop body ends: add R(E)+R(S) to those later V-FMT/diagnostic phases
even after comparison scratch drops. Their retained bytes do not disappear
merely because the verdict has been computed.
Floor::not_covered191-197 uses HInputs and R(E)+R(S)+HFloor while its previously
collected key Strings/output Vec remain source11 caller owners.

For each value-control triple (lane.rs56-65):

```
HControl=max(HParse(E), R(E)+HParse(S),
             R(E)+R(S)+HParse(O), R(E)+R(S)+R(O)+HPred(O,E,S))
```

Its Exact objects drop at closure return before the control name is formatted.
Keep the SourceParts/control lookup/previous ControlTally children from source11.
Use max across the finite row/control shape list; never multiply one comparison's
scratch by row count. All reference/control signs and target variants are covered
by the branch maxima; shape grouping is not a fixture exclusion.

## Finite arithmetic check and exact-to-text reachability

ARITHMETIC.json checks the coarse all-case interval bounds without evaluating
any numerical predicate. Decimal B<=160; tolerance p10[-2912,264]. Magnitude
Q<=4197 bits, U/L<=12901 bits, U2/L2<=25802 bits with p10[-5824,546]. The
largest padded alignment lift is25802+2148+4*(546+5824)=53430 bits, hence
normalized length<=1670 u32 limbs. All shift append lengths<=875, mul accumulator
lengths<=809, and add/sub reservations<=437 in this expression set; power-of-ten
growth gives the largest generic retained capacity<=3340 u32 limbs (13360 bytes).
These are **per-owner ceilings**, not a whole-expression multiplier or E_max.
Use the explicit live-DAG maxima above to compose them.

Search of frozen VR/src and examples found no call to Exact::approx. Its helper
decimal_digits is called only by that unreachable method. Neither exact-to-text
helper is reached in the selected comparison/control/floor/observation paths;
there is no returned numeric text term here. Error/diagnostic text remains with
V-FMT. The Exact calls in parity.rs290-299 occur after its dense=false early
return246-247 and are outside vk_scale's sparse tail. Generic dense parity can
reuse these primitive rules but requires its own loop/retained-worst ownership;
no dense-parity heap claim is made or existing test domain removed.

## Remaining premises and claim boundary

1. Bind U=size_of::<&[u8]>() and the selected allocation contracts to final
   compiler/library/target. It is a public layout expression, not a request for
   a private overlay or new tool. Other u32/u64 request strides are4/8.
2. Caller-derived observation/floor inputs must be finite whenever from_f64 is
   reached. Kernel Normal/Subnormal/Underflow tags supply finite raw values;
   observe's twist/extension division and floor's coupled/per-member arithmetic
   are not by themselves a proof of finite results (lane162-169; floor117-158).
   A nonfinite input hits X290's assertion before allocating its new Nat; panic
   formatting/unwind then belongs to the existing V-FMT/runtime obligation.
   This packet neither excludes that branch nor asserts the premise from old
   passing runs. Establish it from the actual fixed-model/caller basis, or retain
   a separately bounded error path, before claiming a complete global envelope.
3. Independently check these equations and descriptor extraction, then perform
   the separately authorized checked-estimator translation/final-A1 reconciliation.
   The retained-model/input/parser/serde/format/I/O/sparse owners remain source11's
   separate cells. Full V-EXACT numeric integration is not claimed merely by a
   per-Nat ceiling; source11 needs the composed per-call max and caller union.

No observed scratch amount, safety factor, acceptance reduction, new fixture,
precision change or other-process baseline was used. No Rust/runtime/probe or
maintained code change was authorized or performed. This narrows V-EXACT to
source-replayable recurrences, fixed descriptors and the stated final bindings.
