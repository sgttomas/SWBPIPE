# RV28 — independent A1 design review

**VERIFIED for the proposed minimum SI kernel publication-certificate design, under the accepted R7 verification-error premises.** No blocking mathematical finding in that bounded slice. There is one SHOULD-FIX in the proposed facade/unit contract, below. The complete F2a/D2 executable publication path is **not verified** by this return. Review is not policy selection, implementation authorization, engineering acceptance, or a product/native witness.

The reviewed proposal remains unaccepted until ROOT disposes this review. It is
`R/design_a1/DESIGN_PROPOSAL.md`, SHA256
`926dea73178b0ecde07203fdf7fc85e5ce752b1a5f6ead7cf31a30673249b178`;
its eight-entry seal `080de38cd614012a7eba50b28b12e52ac122fb8246747e77f88fa9a82ab6c34c`
was independently verified. Review origin: TASK `/root/rv28_a1_design`,
parent `/root`, native `collaboration.spawn_agent`; no descendants.
The selected skill is Root `.agents/skills/software-code-review/SKILL.md`.
This is a read-only design/source review, not a mergeable product diff.

Aliases: P = projects/chirality-piping; FK = P/core/solver/frame_kernel;
T3 = this packet's parent run; R = T3/RESUME_2026-09-30. FK source is
`d01ad98a754698631f927709d08284c272de85e8`, tree
`197bdfdf1af7ca7e05b0192bc3f70f6a6ff86e86`, independently equal to audit
`3bddc2b05f6106e969c7cf43373b230845c7cc66` and records revision
`a6b40d2d036acac556e28f28f5b482a4adb39333`.
The review brief, COMMON and owner plan are pinned to COORD
`30fd4848b4f61c4fd9405e382b82462048fff8a3`.
Actual local source bytes matched those pins. Origins/hashes are in BASIS.json.

## Findings and dispositions

### RV28-1 — SHOULD-FIX before F2a reliance: pin the unit coordinate of every predicate and bound

Location: proposal §7, particularly “The final product x/y, S, class and b must use
the declared published unit consistently” and “After the pinned reader
normalization, re-certify ... against its actual unit bound.”

Accepted D1 DESIGN §4.1.6.1 item 3 (line 497) normalizes published rows to SI
with the pinned binary64 operations **before** maxima, coupling and classification.
D2 DESIGN §4.9.3 G5b/G5c (lines 548–593) mirrors those operations.
Consequently a displayed/published mm row, its normalized m value, the receipt's
scale, and the value to which D2 adds/subtracts its bound cannot simply share
one unnamed “actual unit bound.” The present wording is ambiguous between
normalization and a change to the scale/class contract. A literal implementation
that compares a raw mm value to an m threshold, or adds an m bound to a mm value,
has wrong class or interval semantics.

**Remedy:** the bounded F2a design must name raw published value y in unit U,
the exact unit map a_U to SI, the pinned reader value n = fl(a_U y), intended
truth in both coordinates, and the unit of each encoded b. Preserve the accepted
SI G5b/c operations unless a contract amendment is selected. Establish the
certificate in the actual predicate coordinate, including |n-a_U y|, and state
how the certified radius reaches D2's actual binding operand and unit. Use
non-power-of-two factors, source-unit re-entry and m/mm plus N/kN round trips
to distinguish the paths. Exact classification parity does not itself certify
the conversion error.

This does **not** invalidate or block the proposed unchanged-SI kernel gate.
It is the concrete missing facade specification already within the proposal's
stated before-F2a gap. It is routine specification/implementation work if it
preserves the adopted normalization and bound contract; changing those contracts
requires the owning decision. An additive clarification should be backchecked
by RV28; do not rewrite the sealed proposal.

### Notes that constrain implementation, without creating an additional owner checkpoint

- Proposal §1 labels bare b versus qualified b as “Owner choice O1.” The owner
  already authorized the smallest warranted correction with conservative named
  refusal (PLAN, Accepted direction). A bare-b certificate strengthens admission
  while preserving the displayed b and D2's intended interval. ROOT can dispose
  this reviewed recommendation within that direction, subject to unchanged
  protected availability criteria. No separate human permission is needed
  merely because the gate may refuse an uncertified extreme case.
- Choosing qualified intervals, outward per-row radii, altered scales/classes,
  an input cutoff, or reversing protected expected availability changes the
  published contract/domain or protected criterion. Those are actual decisions.
  If qualified intervals are selected, explicitly pin delta_p to the adopted
  A1 factors (2^-22 at 128/256 and 2^-21 at 512); do not leave delta_p symbolic.
  I select neither route for ROOT.
- Proposal §4 step 6 says not to relax either protected relative predicate.
  Make this executable by requiring both the exact algebraic allowance and
  the exact decoding of the current comparator's binary64 allowance, i.e.
  H <= min(A_exact, A_f64), in addition to 10^9 H <= |x|. “May use” the smaller
  allowance must not permit choosing the larger one. FK tests/retained_k4/models.rs
  lines 494–522 pin the existing operation order. This is clarification of the
  stated preservation requirement, not a new tolerance.
- No actual new ordinary-route availability loss has been measured by this
  reviewer or the proposal. Protected outcome conflicts require disposition
  when observed, not an invented blanket gate now.

## Independent derivation and adversarial checks

Forty-six exact standard-library Fraction checks passed. The script was written
fresh for this review, without reading or running the designer's exact_checks.py,
the old response's designs, or a solver as an oracle. It includes synthetic
certificate discriminators; these are not claims that production mutants were
executed or killed. EXACT_CHECKS.json contains all checks and exact quantities.

Let J_k be the candidate-publishable, non-input-derived set. For every row,
set d_j=|v_j-c_j| and r_j=|c_j-x_j|. Then

    |max_J |v| - max_J |x|| <= max_J |v-x| <= max_J(d+r) = D_k.

This holds with different maximizing rows. Candidate rounding is bounded by
2^-53|c|+h/2, h=2^-1074; replacing it with a purely relative term fails for
subnormals. A nonzero candidate that rounds to zero is Underflow, not a zero
member of J. FK wide/multi.rs:714–754 implements that distinction.
O9 must be applied to both maxima using the same candidate outcome; verification
overflow/underflow is not a replacement selection rule.

For finite positive L and exactly decoded operands, max is 1-Lipschitz:

    gap_tr <= max(D_tr,L D_ro) + eta_v,tr + eta_x,tr
    gap_ro <= max(D_ro,D_tr/L) + eta_v,ro + eta_x,ro
    gap_fo <= max(D_fo,D_mo/L) + eta_v,fo + eta_x,fo
    gap_mo <= max(D_mo,L D_fo) + eta_v,mo + eta_x,mo.

Here eta is the **absolute** error in the corresponding multiply/divide before
max. Both computations use the original four uncoupled maxima; sequentially
feeding an updated maximum into another expression changes the contract.
For L=0 the terms are omitted, giving gap <= D_k.
The operational L is the prescribed binary64 extent, not exact physical length.
Nonfinite extents/scales/allowances require refusal before a certificate; a
comparison with infinity cannot certify a finite published claim.

Checks cover all four amplification directions using c=5h/4, v=c+h/16,
x=h and L=2^100 or 2^-100. The actual amplified discrepancy is larger than h/2
added after coupling. Four opposite-length cases have a positive retained
coupling but binary64 coupling zero. Distinct-max-winner and O9 cases pass the
correct inequality. Applying identical exact-decoded Phi by max is nonexpansive;
it does not generally erase a gap. Phi is force/moment-only at 512.

The threshold branch is genuinely discontinuous: for x=0 and
S immediately below 2^-988, A1 b is 2^-1052+h, while at S=2^-988 it is
2^-1052. Classification ties use the actual rounded threshold.
Therefore the gate must consume the final class/b bits and must not reuse a
bound from a pre-floor, pre-normalization, or differently classified row.

Under the accepted verification theorem |v-q*| <= E, triangle inequality gives

    |x-q*| <= |x-v| + E = H.

That result uses no scale-transfer estimate. At b=0 the nonnegative certificate
passes exactly when x=v and E=0. Agreement of two zeros alone is insufficient.
A synthetic honest G5a lower-bound failure is also exhibited: v=3h/4,
x=h, k=2^100 and Ehat=RU(kv). The direct row certificate fits A1 b, but pinned
rounding makes the reader's lower bound approximately kh, larger than
fl(Ehat(1+2^-40)). This is not a source witness; it independently shows why
G5a preflight must remain a real check, not an “honest E always passes” shortcut.

From C17's supplied immutable primitive bits, summing the opposed axial forces
leaves 9h, and the spring stiffness 2^-33 yields u0=9*2^-1041.
With observed x=0 and b=2^-1038, error/b=9/8 > 1+2^-22.
Any valid E makes H at least that error. Thus this p128 publication necessarily
fails both proposed bare and qualified certificate choices. This conclusion
does not infer v, E, a later selection, or product/native reachability.

## E is a certified error only after the full formula

All quantities below are from the **verification** state, with
P=report.precision=2p. Do not double report.precision again.
R7 §5.5 Theorem (lines 563–574) is the upstream premise, not its disproved
publication corollary (lines 578–583).

| Closed row family | Error used by the gate | Source trace |
|---|---|---|
| Free translation/rotation components | W_plus | verify.rs:1153–1159; R7 item 12 and theorem |
| Displacement magnitude, including prescribed components | W_plus + 2^(1-P)|v| | verify.rs:1161–1181; R7:462–465 |
| Member end actions and station actions, force and moment | 69*2^-P E_q + W + 2^-P W + C | verify.rs:105–199, 762–775, 873–877, 1145–1151 |
| Global and directional spring actions | same full force/moment expression | verify.rs:201–224, 873–877, 1145–1151 |
| Reactions | same; E_q includes exact net ledger magnitude | verify.rs:226–242 |
| Support force/moment magnitudes | same; includes every restrained, global-spring and directional contributor | verify.rs:245–273; recover.rs:184–196 |
| Input-derived constrained/prescribed components | exact-source one-round contract, no solve certificate | adaptive.rs:946–959; recover.rs:104–118 |
| Candidate-unpublishable rows | retain range outcome, no finite solve claim from H | adaptive.rs:2051–2059, 2604–2636; wide/multi.rs:714–754 |

The full QuantityId set is recover.rs:53–75; there is no additional unaccounted
kernel row family. Displacement magnitudes are not input-derived merely because
some or all components are prescribed. Their W_plus sums free-component bounds
and the exact discrepancy between retained and exact prescribed components.
Support magnitudes use the accepted R7 force/moment theorem, including its
formation/recovery rounding counts; they must not be treated as one signed
component with a cancellation exemption.

report.e_rows is the **formation scale** E_q, formed with stage-nearest
rounding (verify.rs:760–775). It is not E and need not itself be an error upper
bound. The constant 69 theorem accounts for that formation. W is recovered from
delta with no load ledger (849–877); C is the upward sum of a_s*t1, a_s*t2,
a_s*t3 (1145–1151). W_plus is separately upward formed. Replacing the whole
formula by e_rows, Ehat, W, or epsilon*S_x is invalid.

The residual uses contribution-level K^c and exact prescription terms
(verify.rs:795–844), and r2 retains all corresponding products (881–914).
The proof decomposes verification error into recovery formation, delta recovery,
remaining residual and operator formation. These give respectively
69*2^-P E_q, W(1+2^-P), a_s*t3, and a_s*t1, with a_s*t2 bounding correction
recovery. t1 may be positive even when delta and r2 vanish.
The accepted A2 min is over completed certified bounds only; an unavailable
bound cannot become zero or a partial value. Existing rule ordering checks
uc/theta/g/charge before H. Data flags use nonzero source terms, retained state,
or exact nonzero coupled prescriptions (bound.rs:127–156).
No-data blocks retain the accepted nonsingularity premise explicitly preserved
by ROOT_RULINGS_V1:2034. This review does not claim to prove nonsingularity from
zero output.

The old stop/charge algebra is also correct as retained-state algebra:
256-69-64(1+2^-P)-60 = 63-64*2^-P > 62 at the relevant P.
At p512 the charge allowance adds the 2^-22 relative term. Neither statement
transfers to publication without the new H test. Direct omission discriminators
for formation, W, W*2^-P, C, W_plus and magnitude rounding are in the exact packet.

## Executable kernel completeness and remaining work

The minimum SI kernel gate is specified sufficiently to implement and review
without selecting a new numerical method. ExactWideSum supports exact
add_binary64/add_wide, magnitude, u64 scaling and powers of two
(wide_sum.rs:330–369, 422–479); factor 69 and decimal cross multiplication fit
those interfaces. Its existing span is 8,128 bits (lines 43–47).
A refusal caused by widely separated terms, even where cancellation could have
produced a small H, is conservative. No silent truncation, f64 subtraction,
rounded H, positive-term dropping, or enlarged allowance is permitted.
The gate must validate nonnegative E terms and expected field/layout identities
before comparison. Missing values are not mathematical zeros.

Source lifetimes permit the gate immediately after compare_states and before
Accepted/Verified are assigned (adaptive.rs:3172–3235).
The candidate, verification and matching VerificationReport are all live.
The type match at 2386–2411 binds the precision pair; layout/order/identity
validation remains required. Finalization currently performs exact prescription
replacement and classification later (3307–3315). The new gate must receive that
same provisional publication, then finish_selected must emit those bytes or
check bit-identical recomputation. Range-only rows are not converted to zero.
Failure must reach the existing rejected-candidate escalation path; at 512 it
must reach Ceiling with the failed predicate in attempts. Span/exponent/budget
retain existing terminal precedence. All work, including rejected and stopped
paths, must be charged to candidate/case/invocation totals before return.

Verification reports are **not** retained in RetainedSolve: adaptive.rs:2701–2710
stores states, cache, public evidence and publication; finish_selected:3403–3411
drops the local report after summary extraction. The proposal's suggested later
borrowing iterator cannot borrow a report that no longer exists.
Before F2a relies on private H/E, choose a concrete lifetime: retain sufficient
certificate data/report, certify final rows while it is live, or explicitly
recompute the report with its work/cache effects. This is a required
implementation design, not evidence that the existing API already supplies H.

K6c must re-account the selected approach: provisional values and PublishedRow
vectors, scales/floors, simultaneous live ExactWideSum storage/scratch, comparison
temporaries, failure enums and private certificate storage/report; candidate,
verification, states and cache overlap; release on refusal; and escalation
work/reuse. No timing, E_max, O(1)-extra-memory or admission guarantee is made.
The settled source must feed shared H/VR estimates, admission replay and W1-T4.

Before F2a reliance, also finish RV28-1, corrected-policy registration and reader
recognition, attempt-reason serialization, G5a actual converted-data preflight
with reader operation parity, exact G5b/c A1/floor re-derivation in all three
languages, and finite propagation through every covered derived formula.
The proposal correctly rejects old epsilon*S_pub shortcuts in stress recovery
and the unqualified physical-length/body-extent comparison. Rebuilt span
moments require H_M+|l|H_F plus actual arithmetic/operand uncertainty; division
by section values requires their accepted exact-bit semantics or interval
uncertainty. Unit changes need |y-a*x|+|a|H_x, with exact a and every rounding.
D2's outward endpoints do not turn a qualified b(1+delta) claim into bare b:
at x=0, b=1, one next_up gives 1+2^-52, below 1+2^-22.

The finite implementation tests/mutants in proposal §9 are appropriate but
unrun. The exact discriminators establish how to distinguish algebraic errors;
source fixtures are still needed for F/M coupling, free zero rotation,
prescription-containing magnitudes, floor/threshold boundaries and all row
families. Existing R7/A1/A2 mutants remain obligations even when the new gate
masks a defective older check. Ordinary controls must disclose honest new
refusals, and protected expected selections cannot be silently rewritten.

## Execution evidence and limits

Read-only Git used GIT_OPTIONAL_LOCKS=0. No Git/index mutation, Rust or solver
experiment, source edit, delegation, external engineering corpus, or host-tool
change occurred. No old response material was consulted, so its optional
handoff/read path was not activated. Skill scope validation for a mergeable
software diff was inapplicable to this read-only sealed-design review.

The existing VENV ran review_checks.py with -B, using only the standard library.
Two successful runs occurred: first from owned scratch (21 basis records), then
after adding the D1 base-design hash from the canonical packet (22 records).
Both had 46 passing exact checks; no failed check or changed numerical test
criterion. Canonical stdout/stderr and commands are retained. Initial broad
search output was truncated; relied-on sections were read in bounded excerpts.

Reproduce from the host's ROOT-provided aliases:

    <VENV>/bin/python -B <A1_WT>/<R>/design_review_RV28/review_checks.py \
      <APP_WORKTREE> <COORD> <A1_WT> <owned-output-directory>

The script verifies pinned source bytes and the proposal seal, then writes
BASIS.json, COMMANDS.json and EXACT_CHECKS.json only to the supplied output.
Use an owned scratch output for a rerun so this sealed review is preserved.

This review verifies the new certificate algebra and its mapping to accepted
R7 fields. It does not independently re-prove every floating-point formation
count or inverse-bound implementation in R7, run policy registration/readers,
execute a corrected solver, or establish ordinary/extreme-model availability.
Those limits do not invalidate the bounded mathematical result.

