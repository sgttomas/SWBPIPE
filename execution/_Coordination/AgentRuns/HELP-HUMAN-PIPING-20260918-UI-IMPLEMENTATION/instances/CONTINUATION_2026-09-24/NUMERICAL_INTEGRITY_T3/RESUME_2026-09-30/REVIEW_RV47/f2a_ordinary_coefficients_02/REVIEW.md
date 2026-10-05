# RV47 — ordinary coefficient instantiation backcheck

**Verdict: conditional mathematics and dependency direction confirmed; one
non-blocking evidence correction, RV47-C1.** Frozen candidate
`f5ca39a1a2d4c63b0fda3fe2e3c61bbe8afde074` is suitable for ROOT to integrate as
conditional design evidence with that correction open. The named exact-K control
must be repaired before it is credited as verified. No mathematical blocker,
product defect, new owning decision, implementation or availability closure is
established here.

Native TASK `/root/rv47_f2a_preview_truth`, continuation under ROOT `/root`.
Receipt 2026-10-02 22:15:17 UTC; new-check cutoff 22:35:17; return deadline
22:45:17. Source analysis completed by 22:20:02 UTC. Maintained source remains
`49034a940f3f8cd3f3da4d4cbc839943b808063d`.

Paths: P=`projects/chirality-piping`; C=`P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24`;
T3=`C/NUMERICAL_INTEGRITY_T3`; R=`T3/RESUME_2026-09-30`;
I36=`R/I36/f2a_ordinary_coefficients_02`;
I35=`R/I35/f2a_certificate_arithmetic_01` at `2034c050d6`;
FK=`P/core/solver/frame_kernel/src/structural/retained`.

## Actionable finding

**RV47-C1 — non-blocking evidence defect: the exact-K product control rounds CK.**
At I36 `_run_records/exact_controls.py:133–138`, each CK entry wraps its product
in `bits(...)`. That helper converts to binary64 and then back to Fraction.
Consequently the control named
`coefficient_difference_majorant_is_about_actual_K_products` checks a rounded
product, although RETURN §2 correctly defines CK as the **exact** product of
admitted binary64 primitives.

This matters whenever that product needs more than 53 bits. Independently, with
both operands `1+2^-52`, exact CK exceeds its binary64 rounding by `2^-104`.
A singleton source coefficient interval at the rounded product has true
`dC=2^-104`; the control's CK would give zero. This is an abstract discriminator,
not a product invocation or admitted-product failure.

**Repair:** lift each operand separately, then multiply the Fractions without the
outer `bits`: for example `eh * bits(geom['A'][0])`. Apply the same correction to
all four entries and add a rounding-sensitive control that distinguishes exact
CK from `RN64(CK)`. Rerun and reseal the narrow evidence patch. The written
coefficient theorem needs no change. The original 17 groups rerun and reproduce
the frozen JSON; that successful replay does not cure this wrong test boundary.

## Mathematical backcheck

**Interpolation.** The four terms are exactly
`Thi*Xlo - T*Xlo + T*Xhi - Tlo*Xhi`, equal to the positive-weight numerator
`(Thi-T)*Xlo + (T-Tlo)*Xhi`. With the validated strict bracket, h=Thi-Tlo>0,
so numerator sign exactly decides source positivity. No E endpoint sign gate is
introduced. The independently simulated integer RN64 example
`(Tlo,T,Thi,Elo,Ehi)=(0,7,25,-7,18)` has exact E=0 and rounded E_hat=2^-50,
confirming that the private nonpositive-source refusal is necessary. The existing
ordinary resolver's positive-G-endpoint check, rounded resolved-E/G checks and
actual expression at PP `lib.rs:9184–9276` agree with the packet's distinction.
Selection validity and the alpha requirements remain part of the source adapter.

**Finite local construction.** Every nonzero binary64 product has at most 106
significant bits, magnitude <2^2048 and integer quantum 2^-2148. Four original
terms need at most 4196 occupied exponent positions. Each separated-sign sum and
net is <2^2050, requiring at most 4198 positions; the rounded-nearest residual
term can extend the leading exponent to 2050, hence the conservative 4199 span.
It cannot introduce a nonzero bit below -2148: a value with fewer than 1024
significant bits rounds exactly; a wider value rounds on a coarser grid.
The adjacent step uses at most 1025 positions. These fit the existing 8128-bit
term limit and its 64 carry bits (FK `wide_sum.rs:22–30,218–295`). The exact
positive numerator is at least 2^-2148; h is at least 2^-1074 and less than
2^1025. Directed positive quotient endpoints remain far within the existing
Wide exponent range. Failures still refuse this certificate; this is not a proof
that subsequent bridge/recipe sums always fit.

The lower quotient uses P_down/h_up and the upper P_up/h_down. With positive
lower denominator/numerator these enclose exact X_s. Taking their hull with the
validated positive actual X_hat preserves positivity and includes both meanings.
Actual X_hat=X_K and base/point singleton identities are explicit obligations,
not inferred from a material label. No E/nu substitution occurs.

**I33 rectangle.** Outward products of the positive material/geometry intervals
bound each of EA, GJ, EI_z and EI_y. Differences from exact CK are bounded by the
maximum endpoint distance. CK need not belong to the source coefficient interval.
I33's Bbar/Hbar contraction, perturbation and recovery inequalities use positivity
and these bounds, not the exact-profile E/nu construction. They therefore hold
uniformly for the rectangle, including its conservative uncorrelated tuples.
The common exact frame, nodal ledger, supports, constraints, maps, verification
scaling, twice-B, zero/data-block and strict-alpha premises remain necessary.
Both recovery-functional and response changes are retained. No new operator,
source solve or uniqueness conclusion from rounded factor positivity is used.

**Readout composition.** Separate represented/source stress recipes, the
represented Z_hat versus exact I_K/(D/2) cover, and their final hull preserve the
reviewed direction. For fixed final y, endpoint distance to the hull equals the
largest constituent distance; positive unit conversion preserves this property.
The source direct-row interval contains the K interval. Source/basis/row custody
and actual built A/J identities still need implementing proof. Current circular
Iy=Iz is visible at PP `lib.rs:6554–6560`; a future distinct-axis mapping cannot
reuse the two-division common cover without separate association and cost.

## Work and scratch backcheck

The dependency deltas follow the displayed I35 schedule:

| Item | Checked result |
|---|---|
| Interpolation construction | 2 directed subtractions +8 exact-product multiplications +4 divisions +4 four-term rounding entries =18 entries, replacing I35's four material calls. |
| Three member builders | +14 per construction gives +42m; F3 represented Z adds 2m divisions. |
| Represented B1 stress recipe | Two Q_K endpoint operations plus the 32-call torsion upper gives +34q_sigma. Final hull/raw conversion/distance tests run once. |
| B1 scalar population | D+44m+34q_sigma =1226m+4b+49s+2(k+a)+47q+34q_sigma. B2 and observable reader work remain separately priced. |
| Four-term sum entry | At most seven raw additions, seven shifts, two net/round operations, two sign comparisons and one small-factor step: SumWork upper (1038,1792,768,256), total 3854<4096. |
| Logical scratch | Material schedule needs 21 endpoints; represented-Z and retained first-recipe pairs add four. Conservative extra constant reserve 25*137=3425 field-payload bytes. |

FK `directed.rs:28–85` and `wide_sum.rs:188–214,327–369,465–468,508–543`
support those local counts: the two rounds each net at 2*used, and the initial
positivity plus rounding-side tests each cost used. **Each lower/upper numerator
entry must begin with fresh counters**, with prior success/failure work consumed
before drop/reinitialization. Clearing a sum retains its counters. The packet
states fresh entries and exposes that boundary; a mere `clear` between entries
cannot inherit the per-entry bound.

I35's old five-add grammar in `ARITHMETIC_DESIGN.md` §2 and
`SCHEDULE_AND_COUNTS.md` §5 must explicitly gain this seven-add entry, its work
vector and caller correspondence. Its former per-operation microstep derivation
must be checked for the expanded grammar before reuse. I36 expressly calls for
that integration and does not adopt a tariff, byte allowance, Rust layout,
aggregate no-wrap guarantee or global arithmetic-fit result. Source-population
scans/sorts/maps, extra lifts, copy/zeroing work, failure prefixes and actual
lifetimes remain separate accounted populations, as required.

## Preserved boundaries and verification

ROOT `ROOT_RULINGS_V1.md:5227–5246` accepts the ordinary direction only
conditionally and keeps maxima's coefficient midpoint/location scope and
headlines as result_ref aliases. I36 preserves those limits, independent E/G,
the actual normalized effective-wall bit, existing selection validity, support
magnitude/public-component relation, unchanged predicates/classes/scales,
retired-row exclusions and no combination maximum. No fresh owning decision is
required merely for these proofs or the control repair.

Candidate scope is exactly eight added files; all seven inventory payloads and
19 recorded origin hashes verify. The author checker reproduces its 17-group
frozen output. Independent exact controls pass nine groups, including 450
numerator/quotient cases, sign/rounding discrimination, span, coefficient/hull
and count checks. The reviewer's first checker had a denominator-return typo;
a fixed midpoint assertion caught it, it was corrected, and the history is
retained. Those checks are abstract arithmetic only.

Only this assigned review packet was written. No maintained code, compiler,
solver, model/product runtime, host probes, Git/index/API writes, reference
regeneration, full source copies or delegation occurred. Implementing source
association, arithmetic/API/cost integration, layouts/capacities, aggregate
accounting, code review and protected actual availability remain open. Stop for
ROOT disposition and narrow evidence correction/backcheck.
