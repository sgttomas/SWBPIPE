# A1 publication certificate — proposal for independent review

Status: **PROPOSED; no design selection or implementation authorization.**
HELPS_HUMANS `/root/a1_design` reports directly to HELP_HUMAN `/root` through
Codex native delegation. No child delegation. This is bounded numerical design,
not reusable workflow or host-tool work. No Rust, solver, native/product or unit
route was executed. All proposed implementation and acceptance tests below are
unrun. `BASIS.json`, `EXECUTION.md`, `exact_checks.py` and its raw output preserve
the actual reads/checks and write inventory.

Aliases: P = projects/chirality-piping; FK = P/core/solver/frame_kernel;
T3 = this resumed run's parent; R = T3/RESUME_2026-09-30. Source pin is
`d01ad98a754698631f927709d08284c272de85e8`, FK tree
`197bdfdf1af7ca7e05b0192bc3f70f6a6ff86e86`, identical to audit `3bddc2b...`.
New diagnosis/oracle records are frozen at `a6b40d2d036acac556e28f28f5b482a4adb39333`.
The design brief and COMMON are from COORD `dd677a972799237edfd8f389468aaa13c2ecf330`.
Accepted authority is D1 R7 with ROOT's A1/A2 rulings, not this proposal and not
the old response's conditional designs. No old conditional design was loaded.

## 1. Recommendation and decision

Add one **publication certificate gate after all existing R7 gates and before
marking a candidate Accepted**. Keep existing binary64 row values, O9 membership,
body scales, classes, small-scale A1 bounds and p512 floors. For every finite,
non-input-derived row, use its actual final binary64 value `x`, retained
verification value `v`, and the already available certified verification error
`E` to test an exact upper bound

    H = |x − v| + E.

Recommended absolute rule: **H ≤ b**, with exactly the bound bits the present
publisher would emit. Thus every accepted absolute row really lies inside D2's
literal `[x − b, x + b]`. For relative rows, require the existing public and
protected sharper predicates (§4). Failure rejects this candidate with a named
reason, then uses the existing 128→256→512 schedule; failure at the ceiling
returns Unresolved with that attempt evidence. Extreme inputs need not solve.

This removes reliance on the false universal transfer from retained scales to
published scales. It does not patch the error with a physical range cutoff,
extra precision, a relaxed oracle, or an assumption that computed zeros are exact.
The new condition can reject honest candidates: a sufficient certificate need
not be necessary. That availability tradeoff is explicit and must be measured.

**Owner choice O1:** select the recommended bare-b gate, or retain the qualified
R7 radius `b(1+δ_p)` and make D2 use that larger radius everywhere. The latter
must not continue to bind `[x−b,x+b]`. It has greater potential availability but
changes consumer semantics. Neither choice is made here.

**Owner choice O2 (only if O1 availability is unacceptable):** adopt an outward
per-row certified radius or a new outward scale contract (§8), with new receipt
semantics and reader/replay consequences. It is not required to fix C17 honestly.

Registration of the corrected policy/identity, and any acceptance or public
contract change, remains ROOT/owner work. An implementing slice must receive a
new explicit grant after independent design review.

## 2. Established failure and source trace

I22/c_01 and oracle_fresh/addendum_02_C17 independently bind unmutated C17:
binary `bcbe897204ec702b99529d25e6d0213d0132af5e6086e0397fae3a8f8ef8a08f`,
raw TSV `841167ed887a71b0e2a904647e1f217758fad304baaaf0d669f5c47035dc797a`.
At p128/P256, D:0 and M:0 publish +0 with `b=2^-1038`. Separate exact source
forces sum to `9·2^-1074`; the grounding spring is `2^-33`, so whole-body axial
equilibrium gives `u0=9·2^-1041`. Error/b is 9/8, greater than `1+2^-22`.
This is BLOCKING kernel false publication. Literal transport is excluded by the
existing numeric magnitude profile. Equivalent-input/native reachability is
unproved in either direction; this is no product witness.

At the source pin:

| Source | Obligation |
|---|---|
| adaptive.rs:276,296 | L is the prescribed binary64 body extent; four independent max/coupling expressions use the original raw maxima |
| adaptive.rs:379,412 | A1 row-bound branch and classification use published S; S=0 emits b=0 |
| adaptive.rs:1904,2013 | verification maxima exclude input-derived and candidate-unpublishable rows, then couple in retained precision; R7 stop/estimate/bound/theta/g/charge |
| verify.rs:603,1113–1208 | report already contains per-row E_q, W, C and W-plus, including magnitude component errors |
| wide/multi.rs:714 | one nearest-even binary64 rounding; nonzero rounded to zero is Underflow without a value |
| adaptive.rs:3140–3275 | selection/escalation and accounting boundary |
| adaptive.rs:3295 | finalization calls publish_prescribed before classify_rows_floored |

The full R7 mathematical basis consulted is
`DESIGN_NUMERICS/REV_5A3_CANDIDATE/D1_REV_5A3_SSTAR_RESOLUTION_R7.md`, especially
§5.1–5.5 and §6.3; A1 is ROOT_RULINGS_V1:2016–2027, A2:2544 onward.
D2 DESIGN §4.9.3, §4.9.9, §4.11 and D1's closed-kind/recovery rules govern
downstream consequences. Source pinning and byte hashes are in BASIS.json.

## 3. Complete coupling obligation, independently derived

Let J_k contain exactly the non-input-derived rows of kind k whose **candidate**
binary64 outcome has a value, including exact zero and subnormal values. The
same J must define both verification and publication maxima (accepted O9).
Input-derived rows and Underflow/Overflow rows are excluded from both maxima.
Those excluded rows still undergo the standing R7 checks; E's formation maxima
continue to include unpublishable force/moment rows. Do not mix these sets.

For j in J, let c_j be candidate retained value, v_j verification retained value,
x_j its final binary64 publication, d_j=|v_j−c_j|, r_j=|c_j−x_j|. All are exact
values for the algebra. Let A_k=max_J |v_j| and X_k=max_J |x_j| (empty max = 0).
The max function is 1-Lipschitz in the sup norm, hence

    |A_k−X_k| ≤ D_k := max_J |v_j−x_j| ≤ max_J(d_j+r_j).

This does not assume the same row wins each maximum. For nearest binary64
rounding, a uniform conservative bound is `r_j ≤ 2^-53 |c_j| + h/2`,
`h=2^-1074`; use the **actual exact difference** when c_j is available. Relative
rounding alone is invalid for raw subnormals. Retained zero is distinguishable
from nonzero Underflow, and no nonzero Underflow may enter J as published zero.

For a finite positive binary64 L, define exact-real coupling

    K_tr(A)=max(A_tr,L A_ro); K_ro(A)=max(A_ro,A_tr/L)
    K_fo(A)=max(A_fo,A_mo/L); K_mo(A)=max(A_mo,L A_fo).

Let η_v,k be the absolute rounding error of the retained coupled operation,
and η_x,k that of the binary64 coupled operation, before max. Exact raw maxima
need no arithmetic rounding. Then, by triangle inequality and max Lipschitz,

    |S_v,tr−S_x,tr| ≤ max(D_tr,L D_ro)+η_v,tr+η_x,tr
    |S_v,ro−S_x,ro| ≤ max(D_ro,D_tr/L)+η_v,ro+η_x,ro
    |S_v,fo−S_x,fo| ≤ max(D_fo,D_mo/L)+η_v,fo+η_x,fo
    |S_v,mo−S_x,mo| ≤ max(D_mo,L D_fo)+η_v,mo+η_x,mo.

These cover raw subnormal errors, candidate/verification differences, unequal
maximizers, the coupling's own rounding, and both amplification directions.
For L=0 both coupling terms are omitted by contract and the bound is just D_k;
L must never be replaced by a separately rounded reciprocal. Nonfinite L or
nonfinite coupled scales cannot support a finite publication/receipt certificate.
L is an operational scale from the prescribed binary64 extent calculation, not
an assertion that it equals exact physical extent. This derivation needs only
that both paths use the identical L bits. Geometry error in mechanics remains
in R7's existing formation proof; derived-stress use of physical member length
has a separate obligation (§7).

At p512, applying the **same exact decoded Φ bits** to both scales by max is
nonexpansive: `|max(S_v,Φ)−max(S_x,Φ)|≤|S_v−S_x|`. It cannot amplify the gap,
but it need not eliminate it. Φ applies only to force/moment and only at 512.
Ehat remains the existing binary64 coupling of upward uncoupled formation E;
its own-kind term guarantees Ehat_k≥E_q. That fact does not rely on a relative
error bound for coupling, even when its other operand underflows.

Taking A_ro=5h/4, X_ro=h, L=2^100 gives a translation gap hL/4=2^-976,
20% of S_v. The reciprocal T→R and F/M versions have the same loss. For a
short L the coupled published scale can be zero while the retained scale is
positive. Neither an added h/2 **after coupling** nor checking only whether
the final S is subnormal closes these cases. Crossing S=2^-988 also changes
both classification and the A1 b formula discontinuously.

The exact publication obligation is therefore per final row and final class:

    |x−q*| ≤ |x−v| + |v−q*| ≤ H ≤ the actual promised radius.

The proposed gate enforces this directly. It does not need to estimate the
coupling gap or solve a circular inequality for S_v in terms of S_x. The full
coupling derivation above explains why a scale-only shortcut would need every
term and why the direct certificate is smaller in proof and implementation.

## 4. Exact proposed gate

Run existing R7 (a),(b),(c),(d) in their existing order. Only after they pass:

1. Form the same provisional publication that finish_selected would produce:
   candidate nearest-even outcomes, exact-prescription replacement, identical
   O9 membership, four body scales, same Φ bits, class and b_row. Canonicalize
   zeros only as the current public contract requires; never turn Underflow
   into a value. Bind this publication to the candidate/report/layout pair.
2. Require exact layout lengths, identity, body/kind and expected report fields.
   Missing E_q/W/C for a force/moment or W-plus for a non-input-derived
   displacement/magnitude is a named internal certificate failure, not zero.
3. For each finite non-input-derived kernel row, set E as follows, with P=2p:

| Kind | Certified E using fields already retained in VerificationReport |
|---|---|
| free translation or rotation | W-plus_j |
| translation magnitude | W-plus_j + 2^(1−P)|v_j| (component prescription errors are already in W-plus) |
| force or moment, including end/station/support/spring/reaction rows | 69·2^-P E_q + W_j + 2^-P W_j + C_j |

   Use per-row `report.e_rows[j]`, already present; no new solve, factor, norm,
   inverse estimate or heuristic. The body Ehat can replace E_q conservatively
   only if desired for code reuse and its availability cost is measured. The
   proposal chooses the tighter existing E_q. Do not drop `W·2^-P` or charge.
4. Form H=|x−v|+E as one exact nonnegative expansion. Values x and v are lifted
   exactly; there is no binary64 subtraction, no nearest-rounded H and no
   conversion through f64 for the comparison. The proposed terms are integer
   multiples and powers-of-two scalings of existing dyadics. ExactWideSum can
   represent them until its existing span/exponent/work limit; that limit causes
   a recorded refusal, never a truncated certificate. No widening of those
   limits is proposed.
5. For AbsoluteVerified, compare H≤b (recommended), exact decoded b including
   A1's row term where 0<S<2^-988. At S=0, b=0, so every nonnegative term of H
   must be exactly zero. No observed-zero exemption is allowed.
6. For RelativeVerified, compare H≤10^-9|x|, using exact decimal 1/10^9
   (integer cross multiplication). Preserve the protected sharper check too:

       H ≤ 2^-64 max(|x|,S_x)(1+2^-21) + 2^-53|x| + h.

   The independent oracle reports the mathematically exact predicate and the
   source comparator's operation-by-operation binary64 allowance separately.
   Do not relax either protected test: where those differ, the gate may use the
   smaller exact-decoded allowance. This may conservatively reject a boundary
   candidate. The public relative denominator is |x|, as D1 states; the truth
   denominator is separately reported and cannot silently replace it.
7. InputDerived rows preserve exact-source one-round publication and the
   input-derived contract, not a solve certificate. Unpublishable rows retain
   their range outcome and no numerical class; the gate invents no truth-range
   proof from absent binary64 data. Existing independent range tests remain.
8. First failure by stable layout order: `publication_enclosure` with row id,
   body, kind and selected target predicate. Escalate as a numerical rejection;
   at 512 the case is Unresolved(Ceiling) with the failed publication reason in
   attempts. An exact-arithmetic Span/Exponent/Budget stop retains the existing
   terminal/stop semantics and accounting. Invalid/missing certificate data or
   a nonfinite encoding is separately named `publication_certificate` or the
   standing `receipt_encoding` refusal, not a successful numeric comparison.
9. Only then mark Accepted/Verified and emit exactly that provisional publication
   or recompute it bit-identically with a checked assertion. Nothing downstream
   may replace values, classes, scales, floor, bound or row order without a
   new certificate. Replay recomputes this gate.

No new public summary is necessary for the smallest kernel change: like R7's
E and charge, the certificate relies on producer execution and replay, not on a
claim that readers reconstruct unpublished verification values. Attempt-reason
serialization and registered policy semantics do change. The eventual successor
reader must recognize only the corrected policy as supplying this guarantee;
old policy/evidence must not be retroactively relabelled certified. ROOT should
reserve the exact policy/version token with D2 rather than this proposal
inventing a registration. Row/scale/bound schema can stay unchanged.

## 5. Proof through the full stop/charge chain

The gate relies on the **verification error theorem**, not R7's disputed final
scale-transfer corollary. The precise inherited premises are:

- R7 Lemma B bounds stiffness/recovery formation with its gamma factors and
  tested g scope. Exact source load ledgers and exact prescribed values in r
  remain essential. The external OCR equation corpus supplies no premise.
- For each data block, A2 takes min over completed certified Uc/S values only.
  Lemmas D/E supply B≥inverse norm; missing/refused values never become partial
  bounds. Existing theta≤1/2 gives Lemma C's factor-two Neumann bounds for K*
  and K^c. No Hager–Higham estimate is a correctness bound.
- Contribution residual r and correction residual r2 are exact expansions at
  q_W=min(3p+64,1024), using the final selected verification state. Terms t1
  (formation), t2 (correction recovery), t3 (remaining correction residual)
  give C=||a_bar S||_1(t1+t2+t3). t1 need not vanish when delta and r2 vanish.
- The R7 theorem then gives the E in §4. The magnitude term is explicitly kept.
  All no-data-block conclusions retain R7's nonsingularity/exact-zero-data
  premises; this design does not derive nonsingularity from a computed zero.

For comparison with the old argument, let ε=2^-64 and M=max(|v|,S_v). Existing
rule (a) gives |c−v|+V≤εM. At p128/256 for force/moment, V=256·2^-P Ehat,
W≤64·2^-P Ehat, C≤60·2^-P Ehat, and E_q≤Ehat. Thus

    |c−q*| ≤ εM − [256−69−64(1+2^-P)−60]·2^-P Ehat
            < εM − 62·2^-P Ehat.

At p512, C≤2^-86 M gives the old candidate bound εM(1+2^-22).
Translation/rotation use W-plus in (a), yielding εM, with the stated magnitude
rounding term. These retained-value bounds remain useful, but **none** implies
the final published interval without the missing transfer. The new proof is
instead the triangle inequality `|x−q*|≤|x−v|+E`, followed by the exact gate.
It is valid irrespective of which max wins, subnormal operands, class changes,
or floor dominance. Every retained R7 prerequisite remains required even where
a small H happens to be computed.

Independent-review boundary: this assignment rederived the coupling lemma,
certificate and stop/charge algebra and inspected their source fields. It has
not independently re-audited every floating-point implementation in R7's
formation/factor lemmas. Those are explicit accepted upstream premises, not a
new universal verification claim. A failure of those premises found by review
or mutants would block this proposal too.

## 6. Availability, C17, zeros and floors

C17's p128 publication cannot pass either bare-b or qualified certification if
E is valid: triangle inequality forces H≥9b/8> b(1+2^-22). This is a derived
consequence, not an executed gate. Public TSV does not expose v/E, so no exact
new H, later selected precision, or repaired value is asserted. The p256/512
result may select honestly or refuse; both meet the bounded owner objective.

At b=0 the recommended rule passes iff x=v and E=0. A force/moment row with
E_q>0 has E>0 through the 69·2^-P term at every finite P and therefore refuses
unless a later state has a genuinely zero formation scale. A free zero row may
have W-plus>0 solely through body t1/t3 and also refuse. An all-zero-data body
whose formation, prescriptions and verified rows are exactly zero has H=0 and
can still pass. A row that is zero by symmetry in a loaded body has no exemption:
if its generic certificate is positive while b=0, reject. Existing S-J topology
proofs are not imported into kernel selection by this amendment.

For p512 force/moment with Ehat>0, Φ=RU(2^-438 Ehat)>0 and b≥RU(2^-64 Φ)>0,
at least h. Thus a floor prevents their zero-scale bound, but the old permitted
charge alone can make H exceed **bare** b. The stricter gate may refuse such
ceiling cases. There is no further precision and no proposal to increase it.
If Ehat=0, Φ=0, and positive C/W still cannot pass a zero b. T/R have no floor.
Where Φ and εΦ are normal, b=2^-502 Ehat while the 69·2^-1024 E_q term is at most
69·2^-522 of b; that small formation term does not by itself consume the floor.
Disagreement and charge near the boundary can consume it. Subnormal Φ/b are
handled by the exact bit rule, not that normal-number ratio.

Observed controls: I22 B01–B16 are all honest selections (six p256, ten p128),
592 rows: 176 InputDerived, 413 AbsoluteVerified, three Unpublishable, zero
RelativeVerified. They cover both T/R amplification directions, below/at A1
threshold, raw exact-subnormal/normal boundary and O9 underflow. B10's zero
rotation scale has only input-derived rotation rows. C17 adds four honest
relative rows but is a failing case overall. C18–C24 remain stopped/unrun.

Expected ordinary effect: values, scale bits, class and b bits are unchanged
whenever the new gate passes at the old precision. The E-based gate normally
has much more slack than using εM as its error estimate, but there is **no
measured availability-preservation result** yet, including for the B controls.
Some honest zero rows may newly refuse. Preserve and report that outcome; do
not change the oracle to keep a control selected. An unacceptable ordinary
availability loss goes to the owner with the actual row/certificate terms.
If a new outcome conflicts with a protected expected selection/standing, stop
that acceptance path and return the measured conflict; a mathematically safe
refusal does not by itself authorize changing a protected availability criterion.

A published row x=0 does not imply b=0: another same-body row or a coupled kind
may give it a positive scale and ample allowance. Many positive-E/zero-scale
force or moment states already fail R7 rule (a) before the new gate; count only
additional failures after the standing gates when reporting availability loss.
The new zero-bound concern is especially where binary64 coupling collapses a
positive retained scale, or where subsequent product conversion loses range.

Work: one extra O(Q+B) provisional-publication/certificate pass, no factorization.
Each force/moment H adds |x−v|, 69·2^-P E_q, W, 2^-P W and C; each T/R H has
fewer terms. Exact span and budget can still stop it. A rejected candidate can
cause an additional scheduled solve/verification up to the unchanged ceiling;
the total cost is not merely the local pass. No timing/RSS prediction is made.

## 7. Small coherent F2a/D2 contract and remaining transfer obligations

**Kernel certificate is not by itself a product row certificate.** F2a may
publish a covered row only after all transformations between kernel and that
actual row preserve a certified radius and its final class/bound passes the
same promised-radius check. This is a narrow finalization obligation, not a
new general solver or range-profile undertaking.

- For unchanged SI kernel rows, the kernel's strict b certificate is sufficient.
  D2 keeps bit-for-bit G5b/G5c, including adopted A1 b_row and p512 Φ. Its older
  DESIGN text requiring universal RU(εS) must receive a cited additive update;
  do not edit historical pinned acceptance bytes.
- R7 G5a item 4 claims converted S_pub is relatively close to S_v. §3 refutes
  that as a universal premise too. A new absolute gate does **not** prove that
  every honest E passes G5a. Keep the existing G5a shape, zero, sanity, lower
  and summary predicates as conservative admission checks. The producer must
  execute those **same pinned operations on actual converted published data**
  before emission, refusing `receipt_encoding` with a precise predicate reason
  on failure. Readers run the same check and code order. A passing G5a remains
  a consistency test of producer-attested E, not an independent proof of E.
  G5a can reject an honest case; it cannot authorize a failed certificate.
- Unit normalization for reader scale reconstruction remains the pinned
  binary64 order and factors. That operation defines the receipt checks but
  does not make source-unit or display conversion mathematically exact.
  For an intended linear unit map y*=a q* and actual output y, a valid radius is

      H_y = |y−a x| + |a| H_x,

  with exact a according to the accepted unit definition and actual x/y bits.
  If implementation uses a rounded factor a_hat, the first term includes
  |(a_hat−a)x| and operation rounding; it cannot be omitted. For affine units,
  use |y−(a x+d)|+|a|H_x. W1's scaled mechanics kinds are linear. Repeated
  conversions need the full composition or each outward error step. A converted
  nonzero value rounded to zero remains subject to a positive radius or the
  existing display-range refusal, never an exact zero inference.
- The final product x/y, S, class and b must use the **declared published unit**
  consistently. After the pinned reader normalization, re-certify the actual
  covered product row against its actual unit bound. Changing units is not
  assumed to preserve selection, b bits or class. Source-unit re-entry may
  round primitive numbers into a different intended source; use independent
  exact expectations from those new bits, not a reused SI truth.
- Derived stress/extension/twist recovery needs radius propagation from the
  kernel certificate, not substitution of εS_pub for the actual row errors.
  Conservative rules are `H_(a q)=|a|H_q+rounding`,
  `H_(q1±q2)=H1+H2+rounding`, `H_hypot≤H1+H2+rounding`, and for span statics
  `H_(M−lF)≤H_M+|l|H_F+rounding` when l is exact in the intended formula;
  uncertainty/rounding in l, A, Z, stiffness and user factors must be included
  where the source contract does not treat their decoded bits as exact.
  Actual interval propagation over the already bounded formula is an equivalent
  safe implementation. Then compare the final row radius to its existing b or
  relative claim, and refuse the unsupported publication if certification fails.
  Nonlinear maximum/hypot operations use enclosure/Lipschitz properties; signed
  cancellation must not be hidden by the phrase "well-conditioned". R7's prior
  L_member≤L_body use also needs rounding-aware treatment; using actual |l|
  in propagated error avoids relying on that equality of rounded scales.
- InputDerived rows follow their source contract. Unlisted/pressure-excluded/
  unsupported derived rows remain NotCovered. A failed intended covered-row
  certificate is a named case/facade refusal, not silently changed to NotCovered.
  No ordinary-route failure or product domain restriction is inferred.
- Under bare-b choice, D2 endpoints stay outward `[x−b,x+b]`, including point
  x when b=0 (now justified by H=0, not mere computed agreement). Subsequent
  rule-unit conversions and formula evaluation remain outward. Under qualified
  choice, first form `r=RU(b(1+δ_p))` exactly; endpoints and all notices use r,
  including overflow refusal. One next_up around x±b is not guaranteed to cover
  the missing δ_p b. This is an explicit consumer-contract amendment.

Unclosed before F2a reliance: execution of this gate; independent audit of its
available E fields; actual G5a preflight/reader parity including unit conversion;
finite radius propagation for each covered derived formula; source-unit and
native routes. No blanket guarantee for these is inferred from a kernel fix.
Smallest coherent facade design is to require certification at final publication
and refuse where no verified transfer exists. Exact API/lifetime for passing H
to F2a belongs in its bounded implementation design; reuse a private on-demand
row iterator or retained report rather than expose new public radius semantics
unless owner selects O2. No broad facade implementation is authorized here.

## 8. Outward alternative and guard-only alternatives

**Outward bound alternative:** retain old scales/classes, compute
`b_new=max(b_old,RU(H))` for each absolute row. If H cannot be represented by a
finite binary64 radius, refuse. This mathematically encloses the intended row
without the old scale-transfer claim. It may save candidates which the bare-b
gate rejects. But b_new is no longer re-derivable from published rows and S
alone: H depends on unpublished verification, so D2 needs a registered
producer-attested radius contract, hash binding, replay and revised G5c equality.
Relative rows still need certification; widening an absolute bound cannot
repair them. More width can turn rule passes/fails into indeterminate results;
that is visible behavior needing owner selection.

**Outward scale alternative:** an upward encoding of retained S_v or exact
coupling does not alone solve the problem. It must also cover direct row
rounding and the p512 charge allowance; b≥εS_v is not generally H≤b.
RU of binary64-rounded raw operands still misses their original lost h/4.
Taking an upper scale from v plus every coupling-error term can be valid only
with its actual final b/class certificate. Such a scale also breaks D2's
current recomputation from published rows and can demote relative rows. It
needs additional receipt data or a new scale definition. It is larger in scope
than the direct certificate and has no demonstrated availability advantage here.

**Subnormal-input ban or final-scale guard:** neither is selected. Raw
subnormal *results* can arise from normal operands; the final coupled scale can
be normal or zero. A finite physical cutoff requires owner-supported domain
evidence and is unnecessary for honest refusal. **Always increase p:** cannot
fix binary64 output rounding or guarantee a p512 claim. These are not adequate
substitutes for the certificate.

## 9. Finite acceptance plan and mutants (all solver work unrun)

Use the frozen exact oracle without changed tolerances, then separately test
the strengthened bare-b predicate. Record selection, refused/unresolved,
precision, all row bits, scales/classes/bounds, E terms, H, reason, work and
memory. No expectation may be manufactured from the solver output.

| Test group | Required discrimination |
|---|---|
| C17 replay | old p128 false publication never accepted; later honest selection or named refusal; unchanged frozen truth |
| B01–B16 replay | quantify actual changes, including honest refusals, both T/R directions, threshold and O9; no claim B10 tests free zero-rotation rows |
| finite remaining C cases | only if separately granted; frozen C18–C24 are expectations, not executed coverage |
| four-direction synthetic gate tests | independently chosen dyadic c/v/E/x with raw 5h/4 and L=2^±100 in T→R, R→T, F→M, M→F; tests for the certificate algebra, explicitly not source witnesses |
| actual source F/M controls | two bounded primitive fixtures isolating subnormal force and moment maxima in opposite directions; independent source equations first; required remaining coverage, B does not supply it |
| zeros | all-zero-data body passes; loaded-body exact-zero row with positive generic E refuses if b=0; free rotation zero-scale fixture; no-data premises preserved |
| thresholds/maxima | S immediately below/at/above 2^-988; classification threshold tie and adjacent values; distinct max winners; input-derived dominant row; O9 excluded extreme |
| p512 | force/moment floor dominant/non-dominant/zero/subnormal; T/R not floored; ceiling refusal with nonzero certificate; existing selected-p512 controls for availability |
| full rows | end/station, directional/global spring, support group, reaction, free displacement, prescription-containing magnitude; missing/duplicate/mismatched layout/certificate fields reject |
| ordinary regression | accepted K4/VR correctness fixtures, modest RF-LARGE and representative mixed bodies; quantify newly unavailable ordinary rows before owner choice; no new large experiment implied |
| units/reader | source-unit re-entry from exact decoded bits and non-power-of-two conversions; G5a zero/sanity/lower bound; D2 Rust/Python/TS scale/class/bound parity; interval endpoint and rule-unit round trips; declared covered stress variants |
| accounting/refusal | short budget/span/exponent stops, stable first failing row, verification reuse, combinations and replay; no accepted outcome remains after gate failure |

Required mutants: remove new gate (C17 must kill); use c instead of x (direct
rounding fixture kills); use εS_x instead of E (verification-tail fixture);
omit W-plus/magnitude term; omit recovery 69·2^-P E_q; omit W·2^-P; omit C;
skip positive certificate at b=0; use rounded-down H or rounded-up allowance;
misapply O9; include input-derived in maxima; use updated rather than original
coupling operands; apply Φ at every p or to T/R; use qualified radius in a
bare-b gate; alter published bits after gate; ignore conversion error; retain
G5a closeness as an automatic-pass shortcut. Every guard mutant must have a
concrete finite discriminator or be explicitly left an unclosed test obligation,
not declared killed by a general assertion. Existing R7/A1/A2 protected mutants
remain required; the new gate must not be used to retire their proof obligations.

Some E-term omission mutants are easiest to isolate at the pure certificate
function using exact supplied upper bounds; that validates wiring, not source
reachability. The implemented source controls still need independent exact truth
to catch an understated E. C17 is the one realized false publication available.

## 10. Proposed write set and K6c consequences

Small kernel implementation scope after selection: FK retained/adaptive.rs
(provisional publication, gate, reason/schedule/accounting), existing relevant
retained tests and mutation controls; verify.rs only if a small borrowing
accessor is needed (all mathematical fields already exist); closed retained-api
reexports/serialization consumers and replay tests where new reasons/policy
require them. No change to stiffness, solver/factor loops, bounds, protected
oracles, tolerance, precision ceiling, source admission or host tooling.

ROOT-owned additive D1/ruling/design reconciliation and F2a/D2 acceptance text
must state the selected interval semantics. Later bounded facade scope includes
final-value certification, existing reader checks/parity and typed reason mapping;
the derived formulas/units are the transfer points to verify, not an open-ended
new numerical engine. Old hash-bound records stay historical.

K6c must account for the actual implementation before final E_max: provisional
Binary64Outcome/PublishedRow vectors if newly live; body maxima/scales and floor
storage; ExactWideSum temporaries and their capacity/span; H comparison scratch;
new enum/receipt types; any retained private H vector/report/iterator needed by
the facade; overlap with candidate, verification report, trackers, finalization
and cached states. Prefer reuse/streaming and release provisional storage on
rejection; do not claim O(1) extra memory while allocating a full Publication.
New exact-sum work belongs to the candidate stop/publication stage and both
case/invocation totals, including stopped paths. Gate-driven escalation changes
reachable overlap/work even without a new allocation type. Shared H/VR estimates,
admission replay and post-KF3 W1-T4 must use the final settled source.

## 11. Return boundary

Recommendation is mathematically sufficient for kernel publication under the
named R7 upstream premises. It is not an implemented or independently accepted
repair. The main owner decision is **bare b with conservative refusal** versus
**qualified radius with explicit D2 widening**. Outward b/scale changes are
material alternatives only if measured availability warrants their wider scope.
Fresh independent design review must check this certificate, all row-field
coverage, zero/floor consequences and consumer contract before implementation.
ROOT may then commission one bounded source slice. No public release, F2a
reliance, native/product witness or extreme-model availability is established.
