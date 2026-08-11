# DEC-046 convergence and distinct DEC-026 comparison-value owner packet

**Packet status:** `OWNER-READY / NON-AUTHORITATIVE / RULING REQUIRED`
**Prepared by:** `A2-DEC046-REMEDIATION`, fresh non-delegating ephemeral
Agent 2, attempt 2
**Repository basis:** `c05fe2d6fbc3bd3d3b690f50075e2c878af0faf3`
**Date:** 2026-08-10
**Scope:** replacement for the original DEC-046/census packet. It retains the
verified observations and exact historic 13-case census, and adds the missing
authority-resolution slate and four-suite comparison-value options. It makes
no ruling, repair, promotion, threshold, acceptance, register, Remaining,
receipt, lifecycle, reliance, release, or Git change.

## 1. Decision map: two axes, two independent owner gates

The shorthand “final public-benchmark release tolerances under the DEC-046
convention” spans two quantities whose accepted authorities expressly keep
separate:

| axis | governing authority | question | current status |
| --- | --- | --- | --- |
| nonlinear solver convergence | D-19 / `DEC-046` | when does the assembled active-set loop stop? | CV-B structure ruled; release-scope numeric promotion held |
| result-versus-reference verification | D-04 / `DEC-026` | how close must an observed benchmark value be to its reference? | class/per-kind relative+absolute structure ruled; public per-kind values incomplete |

`DEC-046` is not authority for verification comparisons. `DEC-026` is not
authority for nonlinear convergence. A lawful owner response therefore needs
either one ruling per axis or an explicit deferral of an axis.

### Gate C — DEC-046 convergence-only slate

- **C-A — observed-minimum:** zero changed-support threshold; caps
  one-way/gap/lift-off/friction=`2`, multi-support=`3`.
- **C-B — accepted validation values unchanged:** zero changed-support
  threshold; cap `4` for every class. **Recommended convergence choice.**
- **C-C — additional headroom:** zero changed-support threshold; cap `6` for
  every class; extra iterations are not required by inspected evidence.
- **C-D — defer/decline:** leave release scope `TBD`; keep the accepted
  validation-only records unchanged.

### Gate V — separately authorized DEC-026-derived comparison-value slate

Any V selection is a distinct owner act. It does not derive authority from
DEC-046 and does not alter convergence.

- **V-A — exact/measured boundary:** authorize a public comparison policy
  using the exact per-kind measured maxima and zero-reference floors in §5.1,
  plus the exact sparse maxima in §5.4. No measured headroom.
- **V-B — measured envelope with disclosed 10× headroom:** authorize the
  values in §5.2 and §5.4. The factor `10` is an owner-selected safety/headroom
  choice, not a measured necessity. Kinds observed exact remain exact because
  evidence supports no positive floor for them.
- **V-C — current-precedent projection:** authorize the `1.0e-9` relative seed
  and proposed unit-bearing `1.0e-9` absolute floors in §5.3, with the existing
  narrow DEC-050/053 sparse criteria in §5.4. The absolute-floor projection is
  an inference from the crate-internal absolute epsilon, not an already ruled
  public table; the current sparse precedent omits an absolute parity limit.
- **V-D — authorize measurement only, then return for values:** authorize a
  bounded current-25-fixture observable capture, quantity-kind floor
  measurement, and unit-normalized sparse comparison design; do not rule any
  public numeric value yet.
- **V-E — defer/decline:** do not authorize or select a public comparison-value
  policy in this sitting.

### Non-binding overall recommendation

Select **C-B** for the strictly fenced convergence axis and **V-D** (or V-E if
no further preparation is wanted) for the comparison axis. The concrete V-A,
V-B, and V-C tables are decision-ready, but all three inherit an epistemic
weakness: the committed mechanics capture observes only 11 of the historic 24
fixtures, two quantity kinds occur only behind the block, current source has
25 fixtures without a current whole-suite capture, and the sparse absolute
solution metric is not unit-normalized. C-B has direct 22-fixture support;
final public comparison values do not yet have equivalent coverage.

## 2. Comparison and convergence semantics

### 2.1 Proposed public result-comparison formula

For every continuous analytic quantity kind with finite observed `o`, finite
reference `r`, governed relative member `rtol_kind`, and unit-bearing absolute
floor `atol_kind`, the proposed inclusive predicate is:

```text
abs(o - r) <= max(atol_kind, rtol_kind * abs(r))
```

- Non-finite `o`, `r`, or threshold members fail closed.
- Equality at the boundary passes.
- `atol_kind` carries the same unit as `o` and `r`; `rtol_kind` is
  dimensionless.
- When `r == 0`, the relative term is zero and only the absolute floor can
  admit a nonzero observation.
- Discrete counts and categorical projections stay exact unless a separate
  governing act says otherwise.
- The formula is proposed application algebra for the DEC-026 pair/floor
  structure. It is not the currently implemented benchmark helper and is not
  applied by this packet.

Current mechanics and stress code instead uses finite operands and the
inclusive absolute predicate `abs(observed-recorded) <= 1.0e-9`.
`INTERNAL_ASSERTION_EPSILON = 1.0e-9` is explicitly crate-internal and is not
a public release policy. Nonlinear runner regression compares its recorded
numeric/categorical projection exactly. Sparse records use separate parity,
residual, repeatability, and pivot criteria rather than the formula above.

### 2.2 DEC-046 convergence operation

Current assembled-loop validation records consume the nonnegative integer
`active_set_changed_support_count`. The runtime computes
`effective_tolerance = max(relative_residual_field,
absolute_residual_floor)` and accepts inclusively when the count is at or
below it within the iteration cap. Both stored members currently carry unit
`count`; there is no `rtol * scale + atol` calculation on this axis. Any
effective threshold in `[0,1)` behaves as zero; a threshold `>=1` could accept
a still-changing support. C-A/C-B/C-C therefore retain both members at
`0 count` and differ only in cap.

## 3. Committed four-suite evidence

| suite | committed population | exact recomputation |
| --- | --- | --- |
| mechanics | historic 24 requested: 11 matched, 0 mismatched, 13 blocked; 91 compared and 109 unobserved recorded values | 78/91 values exact; 13 nonzero deltas in exactly four cases; max absolute `3.552713678800501e-15`; max nonzero-reference relative `1.7763568394002505e-15` |
| stress | 3 matched; 11 compared values | 11/11 exact; max delta `0` |
| nonlinear runner regression | 5 matched; five residual-count comparisons plus exact states, changed-support IDs, converged flags, and diagnostics | all numeric and categorical projections exact; 2 recorded converged and 3 recorded nonconverged |
| nonlinear assembled convergence | 9 accepted seed observations plus 13 accepted multi-support observations | first 21 stabilize by iteration 2; cascade stabilizes by iteration 3; final changed-support count `0` |
| sparse | 9 DEC-053 observations | maxima: relative parity `7.060341894958857e-11`, absolute solution delta `5.342535303043405e-10`, absolute residual `1.0058283805847168e-7`; repeat deltas `0`; nonpositive pivots `0` |

Strict exact equality is an informative boundary, not one of the proposed
analytic policies: it admits 78/91 mechanics values and fully admits 7/11
observable mechanics cases; it excludes the 13 nonzero values in
`MECH-TP-PHYS-004`, `005`, `006`, and `007`. It admits all 11 stress values
and the entire exact nonlinear regression projection.

### Mechanics measured maxima by observed kind

| kind / unit | values | max absolute delta | max absolute delta at zero reference | max relative delta at nonzero reference |
| --- | ---: | ---: | ---: | ---: |
| discrete count / `count` | 5 | `0` | `0 count` | `0` |
| dimensionless ratio / `dimensionless` | 4 | `0` | no zero reference observed | `0` |
| displacement / `m` | 6 | `1.734723475976807e-18 m` | `0 m` | `1.2390881971262908e-16` |
| force / `N` | 42 | `1.7763568394002505e-15 N` | `0 N` | `4.440892098500626e-16` |
| length / `m` | 7 | `0 m` | no zero reference observed | `0` |
| linear stiffness / `N/m` | 2 | `0 N/m` | no zero reference observed | `0` |
| moment / `N-m` | 20 | `3.552713678800501e-15 N-m` | `1.7763568394002505e-15 N-m` | `1.7763568394002505e-15` |
| rotation / `rad` | 5 | `8.673617379884035e-19 rad` | no zero reference observed | `2.0016040107424698e-16` |
| stress / `Pa` | 11 | `0 Pa` | `0 Pa` | `0` |

`force_per_length` (`N/m`, 6 blocked values) and `mass_per_length` (`kg/m`,
1 blocked value) have no runner-observed member in this capture. Their measured
relative values and absolute floors are therefore `TBD`. The other 102
blocked recorded values also provide no new observed delta.

## 4. DEC-046 convergence values and admissions

| class | C-A relative / absolute / cap | C-B relative / absolute / cap | C-C relative / absolute / cap |
| --- | --- | --- | --- |
| one-way | `0 count / 0 count / 2` | `0 / 0 / 4` | `0 / 0 / 6` |
| gap | `0 count / 0 count / 2` | `0 / 0 / 4` | `0 / 0 / 6` |
| lift-off | `0 count / 0 count / 2` | `0 / 0 / 4` | `0 / 0 / 6` |
| friction | `0 count / 0 count / 2` | `0 / 0 / 4` | `0 / 0 / 6` |
| multi-support / multi-DOF | `0 count / 0 count / 3` | `0 / 0 / 4` | `0 / 0 / 6` |

| convergence population | C-A | C-B | C-C |
| --- | --- | --- | --- |
| 9 accepted seed observations | admits `9/9` | admits `9/9` | admits `9/9` |
| 13 accepted multi-support observations | admits `13/13` | admits `13/13` | admits `13/13` |
| completes at iteration 3 | only multi-support class admits | admits | admits |
| completes at iteration 4 | excludes | admits | admits |
| completes at iteration 5 or 6 | excludes | excludes | admits |
| final changed-support count 1 | excludes | excludes | excludes |
| mechanics / stress / exact regression / sparse comparison evidence | no effect | no effect | no effect |

C-A is the exact observed cap envelope. C-B preserves the accepted
validation-scope values and supplies one iteration of headroom over the
observed multi-support maximum and two over the other maxima. C-C adds
unevidenced cap headroom. Raising a cap later is a new governance event.

## 5. Distinct DEC-026-derived public comparison-value options

These are owner options, not findings that the numbers are already governed.
The analytic tables use the formula in §2.1. A `TBD` remains blocking and must
keep the applicable tolerance-policy diagnostic active.

### 5.1 V-A — exact/measured boundary

For every observed analytic kind, `rtol` is the maximum nonzero-reference
relative delta and `atol` is the maximum zero-reference absolute delta. This
is the tightest recomputable pair that admits the captured data under the
proposed inclusive formula.

| analytic kind | `rtol` | unit-bearing `atol` | basis |
| --- | ---: | ---: | --- |
| discrete count | `0` | `0 count` | exact discrete projection |
| dimensionless ratio | `0` | `0 dimensionless` | four exact observed values; zero-floor is inferred as exact because no zero reference was observed |
| displacement | `1.2390881971262908e-16` | `0 m` | six observed values |
| force | `4.440892098500626e-16` | `0 N` | 42 observed values |
| length | `0` | `0 m` | seven exact observed values; zero-floor is inferred as exact |
| linear stiffness | `0` | `0 N/m` | two exact observed values; zero-floor is inferred as exact |
| moment | `1.7763568394002505e-15` | `1.7763568394002505e-15 N-m` | 20 observed values, including two zero-reference deltas |
| rotation | `2.0016040107424698e-16` | `0 rad` | five observed values; zero-floor is inferred as exact |
| stress | `0` | `0 Pa` | 11 exact observed values, including a zero reference |
| force per length | `TBD` | `TBD N/m` | no observed runner comparison |
| mass per length | `TBD` | `TBD kg/m` | no observed runner comparison |

The zero floors labeled inferred are deliberately visible: exact observed
nonzero references do not measure near-zero behavior. V-A is a boundary fit,
not a claim of robustness.

### 5.2 V-B — measured boundary with disclosed 10× headroom

The factor `10` multiplies every nonzero V-A member. It is a proposed safety
margin and is not derived from a statistical population, uncertainty model,
professional standard, or additional run. Zero members remain zero rather
than silently inventing positive floors.

| analytic kind | `rtol` | unit-bearing `atol` | caveat |
| --- | ---: | ---: | --- |
| discrete count | `0` | `0 count` | exact |
| dimensionless ratio | `0` | `0 dimensionless` | exact; no measured headroom |
| displacement | `1.2390881971262908e-15` | `0 m` | 10× relative headroom only |
| force | `4.440892098500626e-15` | `0 N` | 10× relative headroom only |
| length | `0` | `0 m` | exact; no measured headroom |
| linear stiffness | `0` | `0 N/m` | exact; no measured headroom |
| moment | `1.7763568394002505e-14` | `1.7763568394002505e-14 N-m` | 10× both measured members |
| rotation | `2.0016040107424698e-15` | `0 rad` | 10× relative headroom only |
| stress | `0` | `0 Pa` | exact; no measured headroom |
| force per length | `TBD` | `TBD N/m` | no observed runner comparison |
| mass per length | `TBD` | `TBD kg/m` | no observed runner comparison |

### 5.3 V-C — current-precedent projection

`DEC-026` rules an analytic relative seed of `1.0e-9` where suites pass.
Mechanics and stress currently use an absolute `1.0e-9` internal assertion
epsilon. V-C projects those two precedents into a proposed public pair by
using `rtol=1.0e-9` and a same-number unit-bearing `atol` for continuous
kinds. That unit-by-unit projection is an explicit governance inference; the
current internal epsilon disclaims public-policy status and was not derived as
a near-zero floor.

| analytic kind | `rtol` | unit-bearing `atol` | evidence status |
| --- | ---: | ---: | --- |
| discrete count | `0` | `0 count` | exact semantics retained |
| dimensionless ratio | `1.0e-9` | `1.0e-9 dimensionless` | proposed projection |
| displacement | `1.0e-9` | `1.0e-9 m` | proposed projection |
| force | `1.0e-9` | `1.0e-9 N` | proposed projection |
| length | `1.0e-9` | `1.0e-9 m` | proposed projection |
| linear stiffness | `1.0e-9` | `1.0e-9 N/m` | proposed projection |
| moment | `1.0e-9` | `1.0e-9 N-m` | proposed projection |
| rotation | `1.0e-9` | `1.0e-9 rad` | proposed projection |
| stress | `1.0e-9` | `1.0e-9 Pa` | proposed projection |
| force per length | `1.0e-9` | `1.0e-9 N/m` | proposed from crate-wide precedent; not runner-measured |
| mass per length | `1.0e-9` | `1.0e-9 kg/m` | proposed from crate-wide precedent; not runner-measured |

### 5.4 Sparse subcriteria under V-A/V-B/V-C

Sparse evidence cannot be reduced honestly to one scalar. A proposed public
gate must evaluate all named subcriteria conjunctively. The absolute solution
delta uses heterogeneous “solution DOF units”; it is not a unit-normalized
per-kind floor. That makes V-A/V-B concrete but weak candidates for a final
public policy and is a principal reason to recommend V-D.

| sparse subcriterion | V-A exact measured boundary | V-B 10× headroom | V-C current narrow precedent |
| --- | ---: | ---: | ---: |
| sparse/dense solution relative delta | `7.060341894958857e-11` | `7.060341894958857e-10` | `1.0e-9` |
| max absolute sparse/dense solution delta | `5.342535303043405e-10 solution-DOF units` | `5.342535303043405e-9 solution-DOF units` | `TBD / not gated` |
| max absolute sparse residual | `1.0058283805847168e-7 generalized-force units` | `1.0058283805847168e-6 generalized-force units` | `1.0e-6 generalized-force units` |
| max absolute repeat-solution delta | `0 solution-DOF units` | `0 solution-DOF units` | `0 solution-DOF units` |
| nonpositive pivot count | `0 count` | `0 count` | `0 count` |

V-C exactly preserves the limits in the accepted **bounded, non-release**
DEC-050/053 observation policy. It does not acquire release authority without
the owner act offered here. Its absent absolute sparse/dense parity member is
not hidden; treating relative parity alone as sufficient would itself be part
of a V-C ruling.

### 5.5 Exact current admissions/exclusions

| committed population | V-A | V-B | V-C | exclusions / unreachable comparisons |
| --- | --- | --- | --- | --- |
| mechanics observable | admits `91/91` values and `11/11` cases | admits `91/91`, `11/11` | admits `91/91`, `11/11` | none among observable rows |
| mechanics historic blocked | compares `0/109` values; unblocks `0/13` cases | same | same | all 13 remain blocked before the predicate |
| stress | admits `11/11` values and `3/3` cases | admits `11/11`, `3/3` | admits `11/11`, `3/3` | none |
| nonlinear regression numeric | admits `5/5` exact residual values | same | same | no numeric exclusion |
| nonlinear regression categorical | exact match required; admits `5/5` cases | same | same | any changed state/set/flag/diagnostic is excluded; no current exclusion |
| nonlinear assembled convergence | no comparison-policy effect | no effect | no effect | governed only by Gate C |
| sparse | admits `9/9` rows on all five V-A subcriteria | admits `9/9` on all five V-B subcriteria | admits `9/9` on the four currently governed V-C subcriteria | V-C does not test absolute parity; no current row fails the other criteria |

The current evidence does not distinguish the three options by current case
pass count; it distinguishes their future admission envelope. V-A fails any
future observed value above the exact maxima. V-B permits at most the stated
10× envelopes. V-C is generally much looser for analytic relative/error
members; on sparse residual it is slightly tighter than V-B
(`1.0e-6 < 1.0058283805847168e-6`), while its relative parity limit is looser
and it omits absolute parity.

## 6. Exact historic census: why the 13 mechanics cases are blocked

`core/runner/headless/src/benchmark_binding.rs` maps 11 mechanics fixture IDs
to public observed-value functions. Every other ID reaches
`CaseEvaluation::NotReusable`. The runner then emits every recorded expected
value with `observed=null`, `delta=null`, and
`within_recorded_basis=null`. Therefore no tolerance predicate is evaluated.
All 13 primary classifications are **implementation**; data=`0`,
tolerance=`0`, fixture=`0`, implementation=`13`. “Fixture/API” below is a
secondary description, not the primary class.

| # | fixture | recorded values / kinds | primary | secondary public-observation gap | effect of every C/V choice |
| ---: | --- | --- | --- | --- | --- |
| 1 | `MECH-CANTILEVER-TIP-FORCE` | 2: length, moment | implementation | public solver omits complete named fixed-end-moment projection | remains blocked |
| 2 | `MECH-STRAIGHT-PIPE-WEIGHT-RECOVERY` | 2: force/length, force | implementation | boolean validator, no value-addressable runner output | remains blocked |
| 3 | `MECH-SUPPORT-BOUNDARY-MIXED` | 3: count, stiffness, rotation | implementation | boolean preparation validator; heterogeneous outputs not projected | remains blocked |
| 4 | `MECH-PRIMITIVE-LOAD-PREP` | 3: force, force/length, length | implementation | boolean preparation validator; named outputs not projected | remains blocked |
| 5 | `MECH-FIXED-FIXED-THERMAL-AXIAL` | 2: ratio, force | implementation | crate-internal formula assertion only | remains blocked |
| 6 | `MECH-IMPOSED-DISPLACEMENT-SPRING` | 1: force | implementation | boolean support-preparation validator; no named value | remains blocked |
| 7 | `MECH-INCLINED-MEMBER-TRANSFORM` | 2: ratios | implementation | boolean transform/symmetry validator; no named projection | remains blocked |
| 8 | `MECH-EXPANSION-LOOP-CURVED-BEND-THERMAL` | 21: displacement, force, moment | implementation | solver/validator exists; no 21-value runner mapping | remains blocked |
| 9 | `MECH-CURVED-BEND-DISTRIBUTED-FIXED-END` | 42: force, moment | implementation | solver/normalized-deviation accessor exists; no 42-value mapping | remains blocked |
| 10 | `MECH-CURVED-BEND-PRESSURE-THRUST-ARC` | 12: displacement, force | implementation | solver/accessor exists; no value mapping | remains blocked |
| 11 | `MECH-TP-PMM-P3-OCCLOADGEN-EQUIVALENT-STATIC` | 6: mass/length, force/length, force | implementation | fixture calculations exist; no runner mapping | remains blocked |
| 12 | `MECH-TP-PMM-P3-SUBSPAN-WIND-EXPOSURE` | 8: force/length, force | implementation | fixture calculations exist; no runner mapping | remains blocked |
| 13 | `MECH-CONSTANT-EFFORT-SUPPORT-APPLIED-LOAD` | 5: force, length, moment | implementation | public solve exists; no complete named mapping | remains blocked |

The recorded expected data exist; a numeric threshold cannot create an
observed counterpart. Repair requires a separately authorized implementation
tranche and is out of scope here.

## 7. Currency and application boundaries

The 13-row census is exact for the immutable July 20 bundle at source commit
`e315fb8406d44dce684cbec091f3174c261efee4`: 24 fixtures, 11 matched, 13
blocked. Current mechanics source at this packet's base has 25 fixtures after
DEC-092, while the runner binding remains unchanged and has no DEC-092 arm.
A current run would likely record a fourteenth block, but that is an inference;
no committed current-base whole-suite capture proves it. No ruling should call
13 the complete current inventory without constraining the claim to the July
20 capture.

After an owner ruling, a separate bounded managed application may only:

1. preserve historical validation and sparse evidence byte-identically;
2. create a new governed release-scope record for the selected axis/value set,
   with a new identity and exact source pointers;
3. bind only the ruled classes, quantity kinds, formula, values, and `TBD`s;
4. run fresh evidence appropriate to that policy before claiming it is met;
5. leave the 13-case runner-observation repair to a separate authorized
   implementation lane;
6. apply TM-PIP-037 disposition and the first two DEL-09-04 Remaining bullets
   only to the extent of the owner's exact rulings; and
7. return to the owner before any lifecycle, reliance, professional,
   publication, or release act.

No C ruling authorizes V values. No V ruling authorizes C values. V-A/V-B
leave blocked-only force-per-length and mass-per-length entries `TBD`; V-C
fills them only through the expressly disclosed current-precedent inference.

## 8. Copyable ruling forms

### Recommended two-axis disposition

> **DEC-046 RELEASE-CONVERGENCE VALUES — C-B.** Authorize a new,
> release-scope governed convergence record with
> `{relative_residual_field=0 count, absolute_residual_floor=0 count,
> max_iterations=4}` for one-way, gap, lift-off, friction, and
> multi-support/multi-DOF. The inclusive criterion requires final changed-
> support count zero. This does not select verification-comparison values,
> unblock mechanics cases, or create an acceptance, lifecycle, reliance, or
> release act.
>
> **DEC-026-DERIVED PUBLIC COMPARISON VALUES — V-D.** Authorize only a bounded
> current-25-fixture observable capture, per-kind near-zero floor measurement,
> and unit-normalized sparse comparison design, returning to me before any
> numeric value is ruled. Do not promote V-A, V-B, or V-C now.

The owner may replace `C-B` with `C-A`, `C-C`, or `C-D`, and may replace V-D
with `V-A`, `V-B`, `V-C`, or `V-E`, by exact identifier. Selecting V-A/V-B/V-C
also selects the formula and all explicit `TBD`/inference caveats in §5.

## 9. Source identities

| source | Git blob | role |
| --- | --- | --- |
| `D-19_release_convergence_tolerance_policy.md` | `9275cd9c44f1af84b443910e08c8ba73aa0e8791` | DEC-046 axis, CV-B structure, boundary |
| `D-04_tolerance_coverage_thresholds.md` | `c6b12371a8473724d0d0e5d8e6be2d26e4e9f8e3` | DEC-026 comparison classes and pair/floor structure |
| `convergence_policy.dec046.json` | `d3850f5becda48c523dfb15cd5d87aabe54220c0` | four-class validation values |
| `multisupport_convergence_policy.dec046.json` | `a6f43a50aa7682d85409f1d25b8982aeff7bd6a9` | multi-support values/evidence set |
| nonlinear benchmark source | `37d19abf27a17c1d1333a81732d0b413e4ca2880` | iteration counts and exact regression behavior |
| nonlinear integration source | `36062a62a95da7efe2bf801a7dad8f3c398f6c9d` | live convergence consumer |
| mechanics benchmark source | `eb65e53075110995a4ddcd93b4181b15392f91d5` | internal epsilon, 25-fixture inventory |
| stress benchmark source | `201f5d84d1a666975000d07fb8e21900b88f9807` | internal epsilon and finite predicate |
| headless benchmark binding | `75fa69df616dd803ebd8409683d2468536c4b6ac` | 11 mappings and fail-closed catch-all |
| mechanics `SUITE_RUN_MECHANICS.json` | `c0fee1b6581169e4427686e5c1932ec6176f5ceb` | 91 observations and exact 13-case census |
| R14 stress output | `2feb1b9cf0abc0f978457a6e1564db4596239a22` | 3 cases / 11 values |
| R14 nonlinear output | `ce8dc556b66dc4384b87acc73829129f1b70b719` | five exact regression cases |
| sparse DEC-053 policy | `c1dce40afc643d177700d7cea49df76d1c8d5262` | current bounded non-release precedent |
| sparse DEC-053 observation | `ad57a55ca33e9b3f7f1a57eb196db52adc49484c` | nine sparse observations |

## 10. Explicit fences

This packet is preparation and report-only discovery. It performs no data,
fixture, suite, runner, solver, policy, case-page, manual, evidence, register,
status, receipt, decision, lifecycle, reliance, acceptance, promotion,
threshold, release, or Git effect. GUI-workflow validation evidence, the
export/CAEPIPE TBD cluster, D-61 reliance posture, lifecycle transitions, and
release acts remain out of scope.
